use ethereum_types::Address;
use std::sync::mpsc;
use message;

pub struct SMCListener {
    period: usize,
    collator_sender: mpsc::Sender<message::Message>
}

impl SMCListener {
    pub fn new(collator_sender: mpsc::Sender<message::Message>) -> SMCListener {
        SMCListener {
            period: 0,
            collator_sender
        }
    }

    fn register(&self, addr: Address) -> Result<bool, &'static str> {
        Ok(false)
    }

    fn get_eligible_collator(&self, shard_id: usize) -> Address {
        Address::zero()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}