pub struct Collator {
    pub registered: bool,
    pub eligible: bool
}

impl Collator {
    pub fn new() -> Collator {
        Collator {
            registered: false,
            eligible: false
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