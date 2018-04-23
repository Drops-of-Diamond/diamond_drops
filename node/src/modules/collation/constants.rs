use ethereum_types;

//  Blobs and Chunks

const CHUNK_SIZE: u32 = 32; // bytes
const CHUNK_DATA_SIZE: u32 = CHUNK_SIZE - 1;  // size of chunk excluding the indicator byte
const COLLATION_SIZE: u32 = 2.pow(20); // bytes
const CHUNKS_PER_COLLATION: u16 = COLLATION_SIZE / CHUNK_SIZE as u16; // 2**15
// size of a blob filling a full collation
const MAX_BLOB_SIZE: u32 = CHUNKS_PER_COLLATION * CHUNK_DATA_SIZE;

#[cfg(test)]
mod tests {

    #[test]
    fn collation_size_mod_chunk_size_is_zero() {
        assert_eq!(COLLATION_SIZE % CHUNK_SIZE, 0, "Test: COLLATION_SIZE {} % 
            CHUNK_SIZE {} == 0", COLLATION_SIZE, CHUNK_SIZE);
    }
}