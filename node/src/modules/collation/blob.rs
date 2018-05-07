use modules::collation::chunk;
use modules::constants::{CHUNK_SIZE, CHUNK_DATA_SIZE, 
    COLLATION_SIZE, CHUNKS_PER_COLLATION, MAX_BLOB_SIZE};

#[derive(PartialEq, Debug, Clone)]
pub struct Blob {
    size: usize,
    data: Vec<u8>
}

impl Blob {
    pub fn new(size: usize, data: Vec<u8>) -> Blob {
        Blob  {
            size,
            data
        }
    }

    /// Create a set of chunks to represent this blob
    pub fn to_chunks(self/*, skip_evm: bool*/) -> Vec<chunk::Chunk> {
        // We can only fit 31 (CHUNK_DATA_SIZE) out of 32 (CHUNK_SIZE) bytes
        // per blob into a chunk, due to adding the indicator byte.
        // self.size is the number of bytes per blob
        // Dimensional analysis:
        // chunks_per_blob = [bytes per blob] / [blob bytes that fit into each chunk].
        let chunks_per_blob: usize = (((self.size as f32 / CHUNK_DATA_SIZE as f32) as f32)
            .ceil()) as usize;
        let terminal_len: u8 = (self.size % CHUNK_DATA_SIZE) as u8;
        let mut chunks: Vec<chunk::Chunk> = vec![];
        for i in 1..chunks_per_blob {
            let mut ind: u8 = 0b0000_0000;
            let mut ch: chunk::Chunk;
            if i != chunks_per_blob { 
                // Build non-terminal chunks
                // ind = skip_evm when true, use a closure as noted above
                let i_data_start: usize = ((i - 1) * CHUNK_DATA_SIZE) as usize;
                let i_data_end: usize = (i * CHUNK_DATA_SIZE) as usize;
                let mut ch_data: [u8; CHUNK_DATA_SIZE] = [0; CHUNK_DATA_SIZE];
                for j in i_data_start..i_data_end {
                    ch_data[j - i_data_start] = self.data[j];
                }
                ch = chunk::Chunk::new(ind, ch_data);
                
            } else {
                // Build the terminal chunk
                // Set the first bit of the indicator byte, the skip_evm flag,
                // according to whether a skip_evm opcode was called
                // TODO once that is done.
                // Assume false until there are multiple EVMs.
                // The skip_evm opcode could change the value of skip_evm to true
                /*
                // Should also separate this into a separate function and use a closure
                // to avoid repeating the same code twice
                if skip_evm {
                    // Set SKIP_EVM flag to 1
                    indicator += 0b1000_0000;
                }
                */
                // Set the 5 least significant bits of the indicator byte
                ind += terminal_len;
                //ind = chunk::Chunk::build_indicator(skip_evm, true, terminal_len);
                // reduce repetition: refactor into a separate iterator function.
                let i_data_start: usize = (i * CHUNK_DATA_SIZE) as usize;
                let mut ch_data: [u8; CHUNK_DATA_SIZE] = [0; CHUNK_DATA_SIZE];
                for j in i_data_start..self.size {
                    ch_data[j - i_data_start] = self.data[j];
                }
                ch = chunk::Chunk::new(ind, ch_data);
            }
            chunks.push(ch);
        }
        chunks
    }

    /// Create a blob from a set of chunks
    pub fn from_chunks(chunks: Vec<chunk::Chunk>) -> Blob {
        let mut size: usize = 0;
        let mut data = vec![];
        for ch in chunks {
            let mask: u8 = 0b00011111;
            let length = &ch.indicator & mask;
            if length == 0 {
                // Chunk is not terminal, read all CHUNK_DATA_SIZE bytes into data
                for i in 0..CHUNK_DATA_SIZE {
                    data.push(ch.data[i as usize]);
                }
                size += CHUNK_DATA_SIZE
            } else {
                // Chunk is terminal, read length bytes into data
                for i in 0..length {
                    data.push(ch.data[i as usize]);
                }
                size += length as usize;
            }
        }

        Blob {
            size,
            data
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_converts_to_chunks() {
        let blob = Blob::new(128, vec![0xff; 128]);
        let blob_chunks = blob.to_chunks(/*true*/);
        let ccind = chunk::Chunk::build_indicator(false, false, 0);
        let term_ccind = chunk::Chunk::build_indicator(false, true, 4);
        let mut correct_blob_chunks = vec![chunk::Chunk::new(ccind, [0xff; CHUNK_DATA_SIZE]); 4];
        correct_blob_chunks.push(chunk::Chunk::new(term_ccind, [0xff, 0xff, 0xff, 0xff, 
                                                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                                                0, 0, 0, 0, 0, 0, 0]));
        assert_eq!(blob_chunks, correct_blob_chunks);
    }
/*
    #[test]
    fn it_converts_to_chunks() {
        let blob = Blob::new(128, vec![0xff; 128]);
        let blob_chunks = blob.to_chunks(/*true*/);
        let ccind = chunk::Chunk::build_indicator(false, false, 0);
        let term_ccind = chunk::Chunk::build_indicator(false, true, 4);
        let mut correct_blob_chunks = vec![chunk::Chunk::new(ccind, [0xff; CHUNK_DATA_SIZE]); 4];
        correct_blob_chunks.push(chunk::Chunk::new(term_ccind, [0xff, 0xff, 0xff, 0xff, 
                                                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                                                0, 0, 0, 0, 0, 0, 0]));
        assert_eq!(blob_chunks, correct_blob_chunks);
    }
*/
    #[test]
    fn it_converts_from_chunks() {
        let ind = chunk::Chunk::build_indicator(false, false, 0);
        let term_ind = chunk::Chunk::build_indicator(false, true, CHUNK_DATA_SIZE as u8);

        let mut chunks = vec![chunk::Chunk::new(ind, [255; CHUNK_DATA_SIZE]); 4];
        chunks.push(chunk::Chunk::new(term_ind, [255; CHUNK_DATA_SIZE]));

        let blob_from_chunks = Blob::from_chunks(chunks);
        let blob = Blob::new(155, vec![255; 155]);
        assert_eq!(blob, blob_from_chunks);
    }
}