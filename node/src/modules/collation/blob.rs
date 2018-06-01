// There are a lot of assert_eq!(...) statements for debugging purposes,
// since I haven't linked this file to src/bin.rs, and thus it can't be
// stepped through in debugging.
// Stuff that I have tried but didn't complete with getting to work
// is also commented out.

// use bitreader::BitReader;
use modules::collation::chunk::Chunk;
use modules::constants::{CHUNK_SIZE, CHUNK_DATA_SIZE,
    COLLATION_SIZE, CHUNKS_PER_COLLATION, MAX_BLOB_SIZE, DATA_BYTES_PER_COLLATION};
use modules::collation::body::{Body/* , BlobBodies */};
use modules::collation::header::Header;
use modules::primitives::CollationHeaderHash;
use modules::errors::*;
use modules::collation::body;
use std::process;

// use std::ops::Try; // for trying to use with structs for error-handling,
// however this is a nightly feature.
// use std::convert::AsMut;

/// Struct of a blob containing data of arbitrary size. This and the Chunks struct are
/// defined according to https://ethresear.ch/t/blob-serialisation/1705. Provides a more efficient
/// form of serialization than RLP, reducing load on disk reads and writes, which is the major
/// bottleneck for Ethereum 1.0. For more details on advantages, see the above link, particularly
/// [this comment](https://ethresear.ch/t/blob-serialisation/1705/5).
#[derive(PartialEq, Debug, Clone)]
pub struct Blob {
    pub data: Vec<u8>
}

impl Blob {
    pub fn new(data: Vec<u8>) -> Blob {
        Blob  {
            data
        }

    }
    /// Create a set of chunks to represent this blob.
    pub fn to_chunks(self, skip_evm: bool) -> Vec<Chunk>/* Result<Vec<Chunk>> */ {
        // Since each element is one byte then getting the length (the number of elements)
        // will also get the number of bytes.
        let bytes_per_blob: usize = self.data.len();
        // We can only fit 31 (CHUNK_DATA_SIZE) out of 32 (CHUNK_SIZE) bytes
        // per blob into a chunk, due to adding the indicator byte.
        // self.size is the number of bytes per blob
        // Dimensional analysis:
        // chunks_per_blob = [bytes per blob] / [blob bytes that fit into each chunk].
        let chunks_per_blob: usize = (((bytes_per_blob as f32 / CHUNK_DATA_SIZE as f32)
            as f32).ceil()) as usize;
        // E.g.: for a 125 byte blob at 31 bytes per chunk, that's 124 bytes that fit
        // into 4 chunks, plus the last chunk for the last byte, i.e. 5 chunks.
        // 125 / 31 rounded up is 5.

        // This won't work when we have a blob with 0 value bytes on the end of it,
        // since those will be included in the length of bytes_per_blob.
        // let terminal_len: u8 = (bytes_per_blob % CHUNK_DATA_SIZE) as u8;
        // 125 % 31 = 1.

        // Assume initially that the blob has no consecutive final zeros (i.e. not [..., 0, 0, 0])
        // However, this has the adverse side effect of when we actually want all the data to be zero in the last blob,
        // the indicator will be set to 0.
        let mut data_length_last_31_bytes: u8
            = CHUNK_DATA_SIZE as u8;
        //let blob_data: &mut Vec<u8> = &mut self.data;
        //let blob_data = self.data;
        //let blob_length = self.data.len()?;
        //assert_eq!{blob_length, 100, "blob_length: {:?}", blob_length}
        if !(0 < data_length_last_31_bytes
            && data_length_last_31_bytes
            <= CHUNK_DATA_SIZE as u8) {
                error!("{:?} is not more than 0 bytes and less than or equal to {:?}",
                data_length_last_31_bytes,
                CHUNK_DATA_SIZE)
            }
        // For collecting chunks:
        let mut chunks: Vec<Chunk> = vec![];
        // Iterate, collecting the blob into chunks
        // Remember, for loops don't iterate the last step in a range!
        // So start from 0 to add 1 (and to avoid doing operations for performance)
        for i in 0..chunks_per_blob {
            // 0, 1, 2, ... chunks_per_blob - 1
            let mut indicator: u8 = 0b0000_0000;
            let mut ch: Chunk;
            let i_data_start: usize = (i * CHUNK_DATA_SIZE) as usize;
            // 0, 31, 62, ... chunks_per_blob * CHUNK_DATA_SIZE
            let mut chunk_data: [u8; CHUNK_DATA_SIZE] = [0; CHUNK_DATA_SIZE];
            if i != chunks_per_blob - 1 {
                // Build non-terminal chunks
                // i_data_end_plus_1 = i_data_start_of_next_31_bytes
                let i_data_end_plus_1: usize = ((i + 1) * CHUNK_DATA_SIZE) as usize;
                // 31, 62, 93, ..., (chunks_per_blob + 1) * CHUNK_DATA_SIZE
                // Again, it doesn't do the last step.
                for j in i_data_start..(i_data_end_plus_1) {
                    // j = (loop 1) 0..30, (loop 2), 31..61,
                    // 62..92, ..., (chunks_per_blob - 2)*CHUNK_DATA_SIZE
                    // .. (chunks_per_blob - 1) * CHUNK_DATA_SIZE - 1
                    // chunks_per_blob won't run in this if loop, it will go up to
                    // chunks_per_blob - 1, which will then not go into the block.
                    chunk_data[j - i_data_start] = self.data[j];
                }
            } else {
                // Build the terminal chunk

                // Set the chunk data, saving iterating the last 0 consecutive bits of
                // the terminal chunk, which are set to 0 anyway above.
                for j in i_data_start..bytes_per_blob {
                    chunk_data[j - i_data_start] = self.data[j];
                }

                //let last_31_bytes_in_blob_in_reverse = blob_data[blob_length-31..].reverse();
                for i in (0..CHUNK_DATA_SIZE).rev() {
                    //assert_eq!{self.data[i], 100, "self.data: {:?}\nself.data[i]: {:?}\ni: {:?}", self.data, self.data[i], i}
                    //assert_eq!{i, 100, "i: {:?}", i}
                    if chunk_data[i] == 0 {
                        data_length_last_31_bytes -= 1;
                        /*
                        assert_eq!(data_length_last_31_bytes,
                            3.14,
                            "\n\ndata_length_last_31_bytes: {:?}\ni:{:?}\nself.data[i]:{:?}\n\n",
                            data_length_last_31_bytes, i, self.data[i]);
                        */
                    } else {
                        break;
                    }
                }

                // Set the 5 least significant bits of the indicator byte
                //assert_eq![chunk_data, [0; 31], "left = chunk_data"];
                //assert_eq!(data_length_last_31_bytes, 50, "indicator: {:?}", indicator);
                indicator = indicator
                    | data_length_last_31_bytes;
                // assert_eq!(indicator, 50, "indicator: {:?}", indicator);
            }
            // Set the first bit of the indicator byte, the skip_evm flag,
            // if a skip_evm opcode was called.
            // This will be false until there are multiple EVMs.
            // The skip_evm opcode could change the value of skip_evm to true
            if skip_evm {
                // Set SKIP_EVM flag to 1
                indicator = indicator | 0b1000_0000;
            }
            ch = Chunk::new(indicator, chunk_data);
            chunks.push(ch);
        }
        chunks/* Ok(chunks) */
    }

    /// Create a blob from a set of chunks
    pub fn from_chunks(chunks: Vec<Chunk>) -> Blob {
        let mut data = vec![];
        for ch in chunks {
            let mask: u8 = 0b0001_1111;
            let data_bytes_length_in_terminal_chunk = &ch.indicator & mask;
            // read the first 3 bits of the indicator and discard the result.
            //BitReader::new(&[ch.indicator]).read_u8(3).unwrap();
            // read the remaining 5 bits into length bits
            //let mut data_bytes_length_in_terminal_chunk
            //    = BitReader::new(&[ch.indicator]).read_u8(5).unwrap();
            for i in 0..CHUNK_DATA_SIZE {
                data.push(ch.data[i as usize]);
            }
            /* This is commented out because the else block for a terminal chunk is not
            writing the final 0 bytes, where data above was instantiated as an empty vector.
            There is a better way to do this. We can instantiate data as vec![0; 31],
            and just have one for loop, regardless of the value of data_bytes_length_in_terminal_chunk.
            if data_bytes_length_in_terminal_chunk == 0 {
                // Chunk is not terminal, read all CHUNK_DATA_SIZE bytes into data.
                for i in 0..CHUNK_DATA_SIZE {
                    data.push(ch.data[i as usize]);
                }
            } else {
                // Chunk is terminal, read data_bytes_length_in_terminal_chunk bytes into data
                for i in 0..data_bytes_length_in_terminal_chunk {
                    data.push(ch.data[i as usize]);
                }
            }
            */
        }
        Blob {
            data
        }
    }

    /// Put blob chunks into (a) collation body(ies).
    /// This is incomplete.
    /// TODO:  functionality for packing multiple blobs into
    /// one collation body.
    /// Further details, see:
    /// https://ethresear.ch/t/blob-serialisation/1705/17 */
    /* if to_chunks(blob).length > CHUNKS_PER_COLLATION {
        Serialize a blob into multiple collation bodies.
    } else {
        Pack the blob chunks into the collation body.
    }*/
    pub fn blob_to_collation_body(self,
        /* collation_header_hash: CollationHeaderHash */) -> Body {
        // We can't just create a collation body, then put a blob into that,
        // since we need to maintain an order of putting blobs into
        // collation bodies, and bodies into collations, and collations
        // ordered in each shard. So this is more for testing, it will need
        // to be modified to find a collation body that has been created by
        // it's CollationHeaderHash. Instantiate a var. to represent a
        // collation body from the input collation_header_hash.
        let bytes_per_blob: usize = self.data.len();
        let blob_as_chunks = self.to_chunks(false);
        if bytes_per_blob > DATA_BYTES_PER_COLLATION {
            //let mut chunk = [0; 31]
            //let mut body = Body::new(chunk);
            //let mut blob_bodies = BlobBodies::new(body);
            /* TODO: this is more of a hack than a proper fix. We still want this to panic to avoid unexpected behaviour,
            or be able to serialize a blob that is larger than a collation (although perhaps not many will do that due
            to the expense, and implementing this is not a high priority, it can probably be done in like phase 6 or 
            later).*/
            warn!("Sorry, sharding developers and researchers haven't agreed on a way to serialize blobs that are \
                larger than a DATA_BYTES_PER_COLLATION ({:?} bytes), please split this blob up into blobs that are  \
                smaller than this and try again, or lobby us to do this.", DATA_BYTES_PER_COLLATION);
            // TODO @dev fix panic case with blob serialization (tim will do)
            /*error!("Sorry, sharding developers and researchers haven't agreed on a way to serialize blobs that are \
                larger than a DATA_BYTES_PER_COLLATION ({:?} bytes), please split this blob up into blobs that are  \
                smaller than this and try again, or lobby us to do this.", DATA_BYTES_PER_COLLATION);
            */
            //process::exit(1);
            /*
            for chunk in blob_as_chunks {
                if chunk % CHUNKS_PER_COLLATION == 0 {
                    // 0..CHUNKS_PER_COLLATION - 1 = first collation body
                    // CHUNKS_PER_COLLATION * n = start of a new collation body
                    // Therefore, start a new collation body.
                    let mut body = Body::new(chunk);
                } else {
                    body.push(chunk);
                }
                if chunk % CHUNKS_PER_COLLATION == CHUNKS_PER_COLLATION - 1 {
                    // first collation of a blob
                } else  {
                    BlobBodies.push(body)
                }
            }
            */
        }
        // Find a collation body that has been created.
        // Search the shard binary Merkle trie for a collation that has the input collation_header_hash.
        // TODO

        // Check that the blob will fit into this particular body (it may be partially full or full).
        // TODO

        // Skip these two steps for now, since storage hasn't finished development.
        // Just create a new collation body.

        // Put the blob into this collation body.
        let body: Body = Body::new(blob_as_chunks);
        /*
        for chunk in blob_as_chunks {
            if chunk == blob_as_chunks[0] {
                // This applies when serializing a blob into a new collation body
                // 0..CHUNKS_PER_COLLATION - 1 = first collation body
                // CHUNKS_PER_COLLATION * n = start of a new collation body
                // Therefore, start a new collation body.

            } else {
                body.chunks.push(chunk);
            }
        }
        */
        body
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn mb_blob_zeros_to_collation_body() {
        // run as cargo test mb_blob_zeros_to_collation_body -- --ignored -> mb_blob_zeros_to_collation_body.log
        let blob = Blob::new(vec![0; DATA_BYTES_PER_COLLATION]);
        // let mut blob_as_chunks = blob.to_chunks();
        let sample_body = blob.blob_to_collation_body();

/*         let sample_chunk = Chunk::new(0, [0; 31]);
        let sample_chunks = vec![sample_chunk; CHUNKS_PER_COLLATION];
        let blob2 = Blob::new(vec![1; DATA_BYTES_PER_COLLATION]);
        let blob_chunks = blob2.to_chunks(false);*/
        let non_terminal_chunk_indicator = Chunk::build_indicator(false, false, 0);
        // Since all bytes are 0, the length is 0.
        let terminal_chunk_indicator = Chunk::build_indicator(false, true, 0);
        let mut correct_blob_chunks = vec![Chunk::new(non_terminal_chunk_indicator,
            [0; CHUNK_DATA_SIZE]); CHUNKS_PER_COLLATION - 1];
        correct_blob_chunks.push(Chunk::new(terminal_chunk_indicator, [0; 31]));
        let expected_body = Body::new(correct_blob_chunks);

        assert_eq!(sample_body, expected_body);
    }

    #[test]
    fn mb_blob_to_collation_body() {
        // run as cargo test mb_blob_to_collation_body -- --ignored -> mb_blob_to_collation_body.log
        let blob = Blob::new(vec![1; DATA_BYTES_PER_COLLATION]);
        // let mut blob_as_chunks = blob.to_chunks();
        let sample_body = blob.blob_to_collation_body();

/*         let sample_chunk = Chunk::new(0, [0; 31]);
        let sample_chunks = vec![sample_chunk; CHUNKS_PER_COLLATION];
        let blob2 = Blob::new(vec![1; DATA_BYTES_PER_COLLATION]);
        let blob_chunks = blob2.to_chunks(false);*/
        let non_terminal_chunk_indicator = Chunk::build_indicator(false, false, 0);
        let terminal_chunk_indicator = Chunk::build_indicator(false, true, 31);
        let mut correct_blob_chunks = vec![Chunk::new(non_terminal_chunk_indicator,
            [1; CHUNK_DATA_SIZE]); CHUNKS_PER_COLLATION - 1];
        correct_blob_chunks.push(Chunk::new(terminal_chunk_indicator, [1; 31]));
        let expected_body = Body::new(correct_blob_chunks);

        assert_eq!(sample_body, expected_body);
    }

    #[test]
    // TODO @dev fix this
    //#[should_panic]
    fn mb_1_b_blob_to_2_colltn_bodies() {
        let blob = Blob::new(vec![0; COLLATION_SIZE + 1]);
        // assert_eq!(blob.data.len(), COLLATION_SIZE + 1); // this actually passes
        // assert!(blob.data.len() > COLLATION_SIZE); // this passes if you uncomment this, but fails otherwise.
        // let mut blob_as_chunks = to_chunks(blob);
        let body = blob.blob_to_collation_body();
    }

    #[test]
    fn to_chunks_skip_evm_0_4_b_blob() {
        let mut blob = Blob::new(vec![0; 31]);
        for i in 0..4 {
            blob.data[i] = 0xff;
        }
        let blob_chunks = blob.to_chunks(false);
        // If you set the length to 4, it will not include the zeros in blob.data.
        let terminal_chunk_indicator = Chunk::build_indicator(false, true, 4);
        let correct_blob_chunks = vec![Chunk::new(terminal_chunk_indicator,
                                                                [0xff, 0xff, 0xff, 0xff,
                                                                0, 0, 0, 0, 0, 0, 0, 0, 0,
                                                                0, 0, 0, 0, 0, 0, 0, 0, 0,
                                                                0, 0, 0, 0, 0, 0, 0, 0, 0])];
        assert_eq!(blob_chunks, correct_blob_chunks,
            "\nblob_chunks[0].data.len(): {:?}, correct_blob_chunks[0].data.len(): {:?}\
            \ndifference = {:?}\n",
            blob_chunks[0].data.len(), correct_blob_chunks[0].data.len(),
            blob_chunks[0].data.len() - correct_blob_chunks[0].data.len());
    }

    #[test]
    fn to_chunks_skip_evm_0_32_b_blob() {
        let blob = Blob::new(vec![0xff; 32]);
        let blob_chunks = blob.to_chunks(false);
        let non_terminal_chunk_indicator = Chunk::build_indicator(false, false, 0);
        let terminal_chunk_indicator = Chunk::build_indicator(false, true, 1);
        let mut correct_blob_chunks = vec![Chunk::new(non_terminal_chunk_indicator,
                                                                [0xff, 0xff, 0xff, 0xff,
                                                                0xff, 0xff, 0xff, 0xff,
                                                                0xff, 0xff, 0xff, 0xff,
                                                                0xff, 0xff, 0xff, 0xff,
                                                                0xff, 0xff, 0xff, 0xff,
                                                                0xff, 0xff, 0xff, 0xff,
                                                                0xff, 0xff, 0xff, 0xff,
                                                                0xff, 0xff, 0xff])];
        correct_blob_chunks.push(Chunk::new(terminal_chunk_indicator, [0xff,
                                                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0]));
        assert_eq!(blob_chunks, correct_blob_chunks,
            "\nblob_chunks[0].data.len(): {:?}, correct_blob_chunks[0].data.len(): {:?}\
            \ndifference = {:?}\n",
            blob_chunks[0].data.len(), correct_blob_chunks[0].data.len(),
            blob_chunks[0].data.len() - correct_blob_chunks[0].data.len());
    }

    #[test]
    fn to_chunks_skip_evm_0_128_b_blob() {
        let blob = Blob::new(vec![0xff; CHUNK_DATA_SIZE*4 + 4]);
        let blob_chunks = blob.to_chunks(false);
        let non_terminal_chunk_indicator = Chunk::build_indicator(false, false, 0);
        let terminal_chunk_indicator = Chunk::build_indicator(false, true, 4);
        let mut correct_blob_chunks = vec![Chunk::new(non_terminal_chunk_indicator,
            [0xff; CHUNK_DATA_SIZE]); 4];
        correct_blob_chunks.push(Chunk::new(terminal_chunk_indicator,
                                                                [0xff, 0xff, 0xff, 0xff,
                                                                0, 0, 0, 0, 0, 0, 0, 0, 0,
                                                                0, 0, 0, 0, 0, 0, 0, 0, 0,
                                                                0, 0, 0, 0, 0, 0, 0, 0, 0]));
        assert_eq!(blob_chunks, correct_blob_chunks,
            "\nblob_chunks[0].data.len(): {:?}, correct_blob_chunks[0].data.len(): {:?}\
            \ndifference = {:?}\n",
            blob_chunks[0].data.len(), correct_blob_chunks[0].data.len(),
            blob_chunks[0].data.len() - correct_blob_chunks[0].data.len(),
            );
    }

    #[test]
    fn to_chunks_skip_evm_1_128_b_blob() {
        let blob = Blob::new(vec![0xff; CHUNK_DATA_SIZE*4+4]);
        let blob_chunks = blob.to_chunks(true);
        let non_terminal_chunk_indicator = Chunk::build_indicator(true, false, 0);
        let terminal_chunk_indicator = Chunk::build_indicator(true, true, 4);
        let mut correct_blob_chunks = vec![Chunk::new(non_terminal_chunk_indicator,
            [0xff; CHUNK_DATA_SIZE]); 4];
        correct_blob_chunks.push(Chunk::new(terminal_chunk_indicator,
                                                                [0xff, 0xff, 0xff, 0xff,
                                                                0, 0, 0, 0, 0, 0, 0, 0, 0,
                                                                0, 0, 0, 0, 0, 0, 0, 0, 0,
                                                                0, 0, 0, 0, 0, 0, 0, 0, 0]));
        assert_eq!(blob_chunks, correct_blob_chunks,
            "\nblob_chunks[0].data.len(): {:?}, correct_blob_chunks[0].data.len(): {:?}\
            \ndifference = {:?}\n",
            blob_chunks[0].data.len(), correct_blob_chunks[0].data.len(),
            blob_chunks[0].data.len() - correct_blob_chunks[0].data.len());
    }

    #[test]
    fn from_chunks_skip_evm_0_4_b_blob() {
        // 0b0001_1111
        let terminal_chunk_indicator = Chunk::build_indicator(
            false, true, 4);
        let mut chunk_1_data = [0; 31];
        for i in 0..4 {
            chunk_1_data[i] = 255;
        }
        // Alternative: https://play.rust-lang.org/?gist=53969b6c3cad694d219091e8550c8ccc&version=undefined&mode=undefined
        // Less efficient: https://play.rust-lang.org/?gist=8ca17ca652ab781b5c4e1fe14ef4f919&version=stable&mode=debug
        let chunks = vec![Chunk::new(terminal_chunk_indicator, chunk_1_data)];
        let blob_from_chunks = Blob::from_chunks(chunks);
        let blob = Blob::new(chunk_1_data.to_vec());
        assert_eq!(blob, blob_from_chunks,
            "\nblob.data.len(): {:?}, blob_from_chunks.data.len(): {:?}\
            \ndifference = {:?}\n",
            blob.data.len(), blob_from_chunks.data.len(),
            blob.data.len() - blob_from_chunks.data.len());
    }

    #[test]
    fn from_chunks_skip_evm_0_32_b_blob() {
        // 0b0000_0000
        let non_terminal_chunk_indicator = Chunk::build_indicator(false, false, 0);
        let terminal_chunk_indicator = Chunk::build_indicator(
            false, true, 1 as u8);
        // 1 chunks, with every non-indicator byte as 255
        let mut chunks = vec![Chunk::new(non_terminal_chunk_indicator,
            [255; CHUNK_DATA_SIZE])];
        // 2nd terminal chunk, with one byte as 255.
        let mut chunk_2_data = [0; 31]; chunk_2_data[0] = 255;
        chunks.push(Chunk::new(terminal_chunk_indicator, chunk_2_data));
        let blob_from_chunks = Blob::from_chunks(chunks);
        let mut blob = Blob::new(vec![0; 62]);
        for i in 0..32 {
            blob.data[i] = 255;
        }
        assert_eq!(blob, blob_from_chunks,
            "\nblob.data.len(): {:?}, blob_from_chunks.data.len(): {:?}\
            \ndifference = {:?}\n",
            blob.data.len(), blob_from_chunks.data.len(),
            blob.data.len() - blob_from_chunks.data.len());
    }

    #[test]
    fn from_chunks_skip_evm_0_155_b_blob() {
        // 0b0000_0000
        let non_terminal_chunk_indicator = Chunk::build_indicator(false, false, 0);
        // 0b0001_1111
        let terminal_chunk_indicator = Chunk::build_indicator(
            false, true, CHUNK_DATA_SIZE as u8);
        // 4 chunks, with every non-indicator byte as 255
        let mut chunks = vec![Chunk::new(non_terminal_chunk_indicator,
            [255; CHUNK_DATA_SIZE]); 4];
        // 5th terminal chunk, also with every non-indicator byte as 255
        chunks.push(Chunk::new(terminal_chunk_indicator, [255; CHUNK_DATA_SIZE]));
        let blob_from_chunks = Blob::from_chunks(chunks);
        let blob = Blob::new(vec![255; CHUNK_DATA_SIZE*5]);//155
        assert_eq!(blob, blob_from_chunks,
            "\nblob.data.len(): {:?}, blob_from_chunks.data.len(): {:?}\
            \ndifference = {:?}\n",
            blob.data.len(), blob_from_chunks.data.len(),
            blob.data.len() - blob_from_chunks.data.len());
    }

    #[test]
    fn from_chunks_skip_evm_1_155_b_blob() {
        // 0b1000_0000
        let non_terminal_chunk_indicator = Chunk::build_indicator(true, false, 0);
        // 0b1001_1111
        let terminal_chunk_indicator = Chunk::build_indicator(
            true, true, CHUNK_DATA_SIZE as u8);
        // as above: 4 chunks, with every non-indicator byte as 255
        let mut chunks = vec![Chunk::new(non_terminal_chunk_indicator,
            [255; CHUNK_DATA_SIZE]); 4];
        // ditto: 5th terminal chunk, also with every non-indicator byte as 255
        chunks.push(Chunk::new(terminal_chunk_indicator, [255; CHUNK_DATA_SIZE]));
        let blob_from_chunks = Blob::from_chunks(chunks);
        let blob = Blob::new(vec![255; CHUNK_DATA_SIZE*5]);
        assert_eq!(blob, blob_from_chunks,
            "\nblob.data.len(): {:?}, blob_from_chunks.data.len(): {:?}\
            \ndifference = {:?}\n",
            blob.data.len(), blob_from_chunks.data.len(),
            blob.data.len() - blob_from_chunks.data.len());
    }
}

// Not used
/* From trying to use an array instead of a vector in the data field
of chunks and blobs, but this is probably unnecessary.
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
*/
