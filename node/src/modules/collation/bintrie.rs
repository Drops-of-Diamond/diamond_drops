use std::collections::HashMap;

use tiny_keccak;
use ethereum_types;

/// A binary Merkle trie for storing blobs based off of Vitalik's implementation at:
/// https://github.com/ethereum/research/blob/master/trie_research/bintrie2/new_bintrie.py
pub struct BinTrie {
    db: HashMap<ethereum_types::H256, ethereum_types::H256>
}

impl BinTrie {
    pub fn new() -> BinTrie {
        let mut db: HashMap<ethereum_types::H256, ethereum_types::H256> = HashMap::new();
        let mut h: ethereum_types::H256 = ethereum_types::H256::zero();

        for i in 0..256 {
            // Concatenate previous two h hashes together into one byte array
            let h1 = h256_to_bytes32(h);
            let h2 = h256_to_bytes32(h);
            let concat_h_bytes = [h1, h2].concat();
            let concat_h = ethereum_types::H256::from_slice(&concat_h_bytes[..]);

            let newh = easy_sha3(&concat_h);
            db.insert(newh, concat_h);
            h = newh;
        }

        BinTrie {
            db
        }
    }
}

// Function to deal with sha3 functions with only one value to hash
fn easy_sha3(value: &[u8]) -> ethereum_types::H256 {
    let mut sha3 = tiny_keccak::Keccak::new_sha3_256();
    let mut hash_bytes: [u8; 32] = [0; 32];
    sha3.update(value);
    sha3.finalize(&mut hash_bytes);
    ethereum_types::H256::from_slice(&hash_bytes[..])
}

// Function to convert H256 to [u8; 32]
fn h256_to_bytes32(h256: ethereum_types::H256) -> [u8; 32] {
    let mut bytes32: [u8; 32] = [0; 32];
    for i in 0..32 {
        bytes32[i] = h256.byte(i);
    }
    bytes32
}