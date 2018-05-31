use std::collections::BTreeMap;
use modules::collation::collation;
use modules::primitives::CollationHeaderHash;
use modules::collation::collation::{Collation};
use modules::collation::body::{Body};
use modules::collation::blob::{Blob};
use modules::collation::header::{Header};
use modules::primitives::{ShardIdHash};

#[derive(PartialEq, Clone, Debug)]
pub struct CollationIndex {

    // TODO @devs: swap out BTreeMap with BinTrie after complete
    pub collation_index: BTreeMap<ShardIdHash, Collation>
}

impl CollationIndex {

    pub fn new(collation_index: BTreeMap<ShardIdHash, Collation>) -> CollationIndex {
        CollationIndex {
            collation_index
        }
    }

    pub fn create_collation_index() {

        // Here we create an index mapping for storing collations
        //
        // Index = {key -> value}
        //
        // CollationIndex = {ShardIdHash -> Collation}
        //
        let index: BTreeMap<ShardIdHash, Collation> = BTreeMap::new();


        // Now we provide a sample collation to store in our index
        //
        // A collation requires 2 parameters
        //
        // 1. header
        // 2. body

        let header = Header::create_sample_collation_header();
        let body = Body::create_sample_collation_body();

        let key: ShardIdHash = header.shard_id;
        let value = Collation::new(header, body);

        println!("key: ShardIdHash = {:?}", key);
        println!("value: Collation = {:?}", value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO @devs: add more test cases that are relevant for collation(s)

    // not used yet, template from blob.rs
//    #[test]
//    fn generateCollationIndex() {
//        // run as cargo test mb_blob_zeros_to_collation_body -- --ignored -> mb_blob_zeros_to_collation_body.log
//
//        let testCollationIndex = CollationIndex::create_collation_index();
//        let blob = Blob::new(vec![0; DATA_BYTES_PER_COLLATION]);
//        // let mut blob_as_chunks = blob.to_chunks();
//        let sample_body = blob.blob_to_collation_body();
//
//        /*         let sample_chunk = Chunk::new(0, [0; 31]);
//                let sample_chunks = vec![sample_chunk; CHUNKS_PER_COLLATION];
//                let blob2 = Blob::new(vec![1; DATA_BYTES_PER_COLLATION]);
//                let blob_chunks = blob2.to_chunks(false);*/
//
//
//
//
//        let non_terminal_chunk_indicator = Chunk::build_indicator(false, false, 0);
//        // Since all bytes are 0, the length is 0.
//        let terminal_chunk_indicator = Chunk::build_indicator(false, true, 0);
//        let mut correct_blob_chunks = vec![Chunk::new(non_terminal_chunk_indicator,
//                                                      [0; CHUNK_DATA_SIZE]); CHUNKS_PER_COLLATION - 1];
//        correct_blob_chunks.push(Chunk::new(terminal_chunk_indicator, [0; 31]));
//        let expected_body = Body::new(correct_blob_chunks);
//
//        assert_eq!(sample_body, expected_body);
//    }
}
