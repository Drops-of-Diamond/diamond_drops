use std::collections::HashMap;

use tiny_keccak;
use ethereum_types;

/// A binary Merkle trie for storing blobs based off of Vitalik's implementation at:
/// https://github.com/ethereum/research/blob/master/trie_research/bintrie2/new_bintrie.py
pub struct BinTrie {
    db: HashMap<ethereum_types::H256, [u8; 64]>,
    zerohashes: Vec<[u8; 32]>
}

impl BinTrie {
    pub fn new() -> BinTrie {
        let mut db: HashMap<ethereum_types::H256, [u8; 64]> = HashMap::new();
        let mut h: ethereum_types::H256 = ethereum_types::H256::zero();

        for i in 0..256 {
            // Concatenate h with itself a.k.a h + h
            let mut h1: [u8; 32] = [0; 32];
            let mut h2: [u8; 32] = [0; 32];
            
            h.copy_to(&mut h1);
            h.copy_to(&mut h2);

            let mut hplush: [u8; 64] = [0; 64];
            for i in 0..32 {
                hplush[i] = h1[i];
                hplush[i + 32] = h2[i];
            }
            
            let newh = easy_sha3(&hplush);
            
            db.insert(newh, hplush);
            h = newh;
        }

        // initialize zerohashes
        let mut zerohashes: Vec<[u8; 32]> = vec![[0; 32]];

        for i in 0..255 {
            // zerohashes[0] + zerohashes[0]
            let mut z2: [u8; 64] = [0; 64];
            for j in 0..32 {
                z2[j] = zerohashes[0][j];
                z2[j + 32] = zerohashes[0][j];
            }
            let mut new_zerohash: [u8; 32] = [0; 32];
            easy_sha3(&z2).copy_to(&mut new_zerohash);
            zerohashes.insert(0, new_zerohash);
        }

        BinTrie {
            db,
            zerohashes
        }
    }

    /// Get a value from the trie
    pub fn get(&mut self, root: ethereum_types::H256, key: ethereum_types::H256) 
        -> [u8; 32] {
        let mut v = root;
        let mut path = ethereum_types::U256::from(key);
        for _ in 0..256 {
            if ((path >> 255) & ethereum_types::U256::from_dec_str("1").unwrap()) == 
                ethereum_types::U256::from_dec_str("1").unwrap() {
                v = ethereum_types::H256::from_slice(&self.db.get(&v).unwrap()[32..]);
            } else {
                v = ethereum_types::H256::from_slice(&self.db.get(&v).unwrap()[..32]);
            }
            path = path << 1;
        }
        let mut v_bytes: [u8; 32] = [0; 32];
        v.copy_to(&mut v_bytes);
        v_bytes
    }

    /// Update a value in the trie
    pub fn update(&mut self, root: ethereum_types::H256, 
                             key: ethereum_types::H256, 
                             value: [u8; 32]) {
        let mut v = root;
        let mut entry: [u8; 64] = [0; 64];
        let mut path = ethereum_types::U256::from(key);
        let mut path2 = ethereum_types::U256::from(key);
        let mut sidenodes = vec![];
        let mut insert: [u8; 64] = [0; 64];
        
        for i in 0..256 {
            entry = *self.db.get_mut(&v).unwrap();
            if ((path >> 255) & ethereum_types::U256::from_dec_str("1").unwrap()) == 
                ethereum_types::U256::from_dec_str("1").unwrap() {
                let mut sn: [u8; 32] = [0; 32];
                for j in 0..32 {
                    sn[j] = entry[j];
                }
                sidenodes.push(sn);
                v = ethereum_types::H256::from_slice(&entry[32..]);
                if i == 255 {
                    for j in 0..32 {
                        insert[j] = entry[j];
                        insert[j + 32] = value[j];
                    }
                }
            } else {
                let mut sn: [u8; 32] = [0; 32];
                for j in 0..32 {
                    sn[j] = entry[j + 32];
                }
                sidenodes.push(sn);
                v = ethereum_types::H256::from_slice(&entry[..32]);
                if i == 255 {
                    for j in 0..32 {
                        insert[j] = value[j];
                        insert[j + 32] = entry[j + 32];
                    }
                }
            }
            path = path << 1;
        }
        &self.db.entry(v).or_insert(insert);
        for i in 0..256 {
            let mut newv = ethereum_types::H256::zero();
            // sidenodes[-1]
            let snl = sidenodes.pop().unwrap();

            if (path2 & ethereum_types::U256::from_dec_str("1").unwrap()) ==
                ethereum_types::U256::from_dec_str("1").unwrap() {
                // sidenodes[-1] + v
                let mut snv: [u8; 64] = [0; 64];
                for j in 0..32 {
                    snv[j] = snl[j];
                    snv[j + 32] = v[j];
                }
                let newv = easy_sha3(&snv);
                self.db.insert(newv, snv);
            } else {
                // v + sidenodes[-1]
                let mut vsn: [u8; 64] = [0; 64];
                for j in 0..32 {
                    vsn[j] = v[j];
                    vsn[j + 32] = snl[j];
                }
                let newv = easy_sha3(&vsn);
                self.db.insert(newv, vsn);
            }
            path2 = path2 >> 1;
            v = newv;
        }
    }

    /// Make a Merkle proof of the key
    pub fn make_merkle_proof(&mut self, root: ethereum_types::H256, key: ethereum_types::H256) 
        -> Vec<[u8; 32]> {
        let mut v = root;
        let mut path = ethereum_types::U256::from(key);
        let mut sidenodes = vec![];
        let mut entry: [u8; 64] = [0; 64];
        for i in 0..256 {
            entry = *self.db.get(&v).unwrap();
            let mut sn: [u8; 32] = [0; 32];
            if (path >> 255) & ethereum_types::U256::from_dec_str("1").unwrap() == 
                ethereum_types::U256::from_dec_str("1").unwrap() {
                for j in 0..32 {
                    sn[j] = entry[j];
                }
                sidenodes.push(sn);
                v = ethereum_types::H256::from_slice(&entry[32..]);
            } else {
                for j in 0..32 {
                    sn[j] = entry[j + 32];
                }
                sidenodes.push(sn);
                v = ethereum_types::H256::from_slice(&entry[..32]);
            }
            path = path << 1;
        }
        sidenodes
    }

    /// Verify a merkle proof
    pub fn verify_proof(&mut self, 
                        proof: Vec<[u8; 32]>, 
                        root: ethereum_types::H256, 
                        key: ethereum_types::H256,
                        value: [u8; 32]) -> bool {
        let mut path = ethereum_types::U256::from(key);
        let mut v = ethereum_types::H256::from(value);
        for i in 0..256 {
            let mut newv = ethereum_types::H256::zero();
            // proof[-1-i], or proof[length - i]
            let proof_entry = proof[256 - i];

            if (path & ethereum_types::U256::from_dec_str("1").unwrap()) == 
                ethereum_types::U256::from_dec_str("1").unwrap() {
                // proof[-1-i] + v
                let mut pv: [u8; 64] = [0; 64];
                for j in 0..32 {
                    pv[j] = proof_entry[j];
                    pv[j + 32] = v[j];
                }
                newv = easy_sha3(&pv);
            } else {
                // v + proof[-1-i] + v
                let mut vp: [u8; 64] = [0; 64];
                for j in 0..32 {
                    vp[j] = v[j];
                    vp[j + 32] = proof_entry[j];
                }
                newv = easy_sha3(&vp);
            }
            path = path >> 1;
            v = newv;
        }
        root == v
    }

    /// Compress a Merkle proof
    pub fn compress_proof(&self, proof: Vec<[u8; 32]>) -> Vec<u8> {
        let mut bits: [u8; 32] = [0; 32];
        let mut oproof: Vec<[u8; 32]> = vec![];
        for (i, p) in proof.iter().enumerate() {
            if *p == self.zerohashes[i] {
                bits[i / 8] = bits[i / 8] ^ (1 << i ^ 8);
            } else {
                oproof.push(*p);
            }
        }
        let mut compressed_proof: Vec<u8> = vec![];
        for i in 0..32 {
            compressed_proof.push(bits[i]);
        }
        for i in 0..oproof.len() {
            for j in 0..32 {
                compressed_proof.push(oproof[i][j]);
            }
        }
        compressed_proof
    }

    /// Decompress a Merkle proof
    pub fn decompress_proof(&self, oproof: Vec<u8>) -> Vec<[u8; 32]> {
        let mut proof: Vec<[u8; 32]> = vec![];
        let mut bits: [u8; 32] = [0; 32];
        for i in 0..32 {
            bits[i] = oproof[i];
        }
        let mut pos = 32;
        for i in 0..256 {
            if (bits[i / 8] & (1 << (i % 8))) == 1 {
                proof.push(self.zerohashes[i]);
            } else {
                let mut proof_elem: [u8; 32] = [0; 32];
                for j in 0..32 {
                    proof_elem[j] = oproof[j + pos];
                }
                proof.push(proof_elem);
                pos = pos + 32;
            }
        }
        proof
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn it_gets_entries() {
        panic!();
    }

    #[test]
    #[ignore]
    fn it_updates_entries() {
        panic!();
    }

    #[test]
    #[ignore]
    fn it_makes_merkle_proofs() {
        panic!();
    }

    #[test]
    #[ignore]
    fn it_verifies_merkle_proofs() {
        panic!();
    }

    #[test]
    #[ignore]
    fn it_compresses_merkle_proofs() {
        panic!();
    }

    #[test]
    #[ignore]
    fn it_decompresses_merkle_proofs() {
        panic!();
    }
}