use collation::header;
use collation::body;
use collation::collation;

use ethereum_types;

pub enum Message {
    Selected{value: bool},
    Shard{value: ethereum_types::U256},
    Header{value: header::Header},
    Collation{value: collation::Collation},
    Proposal{value: collation::Collation}
}