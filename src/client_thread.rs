use std::sync::mpsc;
use std::thread;

use cli::config;
use notary;
use proposer;
use message;

/// A request to terminate a running thread
pub enum Command {
    Terminate
}

/// A client thread to run either a notary or a proposer
pub struct ClientThread {
    pub handle: Option<thread::JoinHandle<()>>,
    mode: config::Mode,
    pub manager: Option<mpsc::Sender<Command>>
}

impl ClientThread {
    /// Creats a new thread to be run
    pub fn new(mode: &config::Mode) -> ClientThread {
        match *mode {
            config::Mode::Notary => { 
                ClientThread {
                    handle: None,
                    mode: mode.clone(), 
                    manager: None
                }
            },
            config::Mode::Proposer => {
                ClientThread {
                    handle: None,
                    mode: mode.clone(),
                    manager: None
                }
            },
            _ => { panic!() }
        }
    }

    /// Run the thread with the given receiver
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
            _ => { panic!() }
        }
    }
}
