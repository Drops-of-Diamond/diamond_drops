extern crate diamond_drops;

use diamond_drops::collator;

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