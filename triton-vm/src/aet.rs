use std::collections::hash_map::Entry::Occupied;
use std::collections::hash_map::Entry::Vacant;
use std::collections::HashMap;
use std::ops::AddAssign;

use itertools::Itertools;
use ndarray::s;
use ndarray::Array2;
use ndarray::Axis;
use num_traits::One;
use num_traits::Zero;
use twenty_first::shared_math::b_field_element::BFieldElement;
use twenty_first::shared_math::digest::Digest;
use twenty_first::shared_math::digest::DIGEST_LENGTH;
use twenty_first::shared_math::tip5;
use twenty_first::shared_math::tip5::Tip5;
use twenty_first::util_types::algebraic_hasher::SpongeHasher;

use crate::error::InstructionError;
use crate::error::InstructionError::InstructionPointerOverflow;
use crate::instruction::Instruction;
use crate::program::Program;
use crate::stark::StarkHasher;
use crate::table::hash_table::HashTable;
use crate::table::hash_table::PermutationTrace;
use crate::table::op_stack_table::OpStackTableEntry;
use crate::table::ram_table::RamTableCall;
use crate::table::table_column::HashBaseTableColumn::CI;
use crate::table::table_column::MasterBaseTableColumn;
use crate::table::u32_table::U32TableEntry;
use crate::table::*;
use crate::vm::CoProcessorCall;
use crate::vm::CoProcessorCall::*;
use crate::vm::VMState;

/// An Algebraic Execution Trace (AET) is the primary witness required for proof generation. It
/// holds every intermediate state of the processor and all co-processors, alongside additional
/// witness information, such as the number of times each instruction has been looked up
/// (equivalently, how often each instruction has been executed).
#[derive(Debug, Clone)]
pub struct AlgebraicExecutionTrace {
    /// The program that was executed in order to generate the trace.
    pub program: Program,

    /// The number of times each instruction has been executed.
    ///
    /// Each instruction in the `program` has one associated entry in `instruction_multiplicities`,
    /// counting the number of times this specific instruction at that location in the program
    /// memory has been executed.
    pub instruction_multiplicities: Vec<u32>,

    /// Records the state of the processor after each instruction.
    pub processor_trace: Array2<BFieldElement>,

    pub op_stack_underflow_trace: Array2<BFieldElement>,

    pub ram_trace: Array2<BFieldElement>,

    /// The trace of hashing the program whose execution generated this `AlgebraicExecutionTrace`.
    /// The resulting digest
    /// 1. ties a [`Proof`](crate::proof::Proof) to the program it was produced from, and
    /// 1. is accessible to the program being executed.
    pub program_hash_trace: Array2<BFieldElement>,

    /// For the `hash` instruction, the hash trace records the internal state of the Tip5
    /// permutation for each round.
    pub hash_trace: Array2<BFieldElement>,

    /// For the Sponge instructions, i.e., `sponge_init`, `sponge_absorb`, and `sponge_squeeze`,
    /// the Sponge trace records the internal state of the Tip5 permutation for each round.
    pub sponge_trace: Array2<BFieldElement>,

    /// The u32 entries hold all pairs of BFieldElements that were written to the U32 Table,
    /// alongside the u32 instruction that was executed at the time. Additionally, it records how
    /// often the instruction was executed with these arguments.
    pub u32_entries: HashMap<U32TableEntry, u64>,

    /// Records how often each entry in the cascade table was looked up.
    pub cascade_table_lookup_multiplicities: HashMap<u16, u64>,

    /// Records how often each entry in the lookup table was looked up.
    pub lookup_table_lookup_multiplicities: [u64; 1 << 8],
}

impl AlgebraicExecutionTrace {
    pub fn new(program: Program) -> Self {
        let program_len = program.len_bwords();

        let mut aet = Self {
            program,
            instruction_multiplicities: vec![0_u32; program_len],
            processor_trace: Array2::default([0, processor_table::BASE_WIDTH]),
            op_stack_underflow_trace: Array2::default([0, op_stack_table::BASE_WIDTH]),
            ram_trace: Array2::default([0, ram_table::BASE_WIDTH]),
            program_hash_trace: Array2::default([0, hash_table::BASE_WIDTH]),
            hash_trace: Array2::default([0, hash_table::BASE_WIDTH]),
            sponge_trace: Array2::default([0, hash_table::BASE_WIDTH]),
            u32_entries: HashMap::new(),
            cascade_table_lookup_multiplicities: HashMap::new(),
            lookup_table_lookup_multiplicities: [0; 1 << 8],
        };
        aet.fill_program_hash_trace();
        aet
    }

    pub fn padded_height(&self) -> usize {
        let relevant_table_heights = [
            self.program_table_length(),
            self.processor_table_length(),
            self.op_stack_table_length(),
            self.ram_table_length(),
            self.hash_table_length(),
            self.cascade_table_length(),
            self.lookup_table_length(),
            self.u32_table_length(),
        ];
        let max_height = relevant_table_heights.into_iter().max().unwrap();
        max_height.next_power_of_two()
    }

    /// Hash the program and record the entire Sponge's trace for program attestation.
    fn fill_program_hash_trace(&mut self) {
        let padded_program = Self::hash_input_pad_program(&self.program);
        let mut program_sponge = StarkHasher::init();
        for chunk in padded_program.chunks(StarkHasher::RATE) {
            program_sponge.state[..StarkHasher::RATE]
                .iter_mut()
                .zip_eq(chunk)
                .for_each(|(sponge_state_elem, &absorb_elem)| *sponge_state_elem = absorb_elem);
            let hash_trace = StarkHasher::trace(&mut program_sponge);
            let trace_addendum = HashTable::trace_to_table_rows(hash_trace);

            self.increase_lookup_multiplicities(hash_trace);
            self.program_hash_trace
                .append(Axis(0), trace_addendum.view())
                .expect("shapes must be identical");
        }

        let instruction_column_index = CI.base_table_index();
        let mut instruction_column = self.program_hash_trace.column_mut(instruction_column_index);
        instruction_column.fill(Instruction::Hash.opcode_b());

        // consistency check
        let program_digest = program_sponge.state[..DIGEST_LENGTH].try_into().unwrap();
        let program_digest = Digest::new(program_digest);
        let expected_digest = self.program.hash::<StarkHasher>();
        assert_eq!(expected_digest, program_digest);
    }

    fn hash_input_pad_program(program: &Program) -> Vec<BFieldElement> {
        let padded_program_length = Self::padded_program_length(program);

        // padding is one 1, then as many zeros as necessary: [1, 0, 0, …]
        let program_iter = program.to_bwords().into_iter();
        let one_iter = [BFieldElement::one()].into_iter();
        let zeros_iter = [BFieldElement::zero()].into_iter().cycle();
        program_iter
            .chain(one_iter)
            .chain(zeros_iter)
            .take(padded_program_length)
            .collect()
    }

    pub fn program_table_length(&self) -> usize {
        Self::padded_program_length(&self.program)
    }

    fn padded_program_length(program: &Program) -> usize {
        // After adding one 1, the program table is padded to the next smallest multiple of the
        // sponge's rate with 0s.
        // Also note that the Program Table's side of the instruction lookup argument requires at
        // least one padding row to account for the processor's “next instruction or argument.”
        // Both of these are captured by the “+ 1” in the following line.
        let min_padded_len = program.len_bwords() + 1;
        let remainder_len = min_padded_len % StarkHasher::RATE;
        let num_zeros_to_add = match remainder_len {
            0 => 0,
            _ => StarkHasher::RATE - remainder_len,
        };
        min_padded_len + num_zeros_to_add
    }

    pub fn processor_table_length(&self) -> usize {
        self.processor_trace.nrows()
    }

    pub fn op_stack_table_length(&self) -> usize {
        self.op_stack_underflow_trace.nrows()
    }

    pub fn ram_table_length(&self) -> usize {
        self.ram_trace.nrows()
    }

    pub fn hash_table_length(&self) -> usize {
        self.sponge_trace.nrows() + self.hash_trace.nrows() + self.program_hash_trace.nrows()
    }

    pub fn cascade_table_length(&self) -> usize {
        self.cascade_table_lookup_multiplicities.len()
    }

    pub fn lookup_table_length(&self) -> usize {
        1 << 8
    }

    pub fn u32_table_length(&self) -> usize {
        self.u32_entries
            .keys()
            .map(|entry| entry.table_length_contribution())
            .sum::<u32>()
            .try_into()
            .unwrap()
    }

    pub fn record_state(&mut self, state: &VMState) -> Result<(), InstructionError> {
        self.record_instruction_lookup(state.instruction_pointer)?;
        self.append_state_to_processor_trace(state);
        Ok(())
    }

    fn record_instruction_lookup(
        &mut self,
        instruction_pointer: usize,
    ) -> Result<(), InstructionError> {
        if instruction_pointer >= self.instruction_multiplicities.len() {
            return Err(InstructionPointerOverflow);
        }
        self.instruction_multiplicities[instruction_pointer] += 1;
        Ok(())
    }

    fn append_state_to_processor_trace(&mut self, state: &VMState) {
        self.processor_trace
            .push_row(state.to_processor_row().view())
            .unwrap()
    }

    pub fn record_co_processor_call(&mut self, co_processor_call: CoProcessorCall) {
        match co_processor_call {
            Tip5Trace(Instruction::Hash, trace) => self.append_hash_trace(*trace),
            SpongeStateReset => self.append_initial_sponge_state(),
            Tip5Trace(instruction, trace) => self.append_sponge_trace(instruction, *trace),
            U32Call(u32_entry) => self.record_u32_table_entry(u32_entry),
            OpStackCall(op_stack_entry) => self.record_op_stack_entry(op_stack_entry),
            RamCall(ram_call) => self.record_ram_call(ram_call),
        }
    }

    pub fn append_hash_trace(&mut self, trace: PermutationTrace) {
        self.increase_lookup_multiplicities(trace);
        let mut hash_trace_addendum = HashTable::trace_to_table_rows(trace);
        hash_trace_addendum
            .slice_mut(s![.., CI.base_table_index()])
            .fill(Instruction::Hash.opcode_b());
        self.hash_trace
            .append(Axis(0), hash_trace_addendum.view())
            .expect("shapes must be identical");
    }

    fn append_initial_sponge_state(&mut self) {
        let round_number = 0;
        let initial_state = Tip5::init().state;
        let mut hash_table_row = HashTable::trace_row_to_table_row(initial_state, round_number);
        hash_table_row[CI.base_table_index()] = Instruction::SpongeInit.opcode_b();
        self.sponge_trace.push_row(hash_table_row.view()).unwrap();
    }

    fn append_sponge_trace(&mut self, instruction: Instruction, trace: PermutationTrace) {
        assert!(matches!(
            instruction,
            Instruction::SpongeAbsorb | Instruction::SpongeSqueeze
        ));
        self.increase_lookup_multiplicities(trace);
        let mut sponge_trace_addendum = HashTable::trace_to_table_rows(trace);
        sponge_trace_addendum
            .slice_mut(s![.., CI.base_table_index()])
            .fill(instruction.opcode_b());
        self.sponge_trace
            .append(Axis(0), sponge_trace_addendum.view())
            .expect("shapes must be identical");
    }

    /// Given a trace of the hash function's permutation, determines how often each entry in the
    /// - cascade table was looked up, and
    /// - lookup table was looked up
    /// and increases the multiplicities accordingly
    fn increase_lookup_multiplicities(&mut self, trace: PermutationTrace) {
        // The last row in the trace is the permutation's result: no lookups are performed for it.
        let rows_for_which_lookups_are_performed = trace.iter().dropping_back(1);
        for row in rows_for_which_lookups_are_performed {
            self.increase_lookup_multiplicities_for_row(row);
        }
    }

    /// Given one row of the hash function's permutation trace, increase the multiplicities of the
    /// relevant entries in the cascade table and/or the lookup table.
    fn increase_lookup_multiplicities_for_row(&mut self, row: &[BFieldElement; tip5::STATE_SIZE]) {
        for &state_element in row[0..tip5::NUM_SPLIT_AND_LOOKUP].iter() {
            self.increase_lookup_multiplicities_for_state_element(state_element);
        }
    }

    /// Given one state element, increase the multiplicities of the corresponding entries in the
    /// cascade table and/or the lookup table.
    fn increase_lookup_multiplicities_for_state_element(&mut self, state_element: BFieldElement) {
        for limb in HashTable::base_field_element_into_16_bit_limbs(state_element) {
            match self.cascade_table_lookup_multiplicities.entry(limb) {
                Occupied(mut cascade_table_entry) => *cascade_table_entry.get_mut() += 1,
                Vacant(cascade_table_entry) => {
                    cascade_table_entry.insert(1);
                    self.increase_lookup_table_multiplicities_for_limb(limb);
                }
            }
        }
    }

    /// Given one 16-bit limb, increase the multiplicities of the corresponding entries in the
    /// lookup table.
    fn increase_lookup_table_multiplicities_for_limb(&mut self, limb: u16) {
        let limb_lo = limb & 0xff;
        let limb_hi = (limb >> 8) & 0xff;
        self.lookup_table_lookup_multiplicities[limb_lo as usize] += 1;
        self.lookup_table_lookup_multiplicities[limb_hi as usize] += 1;
    }

    fn record_u32_table_entry(&mut self, u32_entry: U32TableEntry) {
        self.u32_entries.entry(u32_entry).or_insert(0).add_assign(1)
    }

    fn record_op_stack_entry(&mut self, op_stack_entry: OpStackTableEntry) {
        let op_stack_table_row = op_stack_entry.to_base_table_row();
        self.op_stack_underflow_trace
            .push_row(op_stack_table_row.view())
            .unwrap();
    }

    fn record_ram_call(&mut self, ram_call: RamTableCall) {
        self.ram_trace
            .push_row(ram_call.to_table_row().view())
            .unwrap();
    }
}

#[cfg(test)]
mod tests {
    use assert2::assert;
    use twenty_first::shared_math::b_field_element::BFIELD_ONE;

    use crate::triton_asm;
    use crate::triton_program;

    use super::*;

    #[test]
    fn pad_program_requiring_no_padding_zeros() {
        let eight_nops = triton_asm![nop; 8];
        let program = triton_program!({&eight_nops} halt);
        let padded_program = AlgebraicExecutionTrace::hash_input_pad_program(&program);

        let expected = [program.to_bwords(), vec![BFIELD_ONE]].concat();
        assert!(expected == padded_program);
    }
}
