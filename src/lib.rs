// External crates
extern crate ethereum_types;

// Module declarations
pub mod cli;
pub mod proposer;
pub mod collator;
pub mod smc_listener;

use std::thread;

pub fn run(config: cli::config::Config) {
    /// The main function to run the node.  
    /// 
    /// # Inputs
    /// 
    /// config - A struct containing the configuration values for the client
    
    println!("Client Mode: {:?}", config.mode);

    let mut proposer = proposer::Proposer::new();
    let mut collator = collator::Collator::new();

    let mut proposer_handle: Option<thread::JoinHandle<()>> = None;
    let mut collator_handle: Option<thread::JoinHandle<()>> = None;

    match config.mode {
        cli::config::Mode::Proposer => {
            println!("Running as a proposer");
            // Start a thread to run the proposer
            proposer_handle = Some(thread::spawn(move || {
                proposer.run();
            }));
        },
        cli::config::Mode::Collator => {
            println!("Running as a collator");
            // Start a thread to run the collator
            collator_handle = Some(thread::spawn(move || {
                collator.run();
            }));
        },
        cli::config::Mode::Both => {
            println!("Running as both a proposer and collator");
            // Start threads for both proposer and collator
            proposer_handle = Some(thread::spawn(move || {
                proposer.run();
            }));
            collator_handle = Some(thread::spawn(move || {
                collator.run();
            }));
        }
    }

    if let Some(handle) = proposer_handle {
        handle.join();
    }

    if let Some(handle) = collator_handle {
        handle.join();
    }
}


