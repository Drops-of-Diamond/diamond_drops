use ethereum_types;
use tiny_keccak;

#[derive(PartialEq, Debug, Clone)]
pub struct Header {
    shard_id: ethereum_types::U256,
    proposer_address: ethereum_types::Address,
    chunk_root: ethereum_types::H256,
    period: ethereum_types::U256,

    // The following fields are pending updates to the sharding spec and are currently ignored
    //parent_hash: ethereum_types::H256,
    //proposer_bid: ethereum_types::U256,
    //proposer_signature: ethereum_types::Signature
}

impl Header {
    pub fn new(shard_id: ethereum_types::U256,
               //parent_hash: ethereum_types::H256,
               chunk_root: ethereum_types::H256,
               period: ethereum_types::U256,
               proposer_address: ethereum_types::Address,
               //proposer_bid: ethereum_types::U256,
               /*proposer_signature: ethereum_types::Signature*/) -> Header {
        
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

    pub fn hash(&self) -> ethereum_types::H256 {
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

        ethereum_types::H256::from_slice(&result_bytes[..])
    }
}

// A crude way of converting the ethereum_types::U256 to a u8 byte array to be hashed.  Suggestions to improve this are desired. 
fn u256_to_bytes32(u256: ethereum_types::U256) -> [u8; 32] {
    let mut bytes32: [u8; 32] = [0; 32];
    for i in 0..32 {
        bytes32[i] = u256.byte(i);
    }
    bytes32
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_produces_correct_hash() {
        // Build the args for collation header creation
        // Shard Id
        let shard_id = ethereum_types::U256::from_dec_str("1").unwrap();
        let shard_id_bytes = u256_to_bytes32(shard_id);
        
        /*
        // Parent Hash
        let parent_hash_bytes: [u8; 32] = [0x50, 0xa1, 0xb3, 0xd5, 0x14, 0xd4, 0x99, 0x63,
                                  0x54, 0x14, 0x7a, 0xd2, 0x89, 0x61, 0x75, 0xb0, 
                                  0x7d, 0x43, 0x7f, 0x9e, 0x58, 0xfa, 0x3c, 0x44, 
                                  0x86, 0xc0, 0x42, 0xf4, 0xc3, 0xd5, 0x05, 0x9b];
        let parent_hash = ethereum_types::H256::from_slice(&parent_hash_bytes[..]);
        */

        // Chunk Root
        let chunk_root_bytes: [u8; 32] = [0x50, 0xce, 0xc0, 0x49, 0x54, 0x77, 0xfb, 0x7e,
                                          0x65, 0x25, 0xc2, 0xa0, 0x39, 0xa3, 0xa9, 0x95,
                                          0x34, 0x90, 0x35, 0xb2, 0xa8, 0x23, 0xa4, 0x99,
                                          0x0b, 0x27, 0xf6, 0xd7, 0xd5, 0x5e, 0xec, 0x6b];
        let chunk_root = ethereum_types::H256::from_slice(&chunk_root_bytes[..]);

        // Period
        let period = ethereum_types::U256::from_dec_str("1").unwrap();
        let period_bytes = u256_to_bytes32(period);

        // Proposer Address
        let proposer_address_bytes: [u8; 20] = [0x39, 0xa4, 0x2d, 0x47, 0x4a,
                                        0x52, 0x96, 0xab, 0x98, 0x52, 
                                        0x3b, 0x1a, 0x3d, 0xef, 0x8f, 
                                        0x18, 0x67, 0xad, 0x32, 0xb0];
        let proposer_address = ethereum_types::H160::from_slice(&proposer_address_bytes[..]);

        // Create the header
        let header = Header::new(shard_id, /*parent_hash,*/ chunk_root, period, proposer_address);

        // Calculate its generated hash
        let header_hash = header.hash();

        // Calculate the expected hash
        let mut sha3 = tiny_keccak::Keccak::new_sha3_256();
        sha3.update(&shard_id_bytes[..]);
        //sha3.update(&parent_hash_bytes[..]);
        sha3.update(&chunk_root_bytes[..]);
        sha3.update(&period_bytes[..]);
        sha3.update(&proposer_address_bytes[..]);

        let mut expected_bytes: [u8; 32] = [0; 32];
        sha3.finalize(&mut expected_bytes);

        let expected = ethereum_types::H256::from_slice(&expected_bytes[..]);

        // Ensure manually calculated hash matches the generated hash
        assert_eq!(expected, header_hash);
    }

}