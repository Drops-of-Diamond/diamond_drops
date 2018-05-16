//use ethereum_types;

//  Blobs and Chunks

pub const CHUNK_SIZE: usize = 32; // bytes
pub const CHUNK_DATA_SIZE: usize = CHUNK_SIZE - 1;  // size of chunk excluding the indicator byte
pub const COLLATION_SIZE: usize = 1048576; // 2^20 bytes, .pow() can't be used in a constant expression. 
pub const CHUNKS_PER_COLLATION: usize = COLLATION_SIZE / CHUNK_SIZE; // 2**15
/// size of a blob filling a full collation
pub const MAX_BLOB_SIZE: usize = CHUNKS_PER_COLLATION * CHUNK_DATA_SIZE;

/* can't make a constant a struct
pub const EMPTY_CHUNK: {u8, [u8; 31]} = Chunk::new(0x00, [0x00; CHUNK_DATA_SIZE]);
pub const EMPTY_CHUNKS_COLLATION_SIZE: vec<{u8, [u8; 31]> = vec![CHUNK_ZEROS; CHUNKS_PER_COLLATION]; */

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn collation_size_mod_chunk_size_is_zero() {
        assert_eq!(COLLATION_SIZE % CHUNK_SIZE, 0, "Test: COLLATION_SIZE {} % 
            CHUNK_SIZE {} == 0", COLLATION_SIZE, CHUNK_SIZE);
    }
}