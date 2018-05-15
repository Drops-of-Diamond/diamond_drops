use modules::collation::{header, body};

#[derive(PartialEq, Clone, Debug)]
pub struct Collation {
    pub header: header::Header,
    pub body: body::Body
}

impl Collation {
    pub fn new(header: header::Header, body: body::Body) -> Collation {
        Collation {
            header,
            body
        }
    }
}