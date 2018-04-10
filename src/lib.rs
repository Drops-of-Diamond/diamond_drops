//! ![uml](ml.svg)

// External crates
extern crate ethereum_types;

// Module declarations
pub mod cli;
pub mod proposer;
pub mod notary;
pub mod smc_listener;
pub mod collation;
pub mod message;

use std::thread;
use std::sync::mpsc;

pub fn run(config: cli::config::Config) -> () {
    /// The main function to run the node.  
    /// 
    /// # Inputs
    /// 
    /// config - A struct containing the configuration values for the client
    
    println!("Client Mode: {:?}", config.mode);

    // Create the channel for the notary and smc listener
    let (notary_sender, notary_receiver) = mpsc::channel();

    // Create the SMC listener
    let smc_listener = smc_listener::SMCListener::new(notary_sender);

    // Create the proposer and notary
    let mut proposer = proposer::Proposer::new();
    let mut notary = notary::Notary::new(notary_receiver);

    // Get thread handles
    let mut proposer_handle: Option<thread::JoinHandle<()>> = None;
    let mut notary_handle: Option<thread::JoinHandle<()>> = None;

    match config.mode {
        cli::config::Mode::Proposer => {
            println!("Running as a proposer");
            // Start a thread to run the proposer
            proposer_handle = Some(thread::Builder::new()
                .name(cli::config::Mode::Proposer.value())
                .spawn(move || {
                    proposer.run();
                })
                .expect("Failed to spawn a proposer thread")
            );
        },
        cli::config::Mode::Notary => {
            println!("Running as a notary");
            // Start a thread to run the notary
            notary_handle = Some(thread::Builder::new()
                .name(cli::config::Mode::Notary.value())
                .spawn(move || {
                    notary.run();
                })
                .expect("Failed to spawn a notary thread")
            );
        },
        cli::config::Mode::Both => {
            println!("Running as both a proposer and notary");
            // Start threads for both proposer and notary
            proposer_handle = Some(thread::Builder::new()
                .name(cli::config::Mode::Proposer.value())
                .spawn(move || {
                    proposer.run();
                })
                .expect("Failed to spawn a proposer thread")
            );
            notary_handle = Some(thread::Builder::new()
                .name(cli::config::Mode::Notary.value())
                .spawn(move || {
                    notary.run();
                })
                .expect("Failed to spawn a notary thread")
            );
        }
    }

    if let Some(handle) = proposer_handle {
        match handle.join() {
            Ok(x) => { println!("Successful proposer thread join {:?}", x); () },
            Err(e) => { panic!("Failed proposer thread join {:?}", e); }
        }
    }

    if let Some(handle) = notary_handle {
        match handle.join() {
            Ok(x) => { println!("Successful notary thread join {:?}", x); () },
            Err(e) => { panic!("Failed notary thread join {:?}", e); }
        }
    }
}


