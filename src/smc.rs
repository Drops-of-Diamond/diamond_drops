#![allow(unused)]

/// Implemented from https://ethresear.ch/t/sharding-phase-1-spec/1407.

extern crate ethereum_types;
extern crate ethcore_bytes;

use self::ethereum_types::{U256, H32, H128, Address};
use self::ethcore_bytes::{Bytes};

//use std::i64;
use std::collections::HashMap;


fn main() {
	
	// Parameters

	// Shards
		// let SMC_ADDRESS = (TBD)
		let network_id = 0b1000_0001;
		let shard_count = 100;                // shards
		let period_length = 5;                // block times
		let lookahead_length = 4;             // periods
		let windback_length = 25;             // collations

	// Collations
		let collation_size = 2u8.pow(20);         // bytes
		let chunk_size = 32;                  // bytes
		let collator_subsidy = 0.001;         // vETH

	// Registries
		let collator_deposit = 1000;          // ETH
		let proposer_deposit = 1;             // ETH
		let min_proposer_balance = 0.1;       // ETH
		let collator_lockup_length = 16128;   // periods
		let proposer_lockup_length = 48;      // periods

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
		collator_pool: HashMap<H128, Address>,
			// This is an array of active collator addresses.
		//collator_pool_len: collator_pool.len(),
			// This is the size of the collator pool.
		empty_slots_stack: HashMap<H128, H128>,		
			// This is the stack of empty collator slot indices.
		//empty_slots_stack_top: empty_slots_stack.pop()[1],
			// This is the top index of the stack.
	}

	struct CollatorRegistry {
		deregistered: H128,
			// deregistered is 0 for not yet deregistered collators.
		pool_index: H128,
	}

	let mut collator_registry: HashMap<Address, CollatorRegistry> = 
		HashMap::new();

	impl CollatorRegistry {
		// Import msg.value and msg.sender
		// Checks:
	 	//	- msg.value >= COLLATOR_DEPOSIT
		// 	- collator_registry[msg.sender] does not exist
		pub fn register_collator(collator_address: Address) -> bool {
			collator_deposit
			
			if !empty_slots_stack.isempty() {
				let mut collator_pool_index = 
					CollatorPool::empty_slots_stack.pop()[1];
			} else {
				let mut collator_pool_index = 
					CollatorPool::collator_pool.len();
			}

			collator_registry.insert(0, collator_pool_index);
			
			CollatorPool::collator_pool.insert(collator_pool_index, collator_address}
		}
	}

	struct ProposerRegistry {
		deregistered: H128,
		balances: U256,
	}

	let mut proposer_registry: HashMap<Address, ProposerRegistry> =
		HashMap::new();
	
	struct Bytes32([u8; 4]);
	struct Bytes24([u8; 3]);

	struct CollationTrees {
		collation_trees: HashMap<U256, HashMap<Bytes24, Bytes32>>,// =
		//	HashMap::new();
			// The collation tree of a shard maps collation hashes to 
		 	// previous collation hashes truncated to 24 bytes packed into
			// a bytes32 with the collation height in the last 8 bytes).

		last_update_periods: HashMap<U256, H128> 
			// This is the period of the last update for each shard.
	}

	struct AvailabilityChallenges {
		// availability_challenges: TBD 
		availability_challenges_len: H128
			// availability challenges counter
	}
}
