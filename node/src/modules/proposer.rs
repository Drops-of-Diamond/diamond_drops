use ethereum_types;

pub struct Proposer {
    id: ethereum_types::U256
}

impl Proposer {
    pub fn new() -> Proposer {
        Proposer {
            id: ethereum_types::U256::from_dec_str("0").unwrap()
        }
    }

    pub fn run(&self) {}

    fn register(&self) {}

    fn add_balance(&self) {}

    fn collect_blobs(&self) {}

    fn prepare_collation(&self) {}

    fn reveal_proposal(&self) {}

    fn broadcast_collation_body(&self) {}
}