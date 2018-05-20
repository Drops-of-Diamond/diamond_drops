use modules::collation::{header, body};

#[derive(PartialEq, Clone, Debug)]
pub struct Collation {
     header: header::Header,
     body: body::Body
}

impl Collation {
    pub fn new(header: header::Header, body: body::Body) -> Collation {
        Collation {
            header,
            body
        }
    }

    // Put blob chunks into (a) collation body(ies).
    /* if to_chunks(blob).length > CHUNKS_PER_COLLATION {
        Serialize a blob into multiple collation bodies.
    } else {
        Pack the blob chunks into the collation body.
    }
    */
}