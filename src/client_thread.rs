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

/// A client thread to run either a notary or a proposer
pub struct ClientThread {
    mode: config::Mode,
    pub manager: Option<mpsc::Sender<Command>>,
    pub handle: Option<thread::JoinHandle<()>>
}

impl ClientThread {
    /// Creats a new thread to be run
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
            _ => { panic!("Invalid mode provided to spawn new child thread from client thread instance") }
        }
    }
}
