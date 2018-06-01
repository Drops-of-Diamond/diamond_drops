use modules::collation::chunk::Chunk;
use modules::collation::blob::Blob;
use modules::constants::COLLATION_SIZE;
use modules::errors::*;

/// Collation body storing arbitrary data or blobs, serialized into 32-byte chunks.
#[derive(PartialEq, Debug, Clone)]
pub struct Body{/*; this doesn't work, presumably because */
    pub chunks: Vec<Chunk>
}

impl Body {
    pub fn new(chunks: Vec<Chunk>) -> Body {
        Body {
            chunks
        }
    }
}

// This must be outside the impl Body, since you can't create a 
// specific instance in an impl, AFAIK.
pub fn create_sample_collation_body() -> Body {
        let blob = Blob::new(vec![4; COLLATION_SIZE]);
        let sample_body = blob.blob_to_collation_body();
        //println!("{:?}", body);
        sample_body
}

/// Serialize collation bodies that correspond to the same blob. In practice a blob should contain
///  a hash and this struct should then contain that hash as a field.
pub struct BlobBodies {
    body: Body
}

impl BlobBodies {
    pub fn new(body: Body) -> BlobBodies {
        BlobBodies {
            body
        }
    }
}

