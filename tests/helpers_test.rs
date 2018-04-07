extern crate diamond_drops;

use diamond_drops::helpers::thread_names;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_sets_thread_name_for_config_arguments() {
        let test_proposer_name = thread_names::Mode::Proposer.value();
        let test_collator_name = thread_names::Mode::Collator.value();

        assert_eq!(test_proposer_name, "proposer".to_string());
        assert_eq!(test_collator_name, "collator".to_string());
    }
}