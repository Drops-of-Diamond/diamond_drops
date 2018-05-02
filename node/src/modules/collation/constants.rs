use ethereum_types;

//  Blobs and Chunks

pub const CHUNK_SIZE: u32 = 32; // bytes
pub const CHUNK_DATA_SIZE: u32 = CHUNK_SIZE - 1;  // size of chunk excluding the indicator byte