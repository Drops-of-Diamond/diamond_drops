// Adapted from Python from https://github.com/ethereum/py-evm/pull/555/files.

use ethereum_types::U256;
use constants;
use std::ops::{Generator, GeneratorState};

/*#[derive(PartialEq, Debug, Clone)]
/// A collation body of 2^15 chunks with 32 bytes (256 bits) per chunk.
pub struct Body {
    body: Vec<ethereum_types::U256; constants::CHUNKS_PER_COLLATION>
}

pub impl Body {
    pub fn new() -> Body {
        Body {body = [[0x00, constants::CHUNK_SIZE], constants::CHUNKS_PER_COLLATION]}
    }
}

impl Iterator for Body {
    type Item = Vec<ethereum_types::U256; constants::CHUNKS_PER_COLLATION>;

    fn check_body_size(self.body) -> <self.body, Err> {
        if self.body.len() != COLLATION_SIZE {
            //error!("{} is not equal to {}", self.body.len(), COLLATION_SIZE)
            //process::exit(1);
        }
        // I'm not sure if there's any advantage to writing it this way instead
        // of assert_eq!(self.body.len(), COLLATION_SIZE).
    }

    #![feature(iterator_step_by)] // nightly, for step_by()
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.step_by
    #![feature(generators, generator_trait)] // nightly, for yield
    // https://doc.rust-lang.org/std/ops/trait.Generator.html
    fn next(&mut self) -> Option<Self::Item> {
        check_body_size(self.body)
        for chunk_start in (0..self.body.len()).step_by(CHUNK_SIZE) {
            let mut generator = || {
                yield self.body[chunk_start:chunk_start + CHUNK_SIZE];
                // Make this also return; we just want to return the last chunk.
            };

            match generator.resume() {
                GeneratorState::Yielded(self.body[chunk_start:chunk_start + CHUNK_SIZE]) => {}
                _ => ()//panic!("unexpected return from resume"),
            }
            match generator.resume() {
                GeneratorState::Complete(self.body[chunk_start:chunk_start + CHUNK_SIZE]) => {}
                _ => ()//panic!("unexpected return from resume"),
            }
        }
    }


}*/