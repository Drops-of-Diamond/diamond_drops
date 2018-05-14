use bitreader::BitReader;
use modules::constants::{CHUNK_SIZE, CHUNK_DATA_SIZE, 
    /*COLLATION_SIZE, */CHUNKS_PER_COLLATION, MAX_BLOB_SIZE};
use modules::errors::*;
use ::std::fmt::{Binary, Formatter, Result};
use ::std::slice::SliceIndex;
//use modules::primitives::{BinaryU8};
// use modules::collation::blob::clone_into_array;

#[derive(PartialEq, Debug, Clone)]
pub struct Chunk {
    pub indicator: u8,
    pub data: [u8; CHUNK_DATA_SIZE]
}

impl Chunk {
    pub fn new(indicator: u8, data: [u8; CHUNK_DATA_SIZE]) -> Chunk {
        Chunk {
            indicator,
            data
        }
    }

    /*
    /// Build the indicator byte with the supplied data.  Length can be
    /// any value if the chunk is not terminal (the value is ignored).
    /// Only used in tests since skip_evm doesn't need to be used now.
    pub fn build_indicator(skip_evm: bool/*, terminal: bool, length: u8*/) -> u8{
        let mut indicator: u8 = 0;

        if skip_evm {
            // Set SKIP_EVM flag to 1
            indicator += 0b1000_0000;
        }
        // Check if it's a terminal chunk and if so, set the length bits.
        if 0 < self.data.len() && self.data.len() < CHUNK_SIZE {
            indicator += self.data.len();
        }
        indicator
    }
    */
    /// Convert the Chunk into bytes
    pub fn chunk_to_bytes(self) -> [u8; CHUNK_SIZE] {
        let mut bytes: [u8; CHUNK_SIZE] = [0; CHUNK_SIZE];
        bytes[0] = self.indicator;
        for i in 1..CHUNK_SIZE {
            bytes[i] = self.data[i-1];
        }
        bytes
    }

    /// Convert CHUNK_SIZE bytes into a chunk
    pub fn bytes_to_chunk(chunk_bytes: [u8; CHUNK_SIZE]) -> Chunk {
        let indicator = chunk_bytes[0];
        let mut data: [u8; 31] = [0; 31];
        /*
        let mut chunk = Chunk {
            indicator: chunk_bytes[0],
            data: [0x00; CHUNK_DATA_SIZE]
        };
        */
        /*
        let indicator_ref = &[chunk_bytes[0]];
        let mut indicator_reader = BitReader::new(indicator_ref);

        let skip_evm = indicator_reader.read_u8(1)?;//.chain_err(|| "Failed to read the 
            // first three bits of the indicator");
        */
        for i in 0..CHUNK_DATA_SIZE {
            data[i] = chunk_bytes[i+1];
        }
        Chunk {
            indicator,
            data
        }
    }
    /// Build an indicator byte with the supplied parameters:
    /// `skip_evm`, `terminal` and `terminal_length` .  Length can be
    /// any value if the chunk is not terminal (the value is ignored).
    /// Only used in tests.
    /// This does not 
    pub fn build_indicator(skip_evm: bool, terminal: bool, terminal_length: u8) -> u8{
        let mut indicator: u8 = 0b0000_0000;
        if skip_evm {
            // Set SKIP_EVM flag to 1
            indicator += 0b1000_0000;
        }
        /* Could use this if we want to build an indicator byte from a chunk,
        then we wouldn't need terminal and terminal_length.
        // Check if it's a terminal chunk and if so, set the length bits.
        if 0 < self.data.len() && self.data.len() < CHUNK_SIZE {
            indicator += self.data.len();
        }
        */
        if terminal {
            assert!(0 < terminal_length && terminal_length < CHUNK_SIZE as u8);
            indicator += terminal_length;
        }
        indicator
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn it_builds_indicator() {
        let full_indicator: u8 = Chunk::build_indicator(true, true, CHUNK_DATA_SIZE as u8);
        let correct_full_indicator: u8 = 0b1001_1111;
        assert_eq!(full_indicator, correct_full_indicator);

        let full_non_terminal_indicator: u8 = Chunk::build_indicator(true, false, 1);
        let correct_full_non_terminal_indicator: u8 = 0b1000_0000;
        assert_eq!(full_non_terminal_indicator, correct_full_non_terminal_indicator);

        let one_bit_length_indicator_skip: u8 = Chunk::build_indicator(true, true, 1);
        let correct_one_bit_length_indicator_skip: u8 = 0b1000_0001;
        assert_eq!(one_bit_length_indicator_skip, correct_one_bit_length_indicator_skip);

        let run_evm_indicator: u8 = Chunk::build_indicator(false, true, 16);
        let correct_run_evm_indicator: u8 = 0b0001_0000;
        assert_eq!(run_evm_indicator, correct_run_evm_indicator);
    }

    #[test]
    fn it_converts_to_bytes() {
        let chunk = Chunk::new(0b1000_0000, [1; CHUNK_DATA_SIZE as usize]);
        let chunk_bytes = chunk.chunk_to_bytes();
        let correct_chunk_bytes: [u8; CHUNK_SIZE] = [0b1000_0000, 1, 
                                            1, 1, 1, 1, 1, 1,
                                            1, 1, 1, 1, 1, 1, 
                                            1, 1, 1, 1, 1, 1,
                                            1, 1, 1, 1, 1, 1,
                                            1, 1, 1, 1, 1, 1];

        assert_eq!(chunk_bytes, correct_chunk_bytes);
    }

    #[test]
    fn it_converts_from_bytes() {
        let chunk = Chunk::new(0b1000_0000, [1; CHUNK_DATA_SIZE]);
        let chunk_bytes = chunk.clone().chunk_to_bytes();
        let same_chunk = Chunk::bytes_to_chunk(chunk_bytes);
        assert_eq!(chunk, same_chunk);
    }

}

/* Commented out because there are errors e.g. on slices
#[derive(PartialEq, Copy, Debug, Clone)]
pub struct Binaryu8(u8);

impl Binary for Binaryu8 {
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        let val = self.0;

        write!(f, "{:b}", val) // delegate to i32's implementation
    }
}

impl From<u8> for Binaryu8 {
    fn from(val: u8) -> Binaryu8 {
        Binaryu8(val)
    }
}

impl<'a> SliceIndex<u8> for Binaryu8 {
    type Output = [u8];

    #[inline]
    fn get(self, slice: &u8) -> Option<&Self::Output> {
        Binaryu8(*slice)
    }

    #[inline]
    fn get_mut(self, slice: &mut u8) -> Option<&mut Self::Output> {
        Binaryu8(slice)
    }

    #[inline]
    unsafe fn get_unchecked(self, slice: &u8) -> &Self::Output {
        Binaryu8(slice)
    }

    #[inline]
    unsafe fn get_unchecked_mut(self, slice: &mut u8) -> &mut Self::Output {
        Binaryu8(slice)
    }

    #[inline]
    fn index(self, slice: &u8) -> &Self::Output {
        Binaryu8(slice)
    }

    #[inline]
    fn index_mut(self, slice: &mut u8) -> &mut Self::Output {
        Binaryu8(slice)
    }
}
*/


/*
impl<'a> SliceIndex<u8> for Binaryu8 {
    type Output = [u8];

    #[inline]
    fn get(self, slice: &u8) -> Option<&Self::Output> {
        Binaryu8(slice)
    }

    #[inline]
    fn get_mut(self, slice: &mut u8) -> Option<&mut Self::Output> {
        Binaryu8(slice)
    }

    #[inline]
    unsafe fn get_unchecked(self, slice: u8) -> &'a Self::Output {
        Binaryu8(slice)
    }

    #[inline]
    unsafe fn get_unchecked_mut(self, slice: &mut u8) -> &'a mut Self::Output {
        Binaryu8(slice)
    }

    #[inline]
    fn index(self, slice: u8) -> &'a Self::Output {
        Binaryu8(slice)
    }

    #[inline]
    fn index_mut(self, slice: &mut u8) -> &'a mut Self::Output {
        Binaryu8(slice)
    }
}
*/

/*
impl SliceIndex<[u8]> for Binaryu8 {
    type Output = [u8];

    #[inline]
    fn get(self, slice: &[u8]) -> Option<&Self::Output> {
        Binaryu8(slice)
    }

    #[inline]
    fn get_mut(self, slice: &mut [u8]) -> Option<&mut Self::Output> {
        Binaryu8(slice)
    }

    #[inline]
    unsafe fn get_unchecked(self, slice: &[u8]) -> &Self::Output {
        Binaryu8(slice)
    }

    #[inline]
    unsafe fn get_unchecked_mut(self, slice: &mut [u8]) -> &mut Self::Output {
        Binaryu8(slice)
    }

    #[inline]
    fn index(self, slice: &[u8]) -> &Self::Output {
        Binaryu8(slice)
    }

    #[inline]
    fn index_mut(self, slice: &mut [u8]) -> &mut Self::Output {
        Binaryu8(slice)
    }
}
*/