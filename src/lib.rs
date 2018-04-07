// Module declarations
pub mod cli;
mod proposer;
mod collator;

use std::thread;

enum ThreadName {
    Proposer,
    Collator
}

impl ThreadName {
    fn value(&self) -> String {
        match *self {
            ThreadName::Proposer => "proposer".to_string(),
            ThreadName::Collator => "collator".to_string(),
        }
    }
}

pub fn run(config: cli::config::Config) {
    /// The main function to run the node.  
    /// 
    /// # Inputs
    /// 
    /// config - A struct containing the configuration values for the client
    
    println!("Client Mode: {:?}", config.mode);

    let proposer = proposer::Proposer::new();
    let collator = collator::Collator::new();

    let mut proposer_handle: Option<thread::JoinHandle<()>> = None;
    let mut collator_handle: Option<thread::JoinHandle<()>> = None;

    match config.mode {
        cli::config::Mode::Proposer => {
            println!("Running as a proposer");
            // Start a thread to run the proposer
            proposer_handle = Some(thread::Builder::new()
                .name(ThreadName::Proposer.value())
                .spawn(move || {
                    proposer.run();
                }).unwrap()
            );
        },
        cli::config::Mode::Collator => {
            println!("Running as a collator");
            // Start a thread to run the collator
            collator_handle = Some(thread::Builder::new()
                .name(ThreadName::Collator.value())
                .spawn(move || {
                    collator.run();
                }).unwrap()
            );
        },
        cli::config::Mode::Both => {
            println!("Running as both a proposer and collator");
            // Start threads for both proposer and collator
            proposer_handle = Some(thread::Builder::new()
                .name(ThreadName::Proposer.value())
                .spawn(move || {
                    proposer.run();
                }).unwrap()
            );
            collator_handle = Some(thread::Builder::new()
                .name(ThreadName::Collator.value())
                .spawn(move || {
                    collator.run();
                }).unwrap()
            );
        }
    }

    if let Some(handle) = proposer_handle {
        handle.join();
    }

    if let Some(handle) = collator_handle {
        handle.join();
    }
}


