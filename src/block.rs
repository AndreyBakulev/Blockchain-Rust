use chrono::prelude::*;
use serde::{ Serialize, Deserialize };
use sha2::Digest;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub index: u32,
    pub timestamp: i64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u32,
}

impl Block {
    pub fn new(index: u32, transactions: Vec<Transaction>, previous_hash: String) -> Self {
        let now = Utc::now();
        let (nonce, hash) = mine_hash(index, now.timestamp(), &transactions, &previous_hash);
        Self {
            index,
            timestamp: now.timestamp(),
            transactions,
            previous_hash,
            hash,
            nonce,
        }
    }
}

fn mine_hash(index: u32, timestamp: i64, transactions: &[Transaction], previous_hash: &str) -> (u32, String) {
    let mut nonce = 0;

    loop {
        let data = serde_json::json!({
            "index": index,
            "timestamp": timestamp,
            "transactions": transactions,
            "previous_hash": previous_hash,
            "nonce": nonce
        });

        let hash = hex::encode(sha2::Sha256::digest(data.to_string().as_bytes()));

        if &hash[..4] == "0000" {
            return (nonce, hash);
        }

        nonce += 1;
    }
}