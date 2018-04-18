/* Copyright 2018 AUTHORS, James Ray, Josiah @ChosunOne, and Luke Schoen

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

//! ![uml](ml.svg)

// External crates
extern crate ethereum_types;
extern crate tiny_keccak;

extern crate fern;
#[macro_use]
extern crate log;
extern crate chrono;

// Module declarations
pub mod cli;
pub mod proposer;
pub mod notary;
pub mod smc_listener;
pub mod collation;
pub mod message;
pub mod client_thread;

// std imports
use std::thread;
use std::time::Duration;
use std::sync::mpsc;

/// The main function to run the node.  
/// 
/// # Inputs
/// 
/// config - A struct containing the configuration values for the client
pub fn run(config: cli::config::Config) -> () {
    debug!("Client Mode: {:?}", config.mode);

    match config.mode {
        cli::config::Mode::Proposer => {
            debug!("Running as a proposer");

            // Create the SMC Listener
            let (smc_tx, smc_rx) = mpsc::channel();
            let smc = smc_listener::SMCListener::new(smc_tx);

            // Start a thread to run the proposer
            let mut proposer_thread = client_thread::ClientThread::new(&config.mode);
            proposer_thread.run(smc_rx);

            if cli::config_env::is_running_with_cargo_test() {
                thread::sleep(Duration::from_secs(1));
                let _result = proposer_thread.manager.unwrap().send(client_thread::Command::Terminate);
            }

            // Wait for thread termination
            match proposer_thread.handle.unwrap().join() {
                Ok(x) => { debug!("Successful proposer thread join {:?}", x); () },
                Err(e) => { panic!("Failed proposer thread join {:?}", e); }
            }
        },
        cli::config::Mode::Notary => {
            debug!("Running as a notary");

            // Create the SMC Listener
            let (smc_tx, smc_rx) = mpsc::channel();
            let smc = smc_listener::SMCListener::new(smc_tx);

            // Start a thread to run the notary
            let mut notary_thread = client_thread::ClientThread::new(&config.mode);
            notary_thread.run(smc_rx);

            if cli::config_env::is_running_with_cargo_test() {
                thread::sleep(Duration::from_secs(1));
                let _result = notary_thread.manager.unwrap().send(client_thread::Command::Terminate);
            }

            // Wait for thread termination
            match notary_thread.handle.unwrap().join() {
                Ok(x) => { debug!("Successful notary thread join {:?}", x); () },
                Err(e) => { panic!("Failed notary thread join {:?}", e); }
            }
        },
        cli::config::Mode::Both => {
            debug!("Running as both a proposer and notary");

            // Create the SMC Listeners
            let (notary_smc_tx, notary_smc_rx) = mpsc::channel();
            let (proposer_smc_tx, proposer_smc_rx) = mpsc::channel();
            let notary_smc = smc_listener::SMCListener::new(notary_smc_tx);
            let proposer_smc = smc_listener::SMCListener::new(proposer_smc_tx);

            // Start threads for both proposer and notary
            let mut proposer_thread = client_thread::ClientThread::new(&cli::config::Mode::Proposer);
            let mut notary_thread = client_thread::ClientThread::new(&cli::config::Mode::Notary);

            proposer_thread.run(proposer_smc_rx);
            notary_thread.run(notary_smc_rx);

            if cli::config_env::is_running_with_cargo_test() {
                thread::sleep(Duration::from_secs(1));
                let _p_result = proposer_thread.manager.unwrap().send(client_thread::Command::Terminate);
                let _n_result = notary_thread.manager.unwrap().send(client_thread::Command::Terminate);
            }

            // Wait for thread termination
            match proposer_thread.handle.unwrap().join() {
                Ok(x) => { debug!("Successful proposer thread join {:?}", x); () },
                Err(e) => { panic!("Failed proposer thread join {:?}", e); }
            }
            match notary_thread.handle.unwrap().join() {
                Ok(x) => { debug!("Successful notary thread join {:?}", x); () },
                Err(e) => { panic!("Failed notary thread join {:?}", e); }
            }
        }
    }
}

