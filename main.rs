extern crate ethereum_types;

use ethereum_types::{U256, Address};

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
		let COLLATION_SIZE = 2 ** 20;         // bytes
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
		shard_id:           uint256, // pointer to shard
		parent_hash:        bytes32, // pointer to parent header
		chunk_root:         bytes32, // pointer to collation body
		period:             int128,
		height:             int128,
		proposer_address:   Address,
		proposer_bid:       uint256,
		proposer_signature: bytes,
	}

	// ------------


	// ### SMC storage
	// The SMC has the following data structures held in storage:

	struct CollatorPool {
		collator_pool_len: int128,
			// size of the collator pool
		collator_pool: [Address; collator_pool_len], 
			// array of active collator addresses
		empty_slots_stack_depth: int128,
		empty_slots_stack: [int128; empty_slots_stack_depth],	
			// stack of empty collator slot indices
		empty_slots_stack_top: int128,		// top index of the stack
	}

	struct CollatorRegistry {
		deregistered: int128, 
			// deregistered is 0 for not yet deregistered collators
		pool_index: int128
	}//[Address] figure out how to accept address as an argument to struct

	struct ProposerRegistry {
		deregistered: int128,
		balances: U256,
	}//[Address]

}
