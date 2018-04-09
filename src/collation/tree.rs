use collation::header;
use collation::body;
use collation::collation;
use ethereum_types;

pub struct Tree {
    nodes: Vec<collation::Collation>
}

impl Tree {
    pub fn new() -> Tree {
        Tree {
            nodes: vec![]
        }
    }

    pub fn push(&mut self, collation: collation::Collation) {
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_pushes_collation_header_into_tree() {
        // Test genesis collation 
        assert!(false);
    }

}