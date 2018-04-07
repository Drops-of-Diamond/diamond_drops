use std::{thread, time};

pub struct Proposer;

impl Proposer {
    pub fn new() -> Proposer {
        Proposer
    }

    pub fn run(&self) {
        let ten_millis = time::Duration::from_millis(10);
        thread::sleep(ten_millis);
        panic!("Make this test fail");
    }

    fn register(&self) {}

    fn add_balance(&self) {}

    fn collect_blobs(&self) {}

    fn prepare_collation(&self) {}

    fn reveal_proposal(&self) {}

    fn broadcast_collation_body(&self) {}
}