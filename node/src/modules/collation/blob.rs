use modules::collation::chunk;

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
    pub fn to_chunks(self, skip_evm: bool) -> Vec<chunk::Chunk> {
        let num_chunks: usize = (self.size - 1) / 31 + 1;
        let terminal_len: u8 = (self.size % 31) as u8;
        let mut chunks: Vec<chunk::Chunk> = vec![];
        for i in 0..num_chunks {
            let mut ind: u8;
            let mut ch: chunk::Chunk;
            if i == num_chunks - 1 {
                ind = chunk::Chunk::build_indicator(skip_evm, true, terminal_len);
                let i_data_start: usize = (i * 31) as usize;
                let mut ch_data: [u8; 31] = [0; 31];
                for j in i_data_start..self.size {
                    ch_data[j - i_data_start] = self.data[j];
                }
                ch = chunk::Chunk::new(ind, ch_data);
            } else {

                ind = chunk::Chunk::build_indicator(skip_evm, false, 0);
                let i_data_start: usize = (i * 31) as usize;
                let i_data_end: usize = ((i + 1) * 31) as usize;
                let mut ch_data: [u8; 31] = [0; 31];
                for j in i_data_start..i_data_end {
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
                // Chunk is not terminal, read all 31 bytes into data
                for i in 0..31 {
                    data.push(ch.data[i as usize]);
                }
                size += 31
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
        let blob = Blob::new(128, vec![255; 128]);
        let blob_chunks = blob.to_chunks(true);
        let ccind = chunk::Chunk::build_indicator(true, false, 0);
        let term_ccind = chunk::Chunk::build_indicator(true, true, 4);
        let mut correct_blob_chunks = vec![chunk::Chunk::new(ccind, [255; 31]); 4];
        correct_blob_chunks.push(chunk::Chunk::new(term_ccind, [255, 255, 255, 255, 
                                                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                                                0, 0, 0, 0, 0, 0, 0]));
        assert_eq!(blob_chunks, correct_blob_chunks);
    }

    #[test]
    fn it_converts_from_chunks() {
        let ind = chunk::Chunk::build_indicator(true, false, 0);
        let term_ind = chunk::Chunk::build_indicator(true, true, 31);

        let mut chunks = vec![chunk::Chunk::new(ind, [255; 31]); 4];
        chunks.push(chunk::Chunk::new(term_ind, [255; 31]));

        let blob_from_chunks = Blob::from_chunks(chunks);
        let blob = Blob::new(155, vec![255; 155]);
        assert_eq!(blob, blob_from_chunks);
    }
}