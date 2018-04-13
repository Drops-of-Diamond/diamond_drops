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
            // Start a thread to run the proposer
            let mut proposer_thread = client_thread::ClientThread::new(&config.mode);
            proposer_thread.run();

            // TODO make this part only run when running 'cargo test'
            thread::sleep(Duration::from_secs(1));
            proposer_thread.manager.unwrap().send(client_thread::Command::Terminate);

            // Wait for thread termination
            match proposer_thread.handle.unwrap().join() {
                Ok(x) => { println!("Successful proposer thread join {:?}", x); () },
                Err(e) => { panic!("Failed proposer thread join {:?}", e); }
            }
        },
        cli::config::Mode::Notary => {
            println!("Running as a notary");
            // Start a thread to run the notary
            let mut notary_thread = client_thread::ClientThread::new(&config.mode);
            notary_thread.run();

            // TODO make this part only run when running 'cargo test'
            thread::sleep(Duration::from_secs(1));
            notary_thread.manager.unwrap().send(client_thread::Command::Terminate);

            // Wait for thread termination
            match notary_thread.handle.unwrap().join() {
                Ok(x) => { println!("Successful notary thread join {:?}", x); () },
                Err(e) => { panic!("Failed notary thread join {:?}", e); }
            }
        },
        cli::config::Mode::Both => {
            println!("Running as both a proposer and notary");
            // Start threads for both proposer and notary
            let mut proposer_thread = client_thread::ClientThread::new(&cli::config::Mode::Proposer);
            let mut notary_thread = client_thread::ClientThread::new(&cli::config::Mode::Notary);

            proposer_thread.run();
            notary_thread.run();

            // TODO make this part only run when running 'cargo test'
            thread::sleep(Duration::from_secs(1));
            proposer_thread.manager.unwrap().send(client_thread::Command::Terminate);
            notary_thread.manager.unwrap().send(client_thread::Command::Terminate);

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


