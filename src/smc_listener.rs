use ethereum_types;
use std::sync::mpsc;
use message;

pub struct SMCListener {
    period: ethereum_types::U256,
    notary_sender: mpsc::Sender<message::Message>
}

impl SMCListener {
    pub fn new(notary_sender: mpsc::Sender<message::Message>) -> SMCListener {
        SMCListener {
            period: ethereum_types::U256::from_dec_str("0").unwrap(),
            notary_sender
        }
    }

    fn register(&self, addr: ethereum_types::Address) -> Result<bool, &'static str> {
        Ok(false)
    }

    fn get_selected_notaries(&self, shard_id: usize) -> Vec<ethereum_types::Address> {
        vec![ethereum_types::Address::zero()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn it_registers() {
        let (tx, rx) = mpsc::channel();
        let smc = SMCListener::new(tx);
        let addr_bytes: [u8; 20] = [0x22, 0xFF, 0x31, 0x10, 0xA2, 0x82, 0xc1, 0x19, 0x77, 0x36, 0xb3, 0xfC, 0xe3, 0x4a, 0xD4, 0xFc, 0x5e, 0xEe, 0x75, 0xc8];
        let addr: ethereum_types::Address = ethereum_types::Address::from_slice(&addr_bytes);
        let result = smc.register(addr).unwrap();
        assert_eq!(true, result);
    }

    #[test]
    #[ignore]
    fn it_gets_selected_notaries() {
        let (tx, rx) = mpsc::channel();
        let smc = SMCListener::new(tx);
        let shard_id = 0;
        let addr = smc.get_selected_notaries(shard_id);

        // The dummy "selected notary"
        let addr_bytes: [u8; 20] = [0x6C, 0xaC, 0xE0, 0x52, 0x83, 0x24, 0xA8, 0xaf, 0xC2, 0xb1, 0x57, 0xCe, 0xbA, 0x3c, 0xDd, 0x2a, 0x27, 0xc4, 0xE2, 0x1f];
        let selected_addr = ethereum_types::Address::from_slice(&addr_bytes);

        assert_eq!(vec![selected_addr], addr);
    }

    #[test]
    #[ignore]
    fn it_listens_for_smc_events() {
        assert!(false);
    }
}