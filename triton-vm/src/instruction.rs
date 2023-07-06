use std::collections::HashMap;
use std::fmt::Display;
use std::result;
use std::vec;

use anyhow::anyhow;
use anyhow::Result;
use get_size::GetSize;
use lazy_static::lazy_static;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use strum::EnumCount;
use strum::IntoEnumIterator;
use strum_macros::EnumCount;
use strum_macros::EnumIter;
use twenty_first::shared_math::b_field_element::BFieldElement;
use twenty_first::shared_math::b_field_element::BFIELD_ZERO;

use AnInstruction::*;

use crate::instruction::InstructionBit::*;
use crate::op_stack::OpStackElement;
use crate::op_stack::OpStackElement::*;

/// An `Instruction` has `call` addresses encoded as absolute integers.
pub type Instruction = AnInstruction<BFieldElement>;
pub const ALL_INSTRUCTIONS: [Instruction; Instruction::COUNT] = all_instructions_without_args();
pub const ALL_INSTRUCTION_NAMES: [&str; Instruction::COUNT] = all_instruction_names();

lazy_static! {
    pub static ref OPCODE_TO_INSTRUCTION_MAP: HashMap<u32, Instruction> = {
        let mut opcode_to_instruction_map = HashMap::new();
        for instruction in Instruction::iter() {
            opcode_to_instruction_map.insert(instruction.opcode(), instruction);
        }
        opcode_to_instruction_map
    };
}

/// A `LabelledInstruction` has `call` addresses encoded as label names.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LabelledInstruction {
    /// Instructions belong to the ISA:
    ///
    /// <https://triton-vm.org/spec/isa.html>
    Instruction(AnInstruction<String>),

    /// Labels look like "`<name>:`" and are translated into absolute addresses.
    Label(String),
}

impl Display for LabelledInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LabelledInstruction::Instruction(instr) => write!(f, "{instr}"),
            LabelledInstruction::Label(label_name) => write!(f, "{label_name}:"),
        }
    }
}

/// A Triton VM instruction. See the
/// [Instruction Set Architecture](https://triton-vm.org/spec/isa.html)
/// for more details.
///
/// The type parameter `Dest` describes the type of addresses (absolute or labels).
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, EnumCount, EnumIter, GetSize, Serialize, Deserialize,
)]
pub enum AnInstruction<Dest: PartialEq + Default> {
    // OpStack manipulation
    Pop,
    Push(BFieldElement),
    Divine,
    Dup(OpStackElement),
    Swap(OpStackElement),

    // Control flow
    Nop,
    Skiz,
    Call(Dest),
    Return,
    Recurse,
    Assert,
    Halt,

    // Memory access
    ReadMem,
    WriteMem,

    // Hashing-related
    Hash,
    DivineSibling,
    AssertVector,
    AbsorbInit,
    Absorb,
    Squeeze,

    // Base field arithmetic on stack
    Add,
    Mul,
    Invert,
    Eq,

    // Bitwise arithmetic on stack
    Split,
    Lt,
    And,
    Xor,
    Log2Floor,
    Pow,
    Div,
    PopCount,

    // Extension field arithmetic on stack
    XxAdd,
    XxMul,
    XInvert,
    XbMul,

    // Read/write
    ReadIo,
    WriteIo,
}

impl<Dest: PartialEq + Default> AnInstruction<Dest> {
    /// Assign a unique positive integer to each `Instruction`.
    pub const fn opcode(&self) -> u32 {
        match self {
            Pop => 2,
            Push(_) => 1,
            Divine => 8,
            Dup(_) => 9,
            Swap(_) => 17,
            Nop => 16,
            Skiz => 10,
            Call(_) => 25,
            Return => 24,
            Recurse => 32,
            Assert => 18,
            Halt => 0,
            ReadMem => 40,
            WriteMem => 26,
            Hash => 48,
            DivineSibling => 56,
            AssertVector => 64,
            AbsorbInit => 72,
            Absorb => 80,
            Squeeze => 88,
            Add => 34,
            Mul => 42,
            Invert => 96,
            Eq => 50,
            Split => 4,
            Lt => 6,
            And => 14,
            Xor => 22,
            Log2Floor => 12,
            Pow => 30,
            Div => 20,
            PopCount => 28,
            XxAdd => 104,
            XxMul => 112,
            XInvert => 120,
            XbMul => 58,
            ReadIo => 128,
            WriteIo => 66,
        }
    }

    const fn name(&self) -> &'static str {
        match self {
            Pop => "pop",
            Push(_) => "push",
            Divine => "divine",
            Dup(_) => "dup",
            Swap(_) => "swap",
            Nop => "nop",
            Skiz => "skiz",
            Call(_) => "call",
            Return => "return",
            Recurse => "recurse",
            Assert => "assert",
            Halt => "halt",
            ReadMem => "read_mem",
            WriteMem => "write_mem",
            Hash => "hash",
            DivineSibling => "divine_sibling",
            AssertVector => "assert_vector",
            AbsorbInit => "absorb_init",
            Absorb => "absorb",
            Squeeze => "squeeze",
            Add => "add",
            Mul => "mul",
            Invert => "invert",
            Eq => "eq",
            Split => "split",
            Lt => "lt",
            And => "and",
            Xor => "xor",
            Log2Floor => "log_2_floor",
            Pow => "pow",
            Div => "div",
            PopCount => "pop_count",
            XxAdd => "xxadd",
            XxMul => "xxmul",
            XInvert => "xinvert",
            XbMul => "xbmul",
            ReadIo => "read_io",
            WriteIo => "write_io",
        }
    }

    pub fn opcode_b(&self) -> BFieldElement {
        self.opcode().into()
    }

    pub fn size(&self) -> usize {
        match matches!(self, Push(_) | Dup(_) | Swap(_) | Call(_)) {
            true => 2,
            false => 1,
        }
    }

    /// Get the i'th instruction bit
    pub fn ib(&self, arg: InstructionBit) -> BFieldElement {
        let opcode = self.opcode();
        let bit_number: usize = arg.into();

        ((opcode >> bit_number) & 1).into()
    }

    fn map_call_address<F, NewDest: PartialEq + Default>(&self, f: F) -> AnInstruction<NewDest>
    where
        F: Fn(&Dest) -> NewDest,
    {
        match self {
            Pop => Pop,
            Push(x) => Push(*x),
            Divine => Divine,
            Dup(x) => Dup(*x),
            Swap(x) => Swap(*x),
            Nop => Nop,
            Skiz => Skiz,
            Call(label) => Call(f(label)),
            Return => Return,
            Recurse => Recurse,
            Assert => Assert,
            Halt => Halt,
            ReadMem => ReadMem,
            WriteMem => WriteMem,
            Hash => Hash,
            DivineSibling => DivineSibling,
            AssertVector => AssertVector,
            AbsorbInit => AbsorbInit,
            Absorb => Absorb,
            Squeeze => Squeeze,
            Add => Add,
            Mul => Mul,
            Invert => Invert,
            Eq => Eq,
            Split => Split,
            Lt => Lt,
            And => And,
            Xor => Xor,
            Log2Floor => Log2Floor,
            Pow => Pow,
            Div => Div,
            PopCount => PopCount,
            XxAdd => XxAdd,
            XxMul => XxMul,
            XInvert => XInvert,
            XbMul => XbMul,
            ReadIo => ReadIo,
            WriteIo => WriteIo,
        }
    }
}

impl<Dest: Display + PartialEq + Default> Display for AnInstruction<Dest> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())?;
        match self {
            Push(arg) => write!(f, " {arg}"),
            Dup(arg) | Swap(arg) => write!(f, " {arg}"),
            Call(arg) => write!(f, " {arg}"),
            _ => Ok(()),
        }
    }
}

impl Instruction {
    /// Get the argument of the instruction, if it has one.
    pub fn arg(&self) -> Option<BFieldElement> {
        match self {
            Push(arg) | Call(arg) => Some(*arg),
            Dup(arg) | Swap(arg) => Some(arg.into()),
            _ => None,
        }
    }

    /// `true` iff the instruction has an argument.
    pub fn has_arg(&self) -> bool {
        self.arg().is_some()
    }

    /// Indicates whether the instruction shrinks the operational stack.
    pub fn shrinks_op_stack(&self) -> bool {
        matches!(self, Pop | Skiz | Assert)
            || matches!(self, WriteMem | WriteIo)
            || matches!(self, Add | Mul | Eq | XbMul)
            || matches!(self, And | Xor | Lt | Pow)
    }

    /// Indicates whether the instruction operates on base field elements that are also u32s.
    pub fn is_u32_instruction(&self) -> bool {
        matches!(
            self,
            Split | Lt | And | Xor | Log2Floor | Pow | Div | PopCount
        )
    }

    /// Change the argument of the instruction, if it has one.
    /// Returns `None` if the instruction does not have an argument or
    /// if the argument is out of range.
    pub fn change_arg(&self, new_arg: BFieldElement) -> Option<Self> {
        let maybe_ord_16 = new_arg.value().try_into();
        match self {
            Push(_) => Some(Push(new_arg)),
            Dup(_) => match maybe_ord_16 {
                Ok(ord_16) => Some(Dup(ord_16)),
                Err(_) => None,
            },
            Swap(_) => match maybe_ord_16 {
                Ok(ord_16) => Some(Swap(ord_16)),
                Err(_) => None,
            },
            Call(_) => Some(Call(new_arg)),
            _ => None,
        }
    }
}

impl TryFrom<u32> for Instruction {
    type Error = anyhow::Error;

    fn try_from(opcode: u32) -> Result<Self> {
        OPCODE_TO_INSTRUCTION_MAP
            .get(&opcode)
            .copied()
            .ok_or(anyhow!("No instruction with opcode {opcode} exists."))
    }
}

impl TryFrom<u64> for Instruction {
    type Error = anyhow::Error;

    fn try_from(opcode: u64) -> Result<Self> {
        let opcode = u32::try_from(opcode)?;
        opcode.try_into()
    }
}

impl TryFrom<usize> for Instruction {
    type Error = anyhow::Error;

    fn try_from(opcode: usize) -> Result<Self> {
        let opcode = u32::try_from(opcode)?;
        opcode.try_into()
    }
}

/// Convert a program with labels to a program with absolute positions
pub fn convert_labels(program: &[LabelledInstruction]) -> Vec<Instruction> {
    let mut label_map = HashMap::new();
    let mut instruction_pointer = 0;

    // 1. Add all labels to a map
    for labelled_instruction in program.iter() {
        match labelled_instruction {
            LabelledInstruction::Label(label_name) => {
                label_map.insert(label_name.clone(), instruction_pointer);
            }

            LabelledInstruction::Instruction(instr) => {
                instruction_pointer += instr.size();
            }
        }
    }

    // 2. Convert every label to the lookup value of that map
    program
        .iter()
        .flat_map(|labelled_instruction| convert_labels_helper(labelled_instruction, &label_map))
        .collect()
}

fn convert_labels_helper(
    instruction: &LabelledInstruction,
    label_map: &HashMap<String, usize>,
) -> Vec<Instruction> {
    match instruction {
        LabelledInstruction::Label(_) => vec![],

        LabelledInstruction::Instruction(instr) => {
            let unlabelled_instruction: Instruction = instr.map_call_address(|label_name| {
                let label_not_found = format!("Label not found: {label_name}");
                let absolute_address = label_map.get(label_name).expect(&label_not_found);
                BFieldElement::new(*absolute_address as u64)
            });

            vec![unlabelled_instruction]
        }
    }
}

const fn all_instructions_without_args() -> [AnInstruction<BFieldElement>; Instruction::COUNT] {
    [
        Pop,
        Push(BFIELD_ZERO),
        Divine,
        Dup(ST0),
        Swap(ST0),
        Nop,
        Skiz,
        Call(BFIELD_ZERO),
        Return,
        Recurse,
        Assert,
        Halt,
        ReadMem,
        WriteMem,
        Hash,
        DivineSibling,
        AssertVector,
        AbsorbInit,
        Absorb,
        Squeeze,
        Add,
        Mul,
        Invert,
        Eq,
        Split,
        Lt,
        And,
        Xor,
        Log2Floor,
        Pow,
        Div,
        PopCount,
        XxAdd,
        XxMul,
        XInvert,
        XbMul,
        ReadIo,
        WriteIo,
    ]
}

const fn all_instruction_names() -> [&'static str; Instruction::COUNT] {
    let mut names = [""; Instruction::COUNT];
    let mut i = 0;
    while i < Instruction::COUNT {
        names[i] = ALL_INSTRUCTIONS[i].name();
        i += 1;
    }
    names
}

/// Indicators for all the possible bits in an [`Instruction`](Instruction).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, EnumCount, EnumIter)]
pub enum InstructionBit {
    #[default]
    IB0,
    IB1,
    IB2,
    IB3,
    IB4,
    IB5,
    IB6,
    IB7,
}

impl Display for InstructionBit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bit_index = usize::from(*self);
        write!(f, "{bit_index}")
    }
}

impl From<InstructionBit> for usize {
    fn from(instruction_bit: InstructionBit) -> Self {
        match instruction_bit {
            IB0 => 0,
            IB1 => 1,
            IB2 => 2,
            IB3 => 3,
            IB4 => 4,
            IB5 => 5,
            IB6 => 6,
            IB7 => 7,
        }
    }
}

impl TryFrom<usize> for InstructionBit {
    type Error = String;

    fn try_from(bit_index: usize) -> result::Result<Self, Self::Error> {
        match bit_index {
            0 => Ok(IB0),
            1 => Ok(IB1),
            2 => Ok(IB2),
            3 => Ok(IB3),
            4 => Ok(IB4),
            5 => Ok(IB5),
            6 => Ok(IB6),
            7 => Ok(IB7),
            _ => Err(format!(
                "Index {bit_index} is out of range for `InstructionBit`."
            )),
        }
    }
}

impl From<InstructionBit> for BFieldElement {
    fn from(instruction_bit: InstructionBit) -> Self {
        let instruction_bit = usize::from(instruction_bit) as u64;
        instruction_bit.into()
    }
}

#[cfg(test)]
mod instruction_tests {
    use itertools::Itertools;
    use num_traits::One;
    use num_traits::Zero;
    use std::collections::HashMap;
    use strum::EnumCount;
    use strum::IntoEnumIterator;
    use twenty_first::shared_math::b_field_element::BFieldElement;

    use crate::instruction::all_instruction_names;
    use crate::instruction::all_instructions_without_args;
    use crate::instruction::Instruction;
    use crate::instruction::InstructionBit;
    use crate::instruction::ALL_INSTRUCTIONS;
    use crate::op_stack::OpStackElement::*;
    use crate::program::Program;

    use super::AnInstruction::*;

    #[test]
    fn opcodes_are_unique() {
        let mut opcodes_to_instruction_map = HashMap::new();
        for instruction in Instruction::iter() {
            let opcode = instruction.opcode();
            let maybe_entry = opcodes_to_instruction_map.insert(opcode, instruction);
            if let Some(other_instruction) = maybe_entry {
                panic!("{other_instruction} and {instruction} both have opcode {opcode}.",);
            }
        }
    }

    #[test]
    fn number_of_instruction_bits_is_correct() {
        let all_opcodes = Instruction::iter().map(|instruction| instruction.opcode());
        let highest_opcode = all_opcodes.max().unwrap();
        let num_required_bits_for_highest_opcode = highest_opcode.ilog2() + 1;
        assert_eq!(
            InstructionBit::COUNT,
            num_required_bits_for_highest_opcode as usize
        );
    }

    #[test]
    fn parse_push_pop_test() {
        let code = "
            push 1
            push 1
            add
            pop
        ";
        let program = Program::from_code(code).unwrap();
        let instructions = program.into_iter().collect_vec();
        let expected = vec![
            Push(BFieldElement::one()),
            Push(BFieldElement::one()),
            Add,
            Pop,
        ];

        assert_eq!(expected, instructions);
    }

    #[test]
    fn fail_on_duplicate_labels_test() {
        let code = "
            push 2
            call foo
            bar: push 2
            foo: push 3
            foo: push 4
            halt
        ";
        let program = Program::from_code(code);
        assert!(
            program.is_err(),
            "Duplicate labels should result in a parse error"
        );
    }

    #[test]
    fn ib_registers_are_binary_test() {
        for instruction in ALL_INSTRUCTIONS {
            for instruction_bit in InstructionBit::iter() {
                let ib_value = instruction.ib(instruction_bit);
                assert!(ib_value.is_zero() || ib_value.is_one(),);
            }
        }
    }

    #[test]
    fn instruction_to_opcode_to_instruction_is_consistent_test() {
        for instr in ALL_INSTRUCTIONS {
            assert_eq!(instr, instr.opcode().try_into().unwrap());
        }
    }

    #[test]
    fn print_all_instructions_and_opcodes() {
        for instr in ALL_INSTRUCTIONS {
            println!("{:>3} {: <10}", instr.opcode(), format!("{}", instr.name()));
        }
    }

    #[test]
    /// Serves no other purpose than to increase code coverage results.
    fn run_constant_methods_test() {
        all_instructions_without_args();
        all_instruction_names();
    }

    #[test]
    fn convert_various_types_to_instructions() {
        let _push = Instruction::try_from(1_usize).unwrap();
        let _dup = Instruction::try_from(9_u64).unwrap();
        let _swap = Instruction::try_from(17_u32).unwrap();
        let _pop = Instruction::try_from(2_usize).unwrap();
    }

    #[test]
    fn change_arguments_of_various_instructions() {
        let push = Instruction::Push(0_u64.into()).change_arg(7_u64.into());
        let dup = Instruction::Dup(ST0).change_arg(1024_u64.into());
        let swap = Instruction::Swap(ST0).change_arg(1337_u64.into());
        let pop = Instruction::Pop.change_arg(7_u64.into());

        assert!(push.is_some());
        assert!(dup.is_none());
        assert!(swap.is_none());
        assert!(pop.is_none());
    }

    #[test]
    fn print_various_instructions() {
        println!("instruction_push: {:?}", Instruction::Push(7_u64.into()));
        println!("instruction_assert: {}", Instruction::Assert);
        println!("instruction_invert: {:?}", Instruction::Invert);
        println!("instruction_dup: {}", Instruction::Dup(ST14));
    }

    #[test]
    fn instruction_size_is_consistent_with_having_arguments() {
        for instruction in Instruction::iter() {
            match instruction.has_arg() {
                true => assert_eq!(2, instruction.size()),
                false => assert_eq!(1, instruction.size()),
            }
        }
    }

    #[test]
    fn opcodes_are_consistent_with_argument_indication_bit() {
        let argument_indicator_bit_mask = 1;
        for instruction in Instruction::iter() {
            let opcode = instruction.opcode();
            println!("Testing instruction {instruction} with opcode {opcode}.");
            assert_eq!(
                instruction.has_arg(),
                opcode & argument_indicator_bit_mask != 0
            );
        }
    }

    #[test]
    fn opcodes_are_consistent_with_shrink_stack_indication_bit() {
        let shrink_stack_indicator_bit_mask = 2;
        for instruction in Instruction::iter() {
            let opcode = instruction.opcode();
            println!("Testing instruction {instruction} with opcode {opcode}.");
            assert_eq!(
                instruction.shrinks_op_stack(),
                opcode & shrink_stack_indicator_bit_mask != 0
            );
        }
    }

    #[test]
    fn opcodes_are_consistent_with_u32_indication_bit() {
        let u32_indicator_bit_mask = 4;
        for instruction in Instruction::iter() {
            let opcode = instruction.opcode();
            println!("Testing instruction {instruction} with opcode {opcode}.");
            assert_eq!(
                instruction.is_u32_instruction(),
                opcode & u32_indicator_bit_mask != 0
            );
        }
    }

    #[test]
    fn instruction_bits_are_consistent() {
        for instruction_bit in InstructionBit::iter() {
            println!("Testing instruction bit {instruction_bit}.");
            let bit_index = usize::from(instruction_bit);
            let recovered_instruction_bit = InstructionBit::try_from(bit_index).unwrap();
            assert_eq!(instruction_bit, recovered_instruction_bit);
        }
    }
}
