use collation::header;
use collation::body;
use collation::collation;

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
        let gsid = ethereum_types::U256::from_dec_str("1");
        let gph = ethereum_types::H256::zero();
        let gcr = ethereum_types::H256::zero();
        let gpa = ethereum_types::Address::zero();
        let gpd = ethereum_types::U256::from_dec_str("0");
        let gps = ethereum_types::Signature::zero();
        let gh = header::Header::new(sid, ph, cr, pa, pd, ps);

        let b = body::Body::new();
        let c = collation::Collation::new(h, b);
        let c2 = c.clone();

        let t = Tree::new();

        t.push(c);

        assert_eq!(vec![c2], t.nodes);

        assert!(false);
    }

}