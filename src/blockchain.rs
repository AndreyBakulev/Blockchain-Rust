use std::{cmp, io};
use crate::block::Block;

pub struct Blockchain {
    chain: Vec<Block>,
    difficulty: i32,
}

impl Blockchain {
    pub fn new(difficulty: i32) -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            difficulty,
        };
        blockchain.create_genesis_block();
        blockchain
    }

    fn create_genesis_block(&mut self) {
        let genesis_block = Block::new("Genesis Block".to_string(), None);
        self.chain.push(genesis_block);
        println!("genesis block created!");
    }

    pub fn mine_latest(&mut self) {
        let mut nonce = 0;
        println!("your difficulty is: {}", self.difficulty as usize);
        println!("Please enter Data for new block!");
        let mut data = String::new();
        io::stdin().read_line(&mut data).expect("error reading");
        let divisor = cmp::min(i32::pow(10, self.difficulty as u32) / 100, 10000);
        let new_block = Block::new(data, self.chain.last());
        let correct_string = "0".repeat(self.difficulty as usize);
        let previous_block = self.chain.last().unwrap();
        let base_block = previous_block.index.to_string() + &*previous_block.timestamp.to_string() + &*previous_block.data.to_string() + &*previous_block.nonce.to_string() + &*previous_block.previous_hash.to_string();
        println!("Mining block #{} of {} difficulty",new_block.index, self.difficulty);
        loop {
            let h1 = base_block.clone();
            //find way to not have h1 and use base_block instead
            let hash: String = Block::calculate_hash(h1 + &*nonce.to_string());
            if hash[..self.difficulty as usize] == correct_string {
                break;
            }
            nonce += 1;
            if nonce % divisor == 0
            {
                print!("\r#{}, Hash: {}", nonce, hash);
            }
        }
        self.chain.push(new_block);
        println!(" \n Block mined successfully!");
    }

    pub fn validate_chain(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            let previous_hash = Block::calculate_hash(
                previous_block.index.to_string() + &*previous_block.timestamp.to_string() + &*previous_block.data.to_string() + &*previous_block.nonce.to_string() + &*previous_block.previous_hash.to_string()
            );

            if current_block.previous_hash != previous_hash {
                return false;
            }
        }
        true
    }

    pub fn retrieve_block(&self, index: usize) {
        if index < self.chain.len() {
            let block = &self.chain[index];
            block.print_block();
        } else {
            println!("Invalid block index");
        }
    }

    pub fn set_difficulty(&mut self, difficulty: i32) {
        self.difficulty = difficulty;
        println!("Difficulty set to {}", difficulty);
    }

    pub fn remove_block(&mut self, index: usize) {
        if index < self.chain.len() {
            self.chain.remove(index);
            println!("Block removed successfully");
        } else {
            println!("Invalid block index");
        }
    }
}