use crate::providers::operations::sync::hlc::mutable_timestamp::MutableTimestamp;
use merkle::MerkleTree;

#[derive(Clone, Debug)]
pub struct Clock {
    pub timestamp: MutableTimestamp,
    pub merkle: MerkleTree,
}
