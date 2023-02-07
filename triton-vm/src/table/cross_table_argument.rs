use itertools::Itertools;
use std::ops::Add;
use std::ops::Mul;

use ndarray::ArrayView1;
use num_traits::One;
use num_traits::Zero;
use strum_macros::Display;
use strum_macros::EnumCount as EnumCountMacro;
use strum_macros::EnumIter;
use twenty_first::shared_math::b_field_element::BFieldElement;
use twenty_first::shared_math::mpolynomial::Degree;
use twenty_first::shared_math::traits::Inverse;
use twenty_first::shared_math::x_field_element::XFieldElement;

use CrossTableChallengeId::*;

use crate::table::challenges::AllChallenges;
use crate::table::challenges::TableChallenges;
use crate::table::extension_table::Evaluable;
use crate::table::extension_table::Quotientable;
use crate::table::processor_table::PROCESSOR_TABLE_NUM_EVALUATION_ARGUMENTS;
use crate::table::processor_table::PROCESSOR_TABLE_NUM_PERMUTATION_ARGUMENTS;
use crate::table::table_column::HashExtTableColumn;
use crate::table::table_column::JumpStackExtTableColumn;
use crate::table::table_column::MasterExtTableColumn;
use crate::table::table_column::OpStackExtTableColumn;
use crate::table::table_column::ProcessorExtTableColumn;
use crate::table::table_column::ProgramExtTableColumn;
use crate::table::table_column::RamExtTableColumn;
use crate::table::table_column::U32ExtTableColumn;

pub const NUM_PUBLIC_EVAL_ARGS: usize = 2; // for public input and output
pub const NUM_PRIVATE_EVAL_ARGS: usize =
    PROCESSOR_TABLE_NUM_EVALUATION_ARGUMENTS - NUM_PUBLIC_EVAL_ARGS;
pub const NUM_PRIVATE_PERM_ARGS: usize = PROCESSOR_TABLE_NUM_PERMUTATION_ARGUMENTS;
pub const NUM_LOOKUP_ARGS: usize = 2;
pub const NUM_CROSS_TABLE_ARGS: usize =
    NUM_PRIVATE_PERM_ARGS + NUM_PRIVATE_EVAL_ARGS + NUM_LOOKUP_ARGS;
pub const NUM_CROSS_TABLE_WEIGHTS: usize = NUM_CROSS_TABLE_ARGS + NUM_PUBLIC_EVAL_ARGS;

pub trait CrossTableArg {
    fn default_initial() -> XFieldElement
    where
        Self: Sized;

    fn compute_terminal(
        symbols: &[BFieldElement],
        initial: XFieldElement,
        challenge: XFieldElement,
    ) -> XFieldElement
    where
        Self: Sized;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PermArg {}

impl CrossTableArg for PermArg {
    fn default_initial() -> XFieldElement {
        XFieldElement::one()
    }

    /// Compute the product for a permutation argument as specified by `initial`, `challenge`,
    /// and `symbols`. This amounts to evaluating polynomial
    ///  `f(x) = initial · Π_i (x - symbols[i])`
    /// at point `challenge`, i.e., returns `f(challenge)`.
    fn compute_terminal(
        symbols: &[BFieldElement],
        initial: XFieldElement,
        challenge: XFieldElement,
    ) -> XFieldElement {
        symbols
            .iter()
            .map(|&symbol| challenge - symbol)
            .fold(initial, XFieldElement::mul)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct EvalArg {}

impl CrossTableArg for EvalArg {
    fn default_initial() -> XFieldElement {
        XFieldElement::one()
    }

    /// Compute the evaluation for an evaluation argument as specified by `initial`, `challenge`,
    /// and `symbols`. This amounts to evaluating polynomial
    /// `f(x) = initial·x^n + Σ_i symbols[n-i]·x^i`
    /// at point `challenge`, i.e., returns `f(challenge)`.
    fn compute_terminal(
        symbols: &[BFieldElement],
        initial: XFieldElement,
        challenge: XFieldElement,
    ) -> XFieldElement {
        symbols.iter().fold(initial, |running_evaluation, &symbol| {
            challenge * running_evaluation + symbol
        })
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct LookupArg {}

impl CrossTableArg for LookupArg {
    fn default_initial() -> XFieldElement {
        XFieldElement::zero()
    }

    fn compute_terminal(
        symbols: &[BFieldElement],
        initial: XFieldElement,
        challenge: XFieldElement,
    ) -> XFieldElement {
        symbols
            .iter()
            .map(|symbol| (challenge - symbol.lift()).inverse())
            .fold(initial, XFieldElement::add)
    }
}

impl LookupArg {
    pub fn compute_terminal_with_multiplicities(
        symbols: &[BFieldElement],
        multiplicities: &[u32],
        initial: XFieldElement,
        challenge: XFieldElement,
    ) -> XFieldElement {
        symbols
            .iter()
            .zip_eq(multiplicities.iter())
            .map(|(symbol, &multiplicity)| {
                (challenge - symbol.lift()).inverse() * XFieldElement::from(multiplicity)
            })
            .fold(initial, XFieldElement::add)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GrandCrossTableArg {}

#[derive(Clone, Debug)]
pub struct CrossTableChallenges {
    pub input_terminal: XFieldElement,
    pub output_terminal: XFieldElement,

    pub processor_to_program_weight: XFieldElement,
    pub processor_to_op_stack_weight: XFieldElement,
    pub processor_to_ram_weight: XFieldElement,
    pub processor_to_jump_stack_weight: XFieldElement,
    pub hash_input_weight: XFieldElement,
    pub hash_digest_weight: XFieldElement,
    pub sponge_weight: XFieldElement,
    pub processor_to_u32_weight: XFieldElement,
    pub clock_jump_difference_lookup_weight: XFieldElement,
    pub input_to_processor_weight: XFieldElement,
    pub processor_to_output_weight: XFieldElement,
}

#[derive(Debug, Copy, Clone, Display, EnumCountMacro, EnumIter, PartialEq, Eq, Hash)]
pub enum CrossTableChallengeId {
    InputTerminal,
    OutputTerminal,

    ProcessorToProgramWeight,
    ProcessorToOpStackWeight,
    ProcessorToRamWeight,
    ProcessorToJumpStackWeight,
    HashInputWeight,
    HashDigestWeight,
    SpongeWeight,
    ProcessorToU32Weight,
    ClockJumpDifferenceLookupWeight,
    InputToProcessorWeight,
    ProcessorToOutputWeight,
}

impl From<CrossTableChallengeId> for usize {
    fn from(val: CrossTableChallengeId) -> Self {
        val as usize
    }
}

impl TableChallenges for CrossTableChallenges {
    type Id = CrossTableChallengeId;

    #[inline]
    fn get_challenge(&self, id: Self::Id) -> XFieldElement {
        match id {
            InputTerminal => self.input_terminal,
            OutputTerminal => self.output_terminal,
            ProcessorToProgramWeight => self.processor_to_program_weight,
            ProcessorToOpStackWeight => self.processor_to_op_stack_weight,
            ProcessorToRamWeight => self.processor_to_ram_weight,
            ProcessorToJumpStackWeight => self.processor_to_jump_stack_weight,
            HashInputWeight => self.hash_input_weight,
            HashDigestWeight => self.hash_digest_weight,
            SpongeWeight => self.sponge_weight,
            ProcessorToU32Weight => self.processor_to_u32_weight,
            ClockJumpDifferenceLookupWeight => self.clock_jump_difference_lookup_weight,
            InputToProcessorWeight => self.input_to_processor_weight,
            ProcessorToOutputWeight => self.processor_to_output_weight,
        }
    }
}

impl Evaluable for GrandCrossTableArg {
    fn evaluate_initial_constraints(
        _base_row: ArrayView1<BFieldElement>,
        _ext_row: ArrayView1<XFieldElement>,
        _challenges: &AllChallenges,
    ) -> Vec<XFieldElement> {
        vec![]
    }

    fn evaluate_consistency_constraints(
        _base_row: ArrayView1<BFieldElement>,
        _ext_row: ArrayView1<XFieldElement>,
        _challenges: &AllChallenges,
    ) -> Vec<XFieldElement> {
        vec![]
    }

    fn evaluate_transition_constraints(
        _current_base_row: ArrayView1<BFieldElement>,
        _current_ext_row: ArrayView1<XFieldElement>,
        _next_base_row: ArrayView1<BFieldElement>,
        _next_ext_row: ArrayView1<XFieldElement>,
        _challenges: &AllChallenges,
    ) -> Vec<XFieldElement> {
        vec![]
    }

    fn evaluate_terminal_constraints(
        _base_row: ArrayView1<BFieldElement>,
        ext_row: ArrayView1<XFieldElement>,
        challenges: &AllChallenges,
    ) -> Vec<XFieldElement> {
        let challenges = &challenges.cross_table_challenges;

        let input_to_processor = challenges.get_challenge(InputTerminal)
            - ext_row[ProcessorExtTableColumn::InputTableEvalArg.master_ext_table_index()];
        let processor_to_output = ext_row
            [ProcessorExtTableColumn::OutputTableEvalArg.master_ext_table_index()]
            - challenges.get_challenge(OutputTerminal);

        let instruction_lookup = ext_row
            [ProcessorExtTableColumn::InstructionLookupClientLogDerivative
                .master_ext_table_index()]
            - ext_row[ProgramExtTableColumn::InstructionLookupServerLogDerivative
                .master_ext_table_index()];
        let processor_to_op_stack = ext_row
            [ProcessorExtTableColumn::OpStackTablePermArg.master_ext_table_index()]
            - ext_row[OpStackExtTableColumn::RunningProductPermArg.master_ext_table_index()];
        let processor_to_ram = ext_row
            [ProcessorExtTableColumn::RamTablePermArg.master_ext_table_index()]
            - ext_row[RamExtTableColumn::RunningProductPermArg.master_ext_table_index()];
        let processor_to_jump_stack = ext_row
            [ProcessorExtTableColumn::JumpStackTablePermArg.master_ext_table_index()]
            - ext_row[JumpStackExtTableColumn::RunningProductPermArg.master_ext_table_index()];
        let hash_input = ext_row
            [ProcessorExtTableColumn::HashInputEvalArg.master_ext_table_index()]
            - ext_row[HashExtTableColumn::HashInputRunningEvaluation.master_ext_table_index()];
        let hash_digest = ext_row
            [HashExtTableColumn::HashDigestRunningEvaluation.master_ext_table_index()]
            - ext_row[ProcessorExtTableColumn::HashDigestEvalArg.master_ext_table_index()];
        let sponge = ext_row[ProcessorExtTableColumn::SpongeEvalArg.master_ext_table_index()]
            - ext_row[HashExtTableColumn::SpongeRunningEvaluation.master_ext_table_index()];
        let processor_to_u32 = ext_row
            [ProcessorExtTableColumn::U32TablePermArg.master_ext_table_index()]
            - ext_row[U32ExtTableColumn::ProcessorPermArg.master_ext_table_index()];
        let clock_jump_difference_lookup = ext_row
            [ProcessorExtTableColumn::ClockJumpDifferenceLookupServerLogDerivative
                .master_ext_table_index()]
            - ext_row[OpStackExtTableColumn::ClockJumpDifferenceLookupClientLogDerivative
                .master_ext_table_index()]
            - ext_row[RamExtTableColumn::ClockJumpDifferenceLookupClientLogDerivative
                .master_ext_table_index()]
            - ext_row[JumpStackExtTableColumn::ClockJumpDifferenceLookupClientLogDerivative
                .master_ext_table_index()];

        let non_linear_sum = challenges.get_challenge(ProcessorToProgramWeight)
            * instruction_lookup
            + challenges.get_challenge(InputToProcessorWeight) * input_to_processor
            + challenges.get_challenge(ProcessorToOutputWeight) * processor_to_output
            + challenges.get_challenge(ProcessorToOpStackWeight) * processor_to_op_stack
            + challenges.get_challenge(ProcessorToRamWeight) * processor_to_ram
            + challenges.get_challenge(ProcessorToJumpStackWeight) * processor_to_jump_stack
            + challenges.get_challenge(HashInputWeight) * hash_input
            + challenges.get_challenge(HashDigestWeight) * hash_digest
            + challenges.get_challenge(SpongeWeight) * sponge
            + challenges.get_challenge(ProcessorToU32Weight) * processor_to_u32
            + challenges.get_challenge(ClockJumpDifferenceLookupWeight)
                * clock_jump_difference_lookup;
        vec![non_linear_sum]
    }
}

impl Quotientable for GrandCrossTableArg {
    fn num_initial_quotients() -> usize {
        0
    }

    fn num_consistency_quotients() -> usize {
        0
    }

    fn num_transition_quotients() -> usize {
        0
    }

    fn num_terminal_quotients() -> usize {
        1
    }

    fn initial_quotient_degree_bounds(_interpolant_degree: Degree) -> Vec<Degree> {
        vec![]
    }

    fn consistency_quotient_degree_bounds(
        _interpolant_degree: Degree,
        _padded_height: usize,
    ) -> Vec<Degree> {
        vec![]
    }

    fn transition_quotient_degree_bounds(
        _interpolant_degree: Degree,
        _padded_height: usize,
    ) -> Vec<Degree> {
        vec![]
    }

    fn terminal_quotient_degree_bounds(interpolant_degree: Degree) -> Vec<Degree> {
        let zerofier_degree = 1 as Degree;
        let max_columns_involved_in_one_cross_table_argument = 1;
        vec![
            interpolant_degree * max_columns_involved_in_one_cross_table_argument - zerofier_degree,
        ]
    }
}