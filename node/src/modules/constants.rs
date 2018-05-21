//use ethereum_types;

//  Blobs and Chunks

pub const CHUNK_SIZE: usize = 32; // bytes
pub const CHUNK_DATA_SIZE: usize = CHUNK_SIZE - 1;  // size of chunk excluding the indicator byte
pub const COLLATION_SIZE: usize = 1048576; // 2^20 bytes, .pow() can't be used in a constant expression. 
pub const CHUNKS_PER_COLLATION: usize = COLLATION_SIZE / CHUNK_SIZE; // 2**15
/// size of a blob filling a full collation
pub const MAX_BLOB_SIZE: usize = CHUNKS_PER_COLLATION * CHUNK_DATA_SIZE;

pub const SAMPLE_COLLATION_PARENT_HASH_BYTES: [u8; 32] = [0x50, 0xa1, 0xb3, 0xd5, 0x14, 0xd4, 0x99, 0x63,
                                                          0x54, 0x14, 0x7a, 0xd2, 0x89, 0x61, 0x75, 0xb0,
                                                          0x7d, 0x43, 0x7f, 0x9e, 0x58, 0xfa, 0x3c, 0x44,
                                                          0x86, 0xc0, 0x42, 0xf4, 0xc3, 0xd5, 0x05, 0x9b];

pub const SAMPLE_COLLATION_CHUNK_ROOT_BYTES: [u8; 32] = [0x50, 0xce, 0xc0, 0x49, 0x54, 0x77, 0xfb, 0x7e,
                                                         0x65, 0x25, 0xc2, 0xa0, 0x39, 0xa3, 0xa9, 0x95,
                                                         0x34, 0x90, 0x35, 0xb2, 0xa8, 0x23, 0xa4, 0x99,
                                                         0x0b, 0x27, 0xf6, 0xd7, 0xd5, 0x5e, 0xec, 0x6b];
pub const SAMPLE_COLLATION_PROPOSER_ADDRESS_BYTES: [u8; 20] = [0x39, 0xa4, 0x2d, 0x47, 0x4a,
                                                               0x52, 0x96, 0xab, 0x98, 0x52,
                                                               0x3b, 0x1a, 0x3d, 0xef, 0x8f,
                                                               0x18, 0x67, 0xad, 0x32, 0xb0];

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn collation_size_mod_chunk_size_is_zero() {
        assert_eq!(COLLATION_SIZE % CHUNK_SIZE, 0, "Test: COLLATION_SIZE {} % 
            CHUNK_SIZE {} == 0", COLLATION_SIZE, CHUNK_SIZE);
    }
}
