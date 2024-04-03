use rayon::prelude::*;
use std::{cmp, io};
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use crate::block::Block;

pub struct Blockchain {
    chain: Vec<Block>
}

impl Blockchain {
    pub fn new(difficulty: i32) -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new()
        };
        blockchain.create_genesis_block(difficulty);
        blockchain
    }

    fn create_genesis_block(&mut self, difficulty: i32) {
        let genesis_block = Block::new("Genesis Block".to_string(), difficulty,None);
        self.chain.push(genesis_block);
        println!("genesis block (diff: {}) created!", difficulty);
    }

    pub fn mine_latest(&mut self) {
        let mut nonce: i64 = 0;
        println!("Please enter a difficulty for the block:");
        let mut difficulty = String::new();
        io::stdin().read_line(&mut difficulty).expect("error reading");
        let difficulty: i32 = difficulty.trim().parse().expect("Invalid input");
        println!("Please enter Data for new difficulty {} block!", difficulty);
        let mut data = String::new();
        io::stdin().read_line(&mut data).expect("error reading");
        let divisor = cmp::min(i32::pow(10, difficulty as u32) / 100, 10000) as i64;

        let mut new_block = Block::new(data,difficulty,self.chain.last());
        let correct_string: &str = &*"0".repeat(difficulty as usize);
        let previous_block = self.chain.last().unwrap();
        let base_block = previous_block.index.to_string() + &*previous_block.timestamp.to_string() + &*previous_block.data.to_string() + &*previous_block.nonce.to_string() + &*previous_block.previous_hash.to_string();
        println!("Mining block #{} of {} difficulty",new_block.index, difficulty);
        let now = Instant::now();
        loop {
            let h1 = base_block.clone();
            //find way to not have h1 and use base_block instead
            let hash: String = Block::calculate_hash(h1 + &*nonce.to_string());
            if hash.starts_with(correct_string) {
                let timer: f64 = (now.elapsed().as_millis() as f64) / 1000f64;
                println!("\nBlock Mined in {} Seconds!\nNonce: {}\nHash: {}", timer, nonce, hash);
                break;
            }
            if nonce % divisor == 0
            {
                print!("\r#{}, Hash: {}", nonce, hash);
            }
            nonce += 1;
        }
        new_block.nonce = nonce;
        self.chain.push(new_block);
    }
    pub fn mine_latest_parallel(&mut self) {
        println!("Please enter a difficulty for the block:");
        let mut difficulty = String::new();
        io::stdin().read_line(&mut difficulty).expect("error reading");
        let difficulty: i32 = difficulty.trim().parse().expect("Invalid input");
        println!("Please enter Data for new difficulty {} block!", difficulty);
        let mut data = String::new();
        io::stdin().read_line(&mut data).expect("error reading");
        let correct_string: &str = &*"0".repeat(difficulty as usize);
        let previous_block = self.chain.last().unwrap();
        let base_block = previous_block.index.to_string()
            + &*previous_block.timestamp.to_string()
            + &*previous_block.data.to_string()
            + &*previous_block.nonce.to_string()
            + &*previous_block.previous_hash.to_string();
        let base_block_clone = base_block.clone();
        let divisor = cmp::min(i32::pow(10, difficulty as u32) / 100, 10000) as i64;
        println!("Mining block #{} of {} difficulty", self.chain.len(), difficulty);
        let now = Instant::now();
        let hash = Arc::new(Mutex::new(String::new()));
        let found_nonce = (0..i64::MAX)
            .into_par_iter()
            .inspect({
                let hash = Arc::clone(&hash);
                let base_block = base_block_clone.clone();
                move |&n| {
                    if n % 100000 == 0 {
                        let current_hash = Block::calculate_hash(base_block.clone() + &*n.to_string());
                        *hash.lock().unwrap() = current_hash.clone();
                        println!("#{}, Hash: {}", n, current_hash);
                        io::stdout().flush().unwrap();
                    }
                }
            })
            .find_any(|&n| {
                let current_hash: String = Block::calculate_hash(base_block_clone.clone() + &*n.to_string());
                current_hash.starts_with(correct_string)
            })
            .unwrap();

        let timer: f64 = (now.elapsed().as_millis() as f64) / 1000f64;
        println!(
            "\nBlock Mined in {} Seconds with Parallelism!\nNonce: {}\nHash: {}",
            timer,
            found_nonce,
            hash.lock().unwrap()
        );
        let mut new_block = Block::new(data, difficulty, self.chain.last());
        new_block.nonce = found_nonce;
        self.chain.push(new_block);
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
    pub fn remove_block(&mut self, index: usize) {
        if index < self.chain.len() {
            self.chain.remove(index);
            println!("Block removed successfully");
        } else {
            println!("Invalid block index");
        }
    }
}