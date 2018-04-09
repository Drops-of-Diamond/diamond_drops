use ethereum_types;

pub struct Header {
    shard_id: ethereum_types::U256,
    parent_hash: ethereum_types::H256,
    chunk_root: ethereum_types::H256,
    proposer_address: ethereum_types::Address,
    proposer_bid: ethereum_types::U256,
    proposer_signature: ethereum_types::Signature
}

impl Header {
    pub fn new(shard_id: ethereum_types::U256, 
               parent_hash: ethereum_types::H256,
               chunk_root: ethereum_types::H256,
               proposer_address: ethereum_types::Address,
               proposer_bid: ethereum_types::U256,
               proposer_signature: ethereum_types::Signature) -> Header {
        
        Header {
            shard_id,
            parent_hash,
            chunk_root,
            proposer_address,
            proposer_bid,
            proposer_signature
        }
    }
}