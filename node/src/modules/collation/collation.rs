use modules::collation::body::{Body/* , BlobBodies */};
use modules::collation::header::Header;

#[derive(PartialEq, Clone, Debug)]
pub struct Collation {
    pub header: Header,
    pub body: Body
}

impl Collation {
    pub fn new(header: Header, body: Body) -> Collation {
        Collation {
            header,
            body
        }
    }
}