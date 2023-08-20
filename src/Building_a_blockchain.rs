use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};

#[derive(Debug)]
struct Block {
    index: u32,
    timestamp: DateTime<Utc>,
    data: String,
    previous_hash: String,
    hash: String,
}

impl Block {
    fn new(index: u32, timestamp: DateTime<Utc>, data: String, previous_hash: String) -> Self {
        let hash = format!("{:x}", Sha256::digest(data.as_bytes()));
        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }
}

fn main() {
    let genesis_block = Block::new(0, Utc::now(), "Genesis Block".to_owned(), "0".to_owned());
    println!("{:?}", genesis_block);
}