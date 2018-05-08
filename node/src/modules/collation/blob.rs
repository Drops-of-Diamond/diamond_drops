// Defined according to https://ethresear.ch/t/blob-serialisation/1705.

use modules::collation::chunk;
use modules::constants::{CHUNK_SIZE, CHUNK_DATA_SIZE, 
    COLLATION_SIZE, CHUNKS_PER_COLLATION, MAX_BLOB_SIZE};

use std::convert::AsMut;
// use std::mem::size_of;

/// Struct of a blob containing data of arbitrary size.
#[derive(PartialEq, Debug, Clone)]
pub struct Blob {
    data: [u8]
}

pub impl Blob {
    pub fn new(data: [u8]) -> Blob {
        Blob  {
            data
        }
    }

    /// Convert a slice of an array to a fixed size array.
    /// From https://stackoverflow.com/a/37682288/7438857.
    pub fn clone_into_array<A, T>(slice: &[T]) -> A
        where A: Sized + Default + AsMut<[T]>,
            T: Clone
    {
        let mut a = Default::default();
        <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);
        a
    }

    /// Create a set of chunks to represent this blob.
    pub fn to_chunks(self, skip_evm: bool) -> Vec<chunk::Chunk> {
        // We can only fit 31 (CHUNK_DATA_SIZE) out of 32 (CHUNK_SIZE) bytes
        // per blob into a chunk, due to adding the indicator byte.
        // self.size is the number of bytes per blob
        // Dimensional analysis:
        // chunks_per_blob = [bytes per blob] / [blob bytes that fit into each chunk].
        // Since each element is one byte then getting the length (the number of elements)
        // will also get the number of bytes.
        let bytes_per_blob: usize = self.data.length()
        let chunks_per_blob: usize = (((bytes_per_blob as f32 / CHUNK_DATA_SIZE as f32) 
            as f32).ceil()) as usize;
        // E.g.: for a 125 byte blob at 31 bytes per chunk, that's 124 bytes that fit 
        // into 4 chunks, plus the last chunk for the last byte, i.e. 5 chunks. 
        // 125 / 31 rounded up is 5.
        let terminal_len: u8 = (bytes_per_blob % CHUNK_DATA_SIZE) as u8;
        // 125 % 31 = 1.
        // For collecting chunks:
        let mut chunks: Vec<chunk::Chunk> = vec![];
        // Iterate, collecting the blob into chunks
        for i in 1..chunks_per_blob {
            let mut indicator: u8;
            let mut ch: chunk::Chunk;
            // let i_data_start: usize = ((i - 1) * CHUNK_DATA_SIZE) as usize;
            let mut ch_data: [u8; CHUNK_DATA_SIZE] 
                = [0; CHUNK_DATA_SIZE]; 
            if i = chunks_per_blob {
                indicator += terminal_len;
            }
            ch_data = clone_into_array(&self.data[(i-1)
                *CHUNK_DATA_SIZE..(i*CHUNK_DATA_SIZE-1)]);
            // It doesn't seem necessary to have two if else blocks with repeated code    
            /*
            if i != chunks_per_blob { 
                // Build non-terminal chunks
                ch_data = clone_into_array(&self.data[(i-1)
                    *CHUNK_DATA_SIZE..(i*CHUNK_DATA_SIZE-1)]);
                    // 0..30 = (1-1)*31..(1*31-1),
                    // 31..61 = (2-1)*31..(2*31-1),
                    // 62..92 = (3-1)*31..(3*31-1), …  
                /* Not needed due to being replaced by the above:
                let i_data_end: usize = (i * CHUNK_DATA_SIZE) as usize;
                for j in i_data_start..i_data_end {
                    ch_data[j - i_data_start] = self.data[j];
                }
                */
            } else {
                // Build the terminal chunk
                // Set the 5 least significant bits of the indicator byte
                
                ch_data = clone_into_array(&self.data[(i-1)
                    *CHUNK_DATA_SIZE..(i*CHUNK_DATA_SIZE-1)]);
                /*
                for j in i_data_start..self.size {
                    ch_data[j - i_data_start] = self.data[j];
                }
                */
            }
            */
            // Set the first bit of the indicator byte, the skip_evm flag,
            // if a skip_evm opcode was called.
            // This will be false until there are multiple EVMs.
            // The skip_evm opcode could change the value of skip_evm to true
            // Should also separate this into a separate function and use a closure
            // to avoid repeating the same code twice
            if skip_evm {
                // Set SKIP_EVM flag to 1
                indicator += 0b1000_0000;
            }
            ch = chunk::Chunk::new(indicator, ch_data);
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

    #[test]
    fn it_converts_to_chunks() {
        let blob = Blob::new(128, vec![0xff; 128]);
        let blob_chunks = blob.to_chunks(true);
        let ccind = chunk::Chunk::build_indicator(true, false, 0);
        let term_ccind = chunk::Chunk::build_indicator(false, true, 4);
        let mut correct_blob_chunks = vec![chunk::Chunk::new(ccind, [0xff; CHUNK_DATA_SIZE]); 4];
        correct_blob_chunks.push(chunk::Chunk::new(term_ccind, [0xff, 0xff, 0xff, 0xff, 
                                                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                                                0, 0, 0, 0, 0, 0, 0]));
        assert_eq!(blob_chunks, correct_blob_chunks);
    }

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