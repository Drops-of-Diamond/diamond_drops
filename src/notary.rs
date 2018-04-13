use collation::header;
use collation::collation;
use collation::body;
use message;

use ethereum_types;

use std::sync::mpsc;
use std::collections::HashMap;

pub struct Notary {
    selected: bool,
    shard_id: ethereum_types::U256,
    collation_vectors: HashMap<ethereum_types::U256, Vec<collation::Collation>>,
    listener: mpsc::Receiver<message::Message>,
}

impl Notary {
    pub fn new(listener: mpsc::Receiver<message::Message>) -> Notary {
        Notary {
            selected: false,
            shard_id: ethereum_types::U256::from_dec_str("0").unwrap(),
            listener,
            collation_vectors: HashMap::new()
        }
    }

    pub fn run(&mut self) {
        loop {
            // Get message from the SMC listener
            let msg_result = self.listener.recv();
            let msg: message::Message;

            match msg_result {
                Ok(m) => { msg = m }
                Err(e) => { eprintln!("Error receiving message from SMC listener: {}", e); continue }
            }

            // Respond to SMC listener message
            match msg {
                message::Message::Selected { value } => { self.selected = value; }
                message::Message::ShardId { value } => { self.shard_id = value; },
                message::Message::Collation { value } => { self.store_collation(value); },
                message::Message::Proposal { value } => { self.store_proposal(value); }
            }

            if self.selected {
                self.get_availability();
                self.submit_vote();
            }
        }
    }

    fn store_collation(&mut self, collation: collation::Collation) {
        // Insert an entry if the current shard_id is not part of the notary
        self.collation_vectors.entry(self.shard_id).or_insert(vec![]);
        let vector = self.collation_vectors.get_mut(&self.shard_id).unwrap();
        vector.push(collation);
    }

    fn store_proposal(&mut self, collation: collation::Collation) {}

    fn get_availability(&mut self) {}

    fn submit_vote(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_stores_collation() {
        // Create the notary
        let (tx, rx) = mpsc::channel();
        let mut notary = Notary::new(rx);

        // Collation parameters
        let g_shard_id = ethereum_types::U256::from_dec_str("0").unwrap();
        let g_chunk_root = ethereum_types::H256::zero();
        let g_period = ethereum_types::U256::from_dec_str("0").unwrap();
        let g_proposer_address = ethereum_types::Address::zero(); 

        // Genesis collation
        let genesis_header = header::Header::new(g_shard_id, g_chunk_root, g_period, g_proposer_address);
        let genesis_collation = collation::Collation::new(genesis_header, body::Body);
        let genesis_collation_cmp = genesis_collation.clone();

        // First Collation parameters
        let cr1 = ethereum_types::H256::zero();
        let period1 = ethereum_types::U256::from_dec_str("1").unwrap();
        let address1 = ethereum_types::Address::zero();

        // First collation
        let first_header = header::Header::new(g_shard_id, cr1, period1, address1);
        let first_collation = collation::Collation::new(first_header, body::Body);
        let first_collation_cmp = first_collation.clone();

        // Push genesis collation into notary
        notary.store_collation(genesis_collation);
        notary.store_collation(first_collation);

        // Check that the operations succeded
        let vector = notary.collation_vectors.get(&g_shard_id).unwrap();
        assert_eq!(vector[0], genesis_collation_cmp);
        assert_eq!(vector[1], first_collation_cmp);
    }

    #[test]
    fn it_stores_proposals() {
        assert!(false);
    }

    #[test]
    fn it_selects_vote() {
        assert!(false);
    }

    #[test]
    fn it_submits_vote() { assert!(false); }