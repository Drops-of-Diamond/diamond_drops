/* Copyright 2018 James Ray (@jamesray1), Josiah Evans (@ChosunOne), Luke Schoen (@ltfschoen)

This is free and unencumbered software released into the public domain.

Anyone is free to copy, modify, publish, use, compile, sell, or
distribute this software, either in source code form or as a compiled
binary, for any purpose, commercial or non-commercial, and by any
means.

In jurisdictions that recognize copyright laws, the author or authors
of this software dedicate any and all copyright interest in the
software to the public domain. We make this dedication for the benefit
of the public at large and to the detriment of our heirs and
successors. We intend this dedication to be an overt act of
relinquishment in perpetuity of all present and future rights to this
software under copyright law.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE 
AUTHORS, James Ray, Josiah @ChosunOne, and Luke Schoen
BE LIABLE FOR ANY CLAIM, DAMAGES OR
OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.
 
For more information, please refer to <http://unlicense.org>
*/

use ethereum_types;
use std::sync::mpsc;
use message;

/// This will monitor the SMC for changes and then send relevant information to the notary or the proposer.
pub struct SMCListener {
    period: ethereum_types::U256,
    notary_sender: mpsc::Sender<message::Message>
}

impl SMCListener {
    /// Creates a new SMC Listener
    pub fn new(notary_sender: mpsc::Sender<message::Message>) -> SMCListener {
        SMCListener {
            period: ethereum_types::U256::from_dec_str("0").unwrap(),
            notary_sender
        }
    }

    fn register_notary_address(&self, notary_addr: ethereum_types::Address) -> bool {
        // TODO - Implement registration of notary address in notary registry of SMC Contract
        let result: Result<String, String> = Result::Err(String::from("Error"));

        match result {
            Ok(msg) => { info!("Success registered notary address {:?} with SMC Contract", notary_addr); true },
            Err(e) => { error!("Error registering notary address {:?} with SMC Contract: {:?}", notary_addr, e); false }
        }
    }

    fn get_selected_notaries(&self, shard_id: ethereum_types::U256) -> Vec<ethereum_types::Address> {
        vec![ethereum_types::Address::zero()]
    }

    fn register_proposer_address(&self, proposer_addr: ethereum_types::Address) -> bool {
        // TODO - Implement registration of proposer address in proposer registry of SMC Contract
        let result: Result<String, String> = Result::Err(String::from("Error"));

        match result {
            Ok(msg) => { info!("Success registered proposer address {:?} with SMC Contract", proposer_addr); true },
            Err(e) => { error!("Error registering proposer address {:?} with SMC Contract: {:?}", proposer_addr, e); false }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn it_registered_notary_address_in_notary_registry_of_smc_contract() {
        let (tx, rx) = mpsc::channel();
        let smc = SMCListener::new(tx);
        let notary_addr_bytes: [u8; 20] = [0x22, 0xFF, 0x31, 0x10, 0xA2,
                                           0x82, 0xc1, 0x19, 0x77, 0x36, 
                                           0xb3, 0xfC, 0xe3, 0x4a, 0xD4, 
                                           0xFc, 0x5e, 0xEe, 0x75, 0xc8];
        let notary_addr: ethereum_types::Address = ethereum_types::Address::from_slice(&notary_addr_bytes);
        let result = smc.register_notary_address(notary_addr);
        assert_eq!(true, result);
    }

    #[test]
    #[ignore]
    fn it_registered_proposer_address_in_proposer_registry_of_smc_contract() {
        let (tx, rx) = mpsc::channel();
        let smc = SMCListener::new(tx);
        let proposer_addr_bytes: [u8; 20] = [0x22, 0xFF, 0x31, 0x10, 0xA2, 
                                             0x82, 0xc1, 0x19, 0x77, 0x36, 
                                             0xb3, 0xfC, 0xe3, 0x4a, 0xD4, 
                                             0xFc, 0x5e, 0xEe, 0x75, 0xc8];
        let proposer_addr: ethereum_types::Address = ethereum_types::Address::from_slice(&proposer_addr_bytes);
        let result = smc.register_proposer_address(proposer_addr);
        assert_eq!(true, result);
    }

    #[test]
    #[ignore]
    fn it_gets_selected_notaries() {
        let (tx, rx) = mpsc::channel();
        let smc = SMCListener::new(tx);
        let shard_id = ethereum_types::U256::from_dec_str("0").unwrap();
        let notary_addr = smc.get_selected_notaries(shard_id);

        // The dummy "selected notary"
        let notary_addr_bytes: [u8; 20] = [0x6C, 0xaC, 0xE0, 0x52, 0x83, 
                                           0x24, 0xA8, 0xaf, 0xC2, 0xb1, 
                                           0x57, 0xCe, 0xbA, 0x3c, 0xDd, 
                                           0x2a, 0x27, 0xc4, 0xE2, 0x1f];
        let selected_notary_addr = ethereum_types::Address::from_slice(&notary_addr_bytes);

        assert_eq!(vec![selected_notary_addr], notary_addr);
    }

    #[test]
    #[ignore]
    fn it_listens_for_smc_events() {
        assert!(false);
    }
}
