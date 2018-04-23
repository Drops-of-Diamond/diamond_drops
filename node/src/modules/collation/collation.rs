use modules::collation::header;
use modules::collation::body;

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
}