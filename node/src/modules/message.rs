use modules::collation::collation;
use modules::primitives::{ShardIdHash};

#[derive(Debug)]
/// A message from the SMC Listener
pub enum Message {
    Selected{value: bool},
    ShardId{value: ShardIdHash},
    Collation{value: collation::Collation},
    Proposal{value: collation::Collation}
}