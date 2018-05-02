use modules::collation:body::{CollationVariables}

fn main() {
    // pow is not a constant function, so this can't be a constant or static.
    pub let collation_variables = CollationVariables {
        collation_size: 2_u32.pow(20), // bytes
        chunks_per_collation: collation_size / CHUNK_SIZE, // 2**15
        /// size of a blob filling a full collation
        max_blob_size: chunks_per_collation * CHUNK_DATA_SIZE,
    }
}