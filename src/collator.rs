pub struct Collator {
    pub registered: bool,
    pub eligible: bool,
    pub shard_id: usize
}

impl Collator {
    pub fn new() -> Collator {
        Collator {
            registered: false,
            eligible: false,
            shard_id: 0
        }
    }

    pub fn run(&mut self) {
        if !self.registered {
            self.register();
        }

        loop {
            while self.eligible{
                self.get_eligibility();
                self.get_collation_headers();
                self.download_collations();
                self.get_collation_headers();
                self.download_collations();
                self.collect_proposals();
                self.select_proposal();
                self.add_header();
            }
        }
    }

    pub fn register(&self) {}

    pub fn check_smc(&self) {}

    pub fn get_eligibility(&self) {}

    pub fn get_collation_headers(&self) {}

    pub fn download_collations(&self) {}

    pub fn collect_proposals(&self) {}

    pub fn select_proposal(&self) {}

    pub fn add_header(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_register() {
        let c = collator::Collator::new();

        assert_eq!(c.registered, false);

        c.register();

        assert_eq!(c.registered, true);
    }

    #[test]
    fn it_checks_smc() {
        assert!(false);
    }

    #[test]
    fn it_gets_eligibility() {
        assert!(false);
    }

    #[test]
    fn it_gets_collation_headers() {
        assert!(false);
    }

    #[test]
    fn it_downloads_collations() {
        assert!(false);
    }

    #[test]
    fn it_collects_proposals() {
        assert!(false);
    }

    #[test]
    fn it_selects_proposal() {
        assert!(false);
    }

    #[test]
    fn it_adds_header() {
        assert!(false);
    }
}