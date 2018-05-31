use ethereum_types;

use modules::primitives::{
    ShardIdHash,
    ChunkRootHash,
    ChunkPeriodHash,
    ProposerAddress,
    CollationHeaderHash,
    //ParentCollationHeaderHash,
    //ProposerBidHash,
    //ProposerSignature
};

use modules::constants::{
    SAMPLE_COLLATION_PARENT_HASH_BYTES,
    SAMPLE_COLLATION_CHUNK_ROOT_BYTES,
    SAMPLE_COLLATION_PROPOSER_ADDRESS_BYTES
};

use tiny_keccak;

#[derive(PartialEq, Debug, Clone)]
pub struct Header {

    pub shard_id: ShardIdHash,
    proposer_address: ProposerAddress,
    chunk_root: ChunkRootHash,
    period: ChunkPeriodHash,

    // The following fields are pending updates to the sharding spec and are currently ignored
    //parent_hash: ParentCollationHeaderHash,
    //proposer_bid: ProposerBidHash,
    //proposer_signature: ProposerSignature
}

impl Header {
    pub fn new(shard_id: ShardIdHash,
               //parent_hash: ParentCollationHeaderHash,
               chunk_root: ChunkRootHash,
               period: ChunkPeriodHash,
               proposer_address: ProposerAddress,
               //proposer_bid: ProposerBidHash,
               /*proposer_signature: ProposerSignature*/) -> Header {

        Header {
            shard_id,
            //parent_hash,
            chunk_root,
            period,
            proposer_address,
            //proposer_bid,
            //proposer_signature
        }
    }

    pub fn hash(&self) -> CollationHeaderHash {
        let mut sha3 = tiny_keccak::Keccak::new_sha3_256();

        // Add the shard id
        let sid = u256_to_bytes32(self.shard_id);
        sha3.update(&sid);

        // Add the parent hash
        /*
        let ph: &mut [u8; 32] = &mut [0; 32];
        self.parent_hash.copy_to(ph);
        sha3.update(ph);
        */

        // Add the chunk root
        let cr: &mut [u8; 32] = &mut [0; 32];
        self.chunk_root.copy_to(cr);
        sha3.update(cr);

        // Add the period
        let p = u256_to_bytes32(self.period);
        sha3.update(&p);

        // Add the proposer address
        let pa: &mut [u8; 20] = &mut [0; 20];
        self.proposer_address.copy_to(pa);
        sha3.update(pa);

        // Finalize hash and return as H256
        let mut result_bytes: [u8; 32] = [0; 32];
        sha3.finalize(&mut result_bytes);

        CollationHeaderHash::from_slice(&result_bytes[..])
    }

    pub fn create_sample_collation_header() -> Header {
        // Build the args for collation header creation
        let shard_id = ShardIdHash::from_dec_str("1").unwrap();
        // let parent_hash = ParentCollationHeaderHash::from_slice(&SAMPLE_COLLATION_PARENT_HASH_BYTES[..]);
        let chunk_root = ChunkRootHash::from_slice(&SAMPLE_COLLATION_CHUNK_ROOT_BYTES[..]);
        let period = ChunkPeriodHash::from_dec_str("1").unwrap();
        let proposer_address = ProposerAddress::from_slice(&SAMPLE_COLLATION_PROPOSER_ADDRESS_BYTES[..]);
        let header = Header::new(shard_id, /*parent_hash,*/ chunk_root, period, proposer_address);
        return header;
    }
}

// TODO - consider replacing with https://docs.rs/ethereum-types/0.3.1/ethereum_types/struct.U256.html#method.as_u32
// A crude way of converting the ethereum_types::U256 to a u8 byte array to be hashed.  Suggestions to improve this are desired.
fn u256_to_bytes32(u256: ethereum_types::U256) -> [u8; 32] {
    let mut bytes32: [u8; 32] = [0; 32];
    for i in 0..32 {
        bytes32[i] = u256.byte(i);
    }
    bytes32
}

pub fn create_sample_collation_header() -> Header {
    // Build the args for collation header creation
    let shard_id = ShardIdHash::from_dec_str("1").unwrap();
    // let parent_hash = ParentCollationHeaderHash::from_slice(&SAMPLE_COLLATION_PARENT_HASH_BYTES[..]);
    let chunk_root = ChunkRootHash::from_slice(&SAMPLE_COLLATION_CHUNK_ROOT_BYTES[..]);
    let period = ChunkPeriodHash::from_dec_str("1").unwrap();
    let proposer_address = ProposerAddress::from_slice(&SAMPLE_COLLATION_PROPOSER_ADDRESS_BYTES[..]);
    let header = Header::new(shard_id, /*parent_hash,*/ chunk_root, period, proposer_address);
    return header;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_produces_correct_hash() {
        let header = create_sample_collation_header();

        // Calculate its generated hash
        let header_hash: CollationHeaderHash = header.hash();
        let shard_id_bytes = u256_to_bytes32(header.shard_id);
        let period_bytes = u256_to_bytes32(header.period);

        // Calculate the expected hash
        let mut sha3 = tiny_keccak::Keccak::new_sha3_256();
        sha3.update(&shard_id_bytes[..]);
        //sha3.update(&SAMPLE_COLLATION_PARENT_HASH_BYTES[..]);
        sha3.update(&SAMPLE_COLLATION_CHUNK_ROOT_BYTES[..]);
        sha3.update(&period_bytes[..]);
        sha3.update(&SAMPLE_COLLATION_PROPOSER_ADDRESS_BYTES[..]);

        let mut expected_bytes: [u8; 32] = [0; 32];
        sha3.finalize(&mut expected_bytes);

        let expected: CollationHeaderHash = CollationHeaderHash::from_slice(&expected_bytes[..]);

        // Ensure manually calculated hash matches the generated hash
        assert_eq!(expected, header_hash);
    }

}
