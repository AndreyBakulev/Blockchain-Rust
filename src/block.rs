use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Block {
    pub index: i32,
    pub timestamp: u64,
    pub data: String,
    pub previous_hash: String,
    pub nonce: i64,
    pub difficulty: i32,
}

impl Block {
    pub fn new(data: String, difficulty: i32, previous_block: Option<&Block>) -> Self {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let (index, previous_hash) = match previous_block {
            Some(block) => (
                block.index + 1,
                Self::calculate_hash(
                    block.index.to_string()
                        + &block.timestamp.to_string()
                        + &block.data
                        + &block.previous_hash
                        + &block.nonce.to_string(),
                ),
            ),
            None => (0, "0".to_string()),
        };
        let block = Block {
            index,
            timestamp,
            data,
            previous_hash,
            nonce: 0,
            difficulty,
        };
        block
    }

    pub fn calculate_hash(raw_data: String) -> String {
        let mut hasher = Sha256::new();
        hasher.update(raw_data);
        let hash = hasher.finalize();
        hex::encode(hash)
    }

    pub fn print_block(&self) {
        println!("--------------------------------");
        println!("Index: {}", self.index);
        println!("Timestamp: {}", self.timestamp);
        println!("Data: {}", self.data);
        println!("Previous Hash: {}", self.previous_hash);
        println!("Nonce: {}", self.nonce);
        println!("--------------------------------");
    }
}