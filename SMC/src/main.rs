extern crate ethereum_types;
extern crate ethcore_bytes;

use ethereum_types::{U256, H32, H128, Address};
use ethcore_bytes::{Bytes};

//use std::i64;
use std::collections::HashMap;

fn main() {
	
	// Parameters

	// Shards
		// let SMC_ADDRESS = (TBD)
		let mut NETWORK_ID = 0b1000_0001;
		let SHARD_COUNT = 100;                // shards
		let PERIOD_LENGTH = 5;                // block times
		let LOOKAHEAD_LENGTH = 4;             // periods
		let WINDBACK_LENGTH = 25;             // collations

	// Collations
		let COLLATION_SIZE = 2u8.pow(20);         // bytes
		let CHUNK_SIZE = 32;                  // bytes
		let COLLATOR_SUBSIDY = 0.001;         // vETH

	// Registries
		let COLLATOR_DEPOSIT = 1000;          // ETH
		let PROPOSER_DEPOSIT = 1;             // ETH
		let MIN_PROPOSER_BALANCE = 0.1;       // ETH
		let COLLATOR_LOCKUP_LENGTH = 16128;   // periods
		let PROPOSER_LOCKUP_LENGTH = 48;      // periods

	// ------------

	// Sharding participants have light-client access to collation 
	// headers via the HeaderAdded logs produced by the addHeader 
	// method. The header fields are:

	struct CollationHeader {
		shard_id:           U256, // pointer to shard
		parent_hash:        H32, // pointer to parent header
		chunk_root:         H32, // pointer to collation body
		period:             H128,
		height:             H128,
		proposer_address:   Address,
		proposer_bid:       U256,
		proposer_signature: Bytes,
	}

	// ------------

	// ### SMC storage
	// The SMC has the following data structures held in storage:

	struct CollatorPool {
		collator_pool: HashMap<Address, H128>,			// array of active collator addresses
		collator_pool_len: H128,				// size of the collator pool
		empty_slots_stack_depth: H128,
		empty_slots_stack: HashMap<H128, H128>,	// stack of empty collator slot indices
		empty_slots_stack_top: H128,				// top index of the stack
	}

	struct CollatorRegistry {
		deregistered: H128, 			// deregistered is 0 for not yet deregistered collators
		pool_index: H128,
	}//[Address] figure out how to accept address as an argument to struct

	struct ProposerRegistry {
		deregistered: H128,
		balances: U256,
	}//[Address]

}
