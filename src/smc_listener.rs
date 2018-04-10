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

    #[test]
    fn it_registers() {
        let (tx, rx) = mpsc::channel();
        let smc = SMCListener::new(tx);
        let addr_bytes: [u8; 20] = [0x22, 0xFF, 0x31, 0x10, 0xA2, 0x82, 0xc1, 0x19, 0x77, 0x36, 0xb3, 0xfC, 0xe3, 0x4a, 0xD4, 0xFc, 0x5e, 0xEe, 0x75, 0xc8];
        let addr: Address = Address::from_slice(&addr_bytes);
        let result = smc.register(addr).unwrap();
        assert_eq!(true, result);
    }

    #[test]
    fn it_gets_eligible_collator() {
        let (tx, rx) = mpsc::channel();
        let smc = SMCListener::new(tx);
        let shard_id = 0;
        let addr = smc.get_eligible_collator(shard_id);

        // The dummy "eligible collator"
        let addr_bytes: [u8; 20] = [0x6C, 0xaC, 0xE0, 0x52, 0x83, 0x24, 0xA8, 0xaf, 0xC2, 0xb1, 0x57, 0xCe, 0xbA, 0x3c, 0xDd, 0x2a, 0x27, 0xc4, 0xE2, 0x1f];
        let eligible_addr = Address::from_slice(&addr_bytes);

        assert_eq!(eligible_addr, addr);
    }

    #[test]
    fn it_listens_for_smc_events() {
        assert!(false);
    }
}