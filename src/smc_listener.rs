use ethereum_types::Address;

pub struct SMCListener {
    period: usize
}

impl SMCListener {
    fn new() -> SMCListener {
        SMCListener {
            period: 0
        }
    }

    fn get_eligible_collator(&self, shard_id: usize) -> Address {
        Address::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gets_eligible_collator() {
        assert!(false);
    }

    #[test]
    fn it_listens_for_smc_events() {
        assert!(false);
    }
}