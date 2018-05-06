use ethereum_types;

/// Advanced types used to create an aliases
pub type ShardIdHash = ethereum_types::U256;
pub type ChunkRootHash = ethereum_types::H256;
pub type ChunkPeriodHash = ethereum_types::U256;

pub type ProposerIdHash = ethereum_types::U256;
pub type NotaryIdHash = ethereum_types::U256;

pub type ProposerAddress = ethereum_types::Address; // ethereum_types::H160;
pub type NotaryAddress = ethereum_types::Address; // ethereum_types::H160;

pub type CollationHeaderHash = ethereum_types::H256;
pub type ParentCollationHeaderHash = ethereum_types::H256;

pub type ProposerBidHash = ethereum_types::U256;
pub type ProposerSignature = ethereum_types::Signature;

pub type Chunk = ethereum_types::U256;