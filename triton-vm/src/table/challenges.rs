use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;

use strum::EnumCount;
use strum::IntoEnumIterator;
use twenty_first::shared_math::b_field_element::BFieldElement;
use twenty_first::shared_math::other::random_elements;
use twenty_first::shared_math::x_field_element::XFieldElement;

use crate::table::cross_table_argument::CrossTableArg;
use crate::table::cross_table_argument::CrossTableChallenges;
use crate::table::cross_table_argument::EvalArg;
use crate::table::cross_table_argument::NUM_CROSS_TABLE_WEIGHTS;
use crate::table::hash_table::HashTableChallenges;
use crate::table::jump_stack_table::JumpStackTableChallenges;
use crate::table::op_stack_table::OpStackTableChallenges;
use crate::table::processor_table::IOChallenges;
use crate::table::processor_table::ProcessorTableChallenges;
use crate::table::program_table::ProgramTableChallenges;
use crate::table::ram_table::RamTableChallenges;
use crate::table::u32_table::U32TableChallenges;

pub trait TableChallenges: Clone + Debug {
    type Id: Display
        + Clone
        + Copy
        + Debug
        + EnumCount
        + IntoEnumIterator
        + Into<usize>
        + PartialEq
        + Eq
        + Hash;

    fn count() -> usize {
        Self::Id::COUNT
    }

    fn get_challenge(&self, id: Self::Id) -> XFieldElement;

    fn to_vec(&self) -> Vec<XFieldElement> {
        Self::Id::iter().map(|id| self.get_challenge(id)).collect()
    }
}

#[derive(Debug, Clone)]
pub struct AllChallenges {
    pub program_table_challenges: ProgramTableChallenges,
    pub input_challenges: IOChallenges,
    pub output_challenges: IOChallenges,
    pub processor_table_challenges: ProcessorTableChallenges,
    pub op_stack_table_challenges: OpStackTableChallenges,
    pub ram_table_challenges: RamTableChallenges,
    pub jump_stack_table_challenges: JumpStackTableChallenges,
    pub hash_table_challenges: HashTableChallenges,
    pub u32_table_challenges: U32TableChallenges,
    pub cross_table_challenges: CrossTableChallenges,
}

impl AllChallenges {
    pub const TOTAL_CHALLENGES: usize = 52 + NUM_CROSS_TABLE_WEIGHTS;

    pub fn create_challenges(
        mut weights: Vec<XFieldElement>,
        claimed_input: &[BFieldElement],
        claimed_output: &[BFieldElement],
    ) -> Self {
        let processor_table_challenges = ProcessorTableChallenges {
            standard_input_eval_indeterminate: weights.pop().unwrap(),
            standard_output_eval_indeterminate: weights.pop().unwrap(),
            hash_input_eval_indeterminate: weights.pop().unwrap(),
            hash_digest_eval_indeterminate: weights.pop().unwrap(),
            sponge_eval_indeterminate: weights.pop().unwrap(),
            instruction_perm_indeterminate: weights.pop().unwrap(),
            op_stack_perm_indeterminate: weights.pop().unwrap(),
            ram_perm_indeterminate: weights.pop().unwrap(),
            jump_stack_perm_indeterminate: weights.pop().unwrap(),

            instruction_table_ip_weight: weights.pop().unwrap(),
            instruction_table_ci_processor_weight: weights.pop().unwrap(),
            instruction_table_nia_weight: weights.pop().unwrap(),

            op_stack_table_clk_weight: weights.pop().unwrap(),
            op_stack_table_ib1_weight: weights.pop().unwrap(),
            op_stack_table_osp_weight: weights.pop().unwrap(),
            op_stack_table_osv_weight: weights.pop().unwrap(),

            ram_table_clk_weight: weights.pop().unwrap(),
            ram_table_ramp_weight: weights.pop().unwrap(),
            ram_table_ramv_weight: weights.pop().unwrap(),
            ram_table_previous_instruction_weight: weights.pop().unwrap(),

            jump_stack_table_clk_weight: weights.pop().unwrap(),
            jump_stack_table_ci_weight: weights.pop().unwrap(),
            jump_stack_table_jsp_weight: weights.pop().unwrap(),
            jump_stack_table_jso_weight: weights.pop().unwrap(),
            jump_stack_table_jsd_weight: weights.pop().unwrap(),

            unique_clock_jump_differences_eval_indeterminate: weights.pop().unwrap(),
            all_clock_jump_differences_multi_perm_indeterminate: weights.pop().unwrap(),

            hash_table_ci_weight: weights.pop().unwrap(),
            hash_state_weight0: weights.pop().unwrap(),
            hash_state_weight1: weights.pop().unwrap(),
            hash_state_weight2: weights.pop().unwrap(),
            hash_state_weight3: weights.pop().unwrap(),
            hash_state_weight4: weights.pop().unwrap(),
            hash_state_weight5: weights.pop().unwrap(),
            hash_state_weight6: weights.pop().unwrap(),
            hash_state_weight7: weights.pop().unwrap(),
            hash_state_weight8: weights.pop().unwrap(),
            hash_state_weight9: weights.pop().unwrap(),

            u32_table_lhs_weight: weights.pop().unwrap(),
            u32_table_rhs_weight: weights.pop().unwrap(),
            u32_table_ci_weight: weights.pop().unwrap(),
            u32_table_result_weight: weights.pop().unwrap(),

            u32_table_perm_indeterminate: weights.pop().unwrap(),
        };

        let program_table_challenges = ProgramTableChallenges {
            instruction_eval_indeterminate: weights.pop().unwrap(),
            address_weight: weights.pop().unwrap(),
            instruction_weight: weights.pop().unwrap(),
            next_instruction_weight: weights.pop().unwrap(),
        };

        let input_challenges = IOChallenges {
            processor_eval_indeterminate: processor_table_challenges
                .standard_input_eval_indeterminate,
        };

        let output_challenges = IOChallenges {
            processor_eval_indeterminate: processor_table_challenges
                .standard_output_eval_indeterminate,
        };

        let op_stack_table_challenges = OpStackTableChallenges {
            processor_perm_indeterminate: processor_table_challenges.op_stack_perm_indeterminate,
            clk_weight: processor_table_challenges.op_stack_table_clk_weight,
            ib1_weight: processor_table_challenges.op_stack_table_ib1_weight,
            osv_weight: processor_table_challenges.op_stack_table_osv_weight,
            osp_weight: processor_table_challenges.op_stack_table_osp_weight,
            all_clock_jump_differences_multi_perm_indeterminate: processor_table_challenges
                .all_clock_jump_differences_multi_perm_indeterminate,
        };

        let ram_table_challenges = RamTableChallenges {
            bezout_relation_indeterminate: weights.pop().unwrap(),
            processor_perm_indeterminate: processor_table_challenges.ram_perm_indeterminate,
            clk_weight: processor_table_challenges.ram_table_clk_weight,
            ramp_weight: processor_table_challenges.ram_table_ramp_weight,
            ramv_weight: processor_table_challenges.ram_table_ramv_weight,
            previous_instruction_weight: processor_table_challenges
                .ram_table_previous_instruction_weight,
            all_clock_jump_differences_multi_perm_indeterminate: processor_table_challenges
                .all_clock_jump_differences_multi_perm_indeterminate,
        };

        let jump_stack_table_challenges = JumpStackTableChallenges {
            processor_perm_indeterminate: processor_table_challenges.jump_stack_perm_indeterminate,
            clk_weight: processor_table_challenges.jump_stack_table_clk_weight,
            ci_weight: processor_table_challenges.jump_stack_table_ci_weight,
            jsp_weight: processor_table_challenges.jump_stack_table_jsp_weight,
            jso_weight: processor_table_challenges.jump_stack_table_jso_weight,
            jsd_weight: processor_table_challenges.jump_stack_table_jsd_weight,
            all_clock_jump_differences_multi_perm_indeterminate: processor_table_challenges
                .all_clock_jump_differences_multi_perm_indeterminate,
        };

        let hash_table_challenges = HashTableChallenges {
            hash_input_eval_indeterminate: processor_table_challenges.hash_input_eval_indeterminate,
            hash_digest_eval_indeterminate: processor_table_challenges
                .hash_digest_eval_indeterminate,
            sponge_eval_indeterminate: processor_table_challenges.sponge_eval_indeterminate,

            ci_weight: processor_table_challenges.hash_table_ci_weight,
            hash_state_weight0: processor_table_challenges.hash_state_weight0,
            hash_state_weight1: processor_table_challenges.hash_state_weight1,
            hash_state_weight2: processor_table_challenges.hash_state_weight2,
            hash_state_weight3: processor_table_challenges.hash_state_weight3,
            hash_state_weight4: processor_table_challenges.hash_state_weight4,
            hash_state_weight5: processor_table_challenges.hash_state_weight5,
            hash_state_weight6: processor_table_challenges.hash_state_weight6,
            hash_state_weight7: processor_table_challenges.hash_state_weight7,
            hash_state_weight8: processor_table_challenges.hash_state_weight8,
            hash_state_weight9: processor_table_challenges.hash_state_weight9,
            hash_state_weight10: weights.pop().unwrap(),
            hash_state_weight11: weights.pop().unwrap(),
            hash_state_weight12: weights.pop().unwrap(),
            hash_state_weight13: weights.pop().unwrap(),
            hash_state_weight14: weights.pop().unwrap(),
            hash_state_weight15: weights.pop().unwrap(),
        };

        let u32_table_challenges = U32TableChallenges {
            lhs_weight: processor_table_challenges.u32_table_lhs_weight,
            rhs_weight: processor_table_challenges.u32_table_rhs_weight,
            ci_weight: processor_table_challenges.u32_table_ci_weight,
            result_weight: processor_table_challenges.u32_table_result_weight,
            processor_perm_indeterminate: processor_table_challenges.u32_table_perm_indeterminate,
        };

        let input_terminal = EvalArg::compute_terminal(
            claimed_input,
            EvalArg::default_initial(),
            processor_table_challenges.standard_input_eval_indeterminate,
        );
        let output_terminal = EvalArg::compute_terminal(
            claimed_output,
            EvalArg::default_initial(),
            processor_table_challenges.standard_output_eval_indeterminate,
        );

        let cross_table_challenges = CrossTableChallenges {
            input_terminal,
            output_terminal,
            program_to_instruction_weight: weights.pop().unwrap(),
            processor_to_instruction_weight: weights.pop().unwrap(),
            processor_to_op_stack_weight: weights.pop().unwrap(),
            processor_to_ram_weight: weights.pop().unwrap(),
            processor_to_jump_stack_weight: weights.pop().unwrap(),
            hash_input_weight: weights.pop().unwrap(),
            hash_digest_weight: weights.pop().unwrap(),
            sponge_weight: weights.pop().unwrap(),
            processor_to_u32_weight: weights.pop().unwrap(),
            all_clock_jump_differences_weight: weights.pop().unwrap(),
            input_to_processor_weight: weights.pop().unwrap(),
            processor_to_output_weight: weights.pop().unwrap(),
        };

        assert!(weights.is_empty(), "{} weights left unused.", weights.len());

        AllChallenges {
            program_table_challenges,
            input_challenges,
            output_challenges,
            processor_table_challenges,
            op_stack_table_challenges,
            ram_table_challenges,
            jump_stack_table_challenges,
            hash_table_challenges,
            u32_table_challenges,
            cross_table_challenges,
        }
    }

    /// Stand-in challenges. Can be used in tests. For non-interactive STARKs, use Fiat-Shamir to
    /// derive the actual challenges.
    pub fn placeholder(claimed_input: &[BFieldElement], claimed_output: &[BFieldElement]) -> Self {
        Self::create_challenges(
            random_elements(Self::TOTAL_CHALLENGES),
            claimed_input,
            claimed_output,
        )
    }
}
