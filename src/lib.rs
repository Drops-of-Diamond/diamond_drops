//! ![uml](ml.svg)

// External crates
extern crate ethereum_types;
extern crate tiny_keccak;

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

pub fn run(config: cli::config::Config) -> () {
    /// The main function to run the node.  
    /// 
    /// # Inputs
    /// 
    /// config - A struct containing the configuration values for the client
    
    println!("Client Mode: {:?}", config.mode);

    match config.mode {
        cli::config::Mode::Proposer => {
            println!("Running as a proposer");

            // Create the SMC Listener
            let (smc_tx, smc_rx) = mpsc::channel();
            let smc = smc_listener::SMCListener::new(smc_tx);

            // Start a thread to run the proposer
            let mut proposer_thread = client_thread::ClientThread::new(&config.mode);
            proposer_thread.run(smc_rx);

            // TODO make this part only run when running 'cargo test'
            thread::sleep(Duration::from_secs(1));
            let _result = proposer_thread.manager.unwrap().send(client_thread::Command::Terminate);

            // Wait for thread termination
            match proposer_thread.handle.unwrap().join() {
                Ok(x) => { println!("Successful proposer thread join {:?}", x); () },
                Err(e) => { panic!("Failed proposer thread join {:?}", e); }
            }
        },
        cli::config::Mode::Notary => {
            println!("Running as a notary");

            // Create the SMC Listener
            let (smc_tx, smc_rx) = mpsc::channel();
            let smc = smc_listener::SMCListener::new(smc_tx);

            // Start a thread to run the notary
            let mut notary_thread = client_thread::ClientThread::new(&config.mode);
            notary_thread.run(smc_rx);

            // TODO make this part only run when running 'cargo test'
            thread::sleep(Duration::from_secs(1));
            let _result = notary_thread.manager.unwrap().send(client_thread::Command::Terminate);

            // Wait for thread termination
            match notary_thread.handle.unwrap().join() {
                Ok(x) => { println!("Successful notary thread join {:?}", x); () },
                Err(e) => { panic!("Failed notary thread join {:?}", e); }
            }
        },
        cli::config::Mode::Both => {
            println!("Running as both a proposer and notary");

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

            // TODO make this part only run when running 'cargo test'
            thread::sleep(Duration::from_secs(1));
            let _p_result = proposer_thread.manager.unwrap().send(client_thread::Command::Terminate);
            let _n_result = notary_thread.manager.unwrap().send(client_thread::Command::Terminate);

            // Wait for thread termination
            match proposer_thread.handle.unwrap().join() {
                Ok(x) => { println!("Successful proposer thread join {:?}", x); () },
                Err(e) => { panic!("Failed proposer thread join {:?}", e); }
            }
            match notary_thread.handle.unwrap().join() {
                Ok(x) => { println!("Successful notary thread join {:?}", x); () },
                Err(e) => { panic!("Failed notary thread join {:?}", e); }
            }
        }
    }
}


