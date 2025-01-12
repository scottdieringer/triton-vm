use arbitrary::Arbitrary;
use get_size::GetSize;
use itertools::Itertools;
use serde::Deserialize;
use serde::Serialize;
use twenty_first::shared_math::b_field_element::BFieldElement;
use twenty_first::shared_math::bfield_codec::BFieldCodec;
use twenty_first::shared_math::tip5::Digest;

use crate::error::ProofStreamError;
use crate::proof_stream::ProofStream;
use crate::stark;

/// Contains the necessary cryptographic information to verify a computation.
/// Should be used together with a [`Claim`].
#[derive(Debug, Clone, GetSize, Serialize, Deserialize, PartialEq, Eq, BFieldCodec, Arbitrary)]
pub struct Proof(pub Vec<BFieldElement>);

impl Proof {
    /// Get the height of the trace used during proof generation.
    /// This is an upper bound on the length of the computation this proof is for.
    /// It it one of the main contributing factors to the length of the FRI domain.
    pub fn padded_height(&self) -> Result<usize, ProofStreamError> {
        let proof_stream = ProofStream::<stark::StarkHasher>::try_from(self)?;
        let proof_items = proof_stream.items.iter();
        let log_2_padded_heights = proof_items
            .filter_map(|item| item.as_log2_padded_height().ok())
            .collect_vec();

        if log_2_padded_heights.is_empty() {
            return Err(ProofStreamError::NoLog2PaddedHeight);
        }
        if log_2_padded_heights.len() > 1 {
            return Err(ProofStreamError::TooManyLog2PaddedHeights);
        }
        Ok(1 << log_2_padded_heights[0])
    }
}

/// Contains the public information of a verifiably correct computation.
/// A corresponding [`Proof`] is needed to verify the computation.
/// One additional piece of public information not explicitly listed in the [`Claim`] is the
/// `padded_height`, an upper bound on the length of the computation.
/// It is derivable from a [`Proof`] by calling [`Proof::padded_height()`].
#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, Eq, GetSize, BFieldCodec, Hash, Arbitrary,
)]
pub struct Claim {
    /// The hash digest of the program that was executed. The hash function in use is Tip5.
    pub program_digest: Digest,

    /// The public input to the computation.
    pub input: Vec<BFieldElement>,

    /// The public output of the computation.
    pub output: Vec<BFieldElement>,
}

impl Claim {
    /// The public input as `u64`s.
    /// If `BFieldElement`s are needed, use field `input`.
    pub fn public_input(&self) -> Vec<u64> {
        self.input.iter().map(|x| x.value()).collect()
    }

    /// The public output as `u64`.
    /// If `BFieldElements`s are needed, use field `output`.
    pub fn public_output(&self) -> Vec<u64> {
        self.output.iter().map(|x| x.value()).collect()
    }
}

#[cfg(test)]
mod tests {
    use assert2::assert;
    use proptest::collection::vec;
    use proptest_arbitrary_interop::arb;
    use rand::random;
    use test_strategy::proptest;
    use twenty_first::shared_math::b_field_element::BFieldElement;
    use twenty_first::shared_math::bfield_codec::BFieldCodec;
    use twenty_first::shared_math::other::random_elements;

    use crate::proof_item::ProofItem;
    use crate::stark::StarkHasher;

    use super::*;

    #[test]
    fn decode_proof() {
        let data: Vec<BFieldElement> = random_elements(348);
        let proof = Proof(data);

        let encoded = proof.encode();
        let decoded = *Proof::decode(&encoded).unwrap();

        assert!(proof == decoded);
    }

    #[test]
    fn decode_claim() {
        let claim = Claim {
            program_digest: random(),
            input: random_elements(346),
            output: random_elements(125),
        };

        let encoded = claim.encode();
        let decoded = *Claim::decode(&encoded).unwrap();

        assert!(claim.program_digest == decoded.program_digest);
        assert!(claim.input == decoded.input);
        assert!(claim.output == decoded.output);
    }

    #[test]
    fn proof_with_no_log_2_padded_height_gives_err() {
        let mut proof_stream = ProofStream::<StarkHasher>::new();
        proof_stream.enqueue(ProofItem::MerkleRoot(random()));
        let proof: Proof = proof_stream.into();
        let maybe_padded_height = proof.padded_height();
        assert!(maybe_padded_height.is_err());
    }

    #[test]
    fn proof_with_multiple_log_2_padded_height_gives_err() {
        let mut proof_stream = ProofStream::<StarkHasher>::new();
        proof_stream.enqueue(ProofItem::Log2PaddedHeight(8));
        proof_stream.enqueue(ProofItem::MerkleRoot(random()));
        proof_stream.enqueue(ProofItem::Log2PaddedHeight(7));
        let proof: Proof = proof_stream.into();
        let maybe_padded_height = proof.padded_height();
        assert!(maybe_padded_height.is_err());
    }

    #[proptest]
    fn decoding_arbitrary_proof_data_does_not_panic(
        #[strategy(vec(arb(), 0..1_000))] proof_data: Vec<BFieldElement>,
    ) {
        let _ = Proof::decode(&proof_data);
    }
}
