use collation::header;
use collation::body;
use collation::collation;

use ethereum_types;

pub enum Message {
    Selected{value: bool},
    ShardId{value: ethereum_types::U256},
    Collation{value: collation::Collation},
    Proposal{value: collation::Collation}
}