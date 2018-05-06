#[derive(PartialEq, Debug, Clone)]
pub struct Chunk {
    pub indicator: u8,
    pub data: [u8; 31]
}

impl Chunk {
    pub fn new(indicator: u8, data: [u8; 31]) -> Chunk {
        Chunk {
            indicator,
            data
        }
    }

    /// Build the indicator byte with the supplied data.  Length can be
    /// any value if the chunk is not terminal (the value is ignored)
    pub fn build_indicator(skip_evm: bool, terminal: bool, length: u8) -> u8{
        let mut indicator: u8 = 0;

        if skip_evm {
            // Set SKIP_EVM flag to 1
            indicator += 0b1000_0000;
        }
        if terminal {
            assert!(0 < length && length < 32);
            indicator += length;
        }
        indicator
    }

    /// Convert the Chunk into bytes
    pub fn as_bytes(self) -> [u8; 32] {
        let mut bytes: [u8; 32] = [0; 32];
        bytes[0] = self.indicator;
        for i in 1..32 {
            bytes[i] = self.data[i-1];
        }
        bytes
    }

    /// Convert 32 bytes into a chunk
    pub fn from_bytes(chunk_bytes: [u8; 32]) -> Chunk {
        let indicator = chunk_bytes[0];
        let mut data: [u8; 31] = [0; 31];
        for i in 1..32 {
            data[i-1] = chunk_bytes[i];
        }
        Chunk {
            indicator,
            data
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_builds_indicator() {
        let full_ind: u8 = Chunk::build_indicator(true, true, 31);
        let correct_full_ind: u8 = 0b1001_1111;
        assert_eq!(full_ind, correct_full_ind);
        let full_non_terminal_ind: u8 = Chunk::build_indicator(true, false, 1);
        let correct_full_non_terminal_ind: u8 = 0b1000_0000;
        assert_eq!(full_non_terminal_ind, correct_full_non_terminal_ind);
        let two_bit_ind: u8 = Chunk::build_indicator(true, true, 1);
        let correct_two_bit_ind: u8 = 0b1000_0001;
        assert_eq!(two_bit_ind, correct_two_bit_ind);
        let run_evm_ind: u8 = Chunk::build_indicator(false, true, 16);
        let correct_run_evm_ind: u8 = 0b0001_0000;;
        assert_eq!(run_evm_ind, correct_run_evm_ind);
    }

    #[test]
    fn it_converts_to_bytes() {
        let chunk = Chunk::new(0b1000_0000, [1; 31]);
        let chunk_bytes = chunk.as_bytes();
        let correct_chunk_bytes: [u8; 32] = [0b1000_0000, 1, 1, 1, 
                                            1, 1, 1, 1, 1, 
                                            1, 1, 1, 1, 1,
                                            1, 1, 1, 1, 1, 1,
                                            1, 1, 1, 1, 1, 1,
                                            1, 1, 1, 1, 1, 1];

        assert_eq!(chunk_bytes, correct_chunk_bytes);
    }

    #[test]
    fn it_converts_from_bytes() {
        let chunk = Chunk::new(0b1000_0000, [1; 31]);
        let chunk_bytes = chunk.clone().as_bytes();
        let same_chunk = Chunk::from_bytes(chunk_bytes);
        assert_eq!(chunk, same_chunk);
    }

}
