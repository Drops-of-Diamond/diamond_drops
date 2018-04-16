use collation::collation;

use ethereum_types;

/// A message from the SMC Listener
pub enum Message {
    Selected{value: bool},
    ShardId{value: ethereum_types::U256},
    Collation{value: collation::Collation},
    Proposal{value: collation::Collation}
}