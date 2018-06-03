use std::collections::BTreeMap;
use modules::primitives::CollationHeaderHash;
use modules::collation::collation::{Collation};
use modules::collation::body;
use modules::collation::header::{Header};
use modules::collation::blob;
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

    pub fn create_collation_index() -> (BTreeMap<ShardIdHash, Collation>) {

        // Here we create an index mapping for storing collations
        //
        // Index = {key -> value}
        //
        // CollationIndex = {ShardIdHash -> Collation}
        //
        let mut index: BTreeMap<ShardIdHash, Collation> = BTreeMap::new();


        // Now we provide a sample collation to store in our index
        //
        // A collation requires 2 parameters
        //
        // 1. header
        // 2. body

        let header = Header::create_sample_collation_header();
        let body = body::create_sample_collation_body();

        let key: ShardIdHash = header.shard_id;
        let value: Collation = Collation::new(header, body);

        index.insert(key, value);

        index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_collation_index() {

        let mut index: BTreeMap<ShardIdHash, Collation> = CollationIndex::create_collation_index();

        assert!(!index.is_empty());

        index.clear();
        assert!(index.is_empty());
    }
}
