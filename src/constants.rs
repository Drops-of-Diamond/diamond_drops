use ethereum_types;

//  Blobs and Chunks
 
const CHUNK_SIZE: i32 = 32
const CHUNK_DATA_SIZE: i32 = CHUNK_SIZE - 1  # size of chunk excluding the indicator byte
const COLLATION_SIZE: ethereum_types::U256 = ethereum_types::U256.pow(2, 20) // bytes
const assert!(COLLATION_SIZE % CHUNK_SIZE == 0)
const CHUNKS_PER_COLLATION = COLLATION_SIZE / CHUNK_SIZE // 2**15
// size of a blob filling a full collation
const MAX_BLOB_SIZE: ethereum_types::U256 = CHUNKS_PER_COLLATION * CHUNK_DATA_SIZE