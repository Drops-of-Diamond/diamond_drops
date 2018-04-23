use ethereum_types;

//  Blobs and Chunks

const CHUNK_SIZE: u32 = 32; // bytes
const CHUNK_DATA_SIZE: u32 = CHUNK_SIZE - 1;  // size of chunk excluding the indicator byte
const COLLATION_SIZE: u32 = 2.pow(20); // bytes
assert_eq!(COLLATION_SIZE % CHUNK_SIZE, 0);
const CHUNKS_PER_COLLATION: u16 = COLLATION_SIZE / CHUNK_SIZE as u16; // 2**15
// size of a blob filling a full collation
const MAX_BLOB_SIZE: u32 = CHUNKS_PER_COLLATION * CHUNK_DATA_SIZE;