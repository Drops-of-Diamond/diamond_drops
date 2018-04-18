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
IN NO EVENT SHALL THE James Ray BE LIABLE FOR ANY CLAIM, DAMAGES OR
OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.

For more information, please refer to <http://unlicense.org>
use collation;
use message;

use std::sync::mpsc;
use std::collections::HashMap;

pub struct Notary {
    eligible: bool,
    shard_id: usize,
    collation_trees: HashMap<usize, collation::tree::Tree>,
    listener: mpsc::Receiver<message::Message>,
}

impl Notary {
    pub fn new(listener: mpsc::Receiver<message::Message>) -> Notary {
        Notary {
            eligible: false,
            shard_id: 0,
            collation_trees: HashMap::new(),
            listener,
        }
    }

    pub fn run(&mut self) {
        loop {
            // Get message from the SMC listener
            let msg_result = self.listener.recv();
            let msg: message::Message;

            match msg_result {
                Ok(m) => { msg = m }
                Err(e) => { eprintln!("Error receiving message from SMC listener"); continue }
            }

            // Respond to SMC listener message
            match msg {
                message::Message::Eligible { value } => { self.eligible = value; }
                message::Message::Shard { value } => { self.shard_id = value; },
                message::Message::Header { value } => { self.store_collation_header(value); },
                message::Message::Collation { value } => { self.store_collation(value); },
                message::Message::Proposal { value } => { self.store_proposal(value); }
            }

            if self.eligible {
                self.select_proposal();
                self.add_header();
            }
        }
    }

    fn store_collation_header(&self, header: collation::header::Header) {}

    fn store_collation(&self, collation: collation::collation::Collation) {}

    fn store_proposal(&self, collation: collation::collation::Collation) {}

    fn select_proposal(&self) {}

    fn add_header(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;
}
