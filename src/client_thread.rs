use std::sync::mpsc;
use std::thread;

use cli::config;
use notary;
use proposer;

/// A request to terminate a running thread
pub enum Command {
    Terminate
}

pub struct ClientThread {
    handle: Option<thread::JoinHandle<()>,
    mode: config::Mode,
    manager: mpsc::Sender<Command>
}

impl ClientThread {
    pub fn new(mode: config::Mode) -> ClientThread {
        match mode {
            config::Mode::Notary => { 
                let (notary_smc_sender, notary_smc_receiver) = mpsc::channel();
                let (notary_manager_sender, notary_manager_receiver) = mpsc::channel();
                let notary = notary::Notary::new(notary_smc_receiver, notary_manager_receiver);

                ClientThread {
                    handle: Some(thread::Builder::new()
                                    .name(cli::config::Mode::Notary.value())
                                    .spawn(move || {
                                    notary.run();
                                    })
                                    .expect("Failed to spawn a notary thread")),
                    mode, 
                    manager: notary_manager_sender
                }
            },
            config::Mode::Proposer => {
                let proposer = proposer::Proposer::new();

                ClientThread {
                    let (proposer_manager_sender, proposer_manager_receiver) = mpsc::channel();
                    handle: Some(thread::Builder::new()
                                .name(cli::config::Mode::Notary.value())
                                .spawn(move || {
                                    notary.run();
                                })
                                .expect("Failed to spawn a notary thread")),
                    mode,
                    manager: proposer_manager_sender
                }
            },
            _ => { eprintln!("A ClientThread should either be a proposer or a notary"); }
        }
    }
}