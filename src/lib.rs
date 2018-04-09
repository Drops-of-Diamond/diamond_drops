//! ![uml](ml.svg)

// External crates
extern crate ethereum_types;

// Module declarations
pub mod cli;
pub mod proposer;
pub mod collator;
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

    // Create the channel for the collator and smc listener
    let (collator_sender, collator_receiver) = mpsc::channel();

    // Create the SMC listener
    let smc_listener = smc_listener::SMCListener::new(collator_sender);

    // Create the proposer and collator
    let mut proposer = proposer::Proposer::new();
    let mut collator = collator::Collator::new(collator_receiver);

    // Get thread handles
    let mut proposer_handle: Option<thread::JoinHandle<()>> = None;
    let mut collator_handle: Option<thread::JoinHandle<()>> = None;

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
        cli::config::Mode::Collator => {
            println!("Running as a collator");
            // Start a thread to run the collator
            collator_handle = Some(thread::Builder::new()
                .name(cli::config::Mode::Collator.value())
                .spawn(move || {
                    collator.run();
                })
                .expect("Failed to spawn a collator thread")
            );
        },
        cli::config::Mode::Both => {
            println!("Running as both a proposer and collator");
            // Start threads for both proposer and collator
            proposer_handle = Some(thread::Builder::new()
                .name(cli::config::Mode::Proposer.value())
                .spawn(move || {
                    proposer.run();
                })
                .expect("Failed to spawn a proposer thread")
            );
            collator_handle = Some(thread::Builder::new()
                .name(cli::config::Mode::Collator.value())
                .spawn(move || {
                    collator.run();
                })
                .expect("Failed to spawn a collator thread")
            );
        }
    }

    if let Some(handle) = proposer_handle {
        match handle.join() {
            Ok(x) => { println!("Successful proposer thread join {:?}", x); () },
            Err(e) => { panic!("Failed proposer thread join {:?}", e); }
        }
    }

    if let Some(handle) = collator_handle {
        match handle.join() {
            Ok(x) => { println!("Successful collator thread join {:?}", x); () },
            Err(e) => { panic!("Failed collator thread join {:?}", e); }
        }
    }
}


