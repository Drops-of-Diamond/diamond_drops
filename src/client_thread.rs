use cli::config;
use notary;
use proposer;
use message;

use std::sync::mpsc;
use std::thread;

/// A request to terminate a running thread
#[derive(Debug)]
pub enum Command {
    Terminate
}

pub struct ClientThread {
    mode: config::Mode,
    pub manager: Option<mpsc::Sender<Command>>,
    pub handle: Option<thread::JoinHandle<()>>
}

impl ClientThread {
    pub fn new(mode: &config::Mode) -> ClientThread {
        match *mode {
            config::Mode::Notary => { 
                ClientThread {
                    mode: mode.clone(),
                    manager: None,
                    handle: None
                }
            },
            config::Mode::Proposer => {
                ClientThread {
                    mode: mode.clone(),
                    manager: None,
                    handle: None
                }
            },
            _ => { panic!("Invalid mode provided to generate client thread instance"); }
        }
    }

    pub fn run(&mut self, smc_receiver: mpsc::Receiver<message::Message>) {
        match self.mode {
            config::Mode::Notary => {
                let (notary_manager_sender, notary_manager_receiver) = mpsc::channel();
                let mut notary = notary::Notary::new(smc_receiver, notary_manager_receiver);

                self.manager = Some(notary_manager_sender);
                self.handle = Some(thread::Builder::new()
                                    .name(config::Mode::Notary.value())
                                    .spawn(move || {
                                        notary.run();
                                    })
                                    .expect("Failed to spawn a notary thread"));
            },
            config::Mode::Proposer => {
                let (proposer_manager_sender, proposer_manager_receiver) = mpsc::channel();
                let mut proposer = proposer::Proposer::new();

                self.manager = Some(proposer_manager_sender);
                self.handle = Some(thread::Builder::new()
                                .name(config::Mode::Proposer.value())
                                .spawn(move || {
                                    proposer.run();
                                })
                                .expect("Failed to spawn a proposer thread"));
            }
            _ => { panic!("Invalid mode provided to spawn new child thread from client thread instance") }
        }
    }
}
