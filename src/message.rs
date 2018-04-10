use collation::header;
use collation::body;
use collation::collation;

pub enum Message {
    Eligible{value: bool},
    Shard{value: usize},
    Header{value: header::Header},
    Collation{value: collation::Collation},
    Proposal{value: collation::Collation}
}