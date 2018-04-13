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
