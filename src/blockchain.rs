use rayon::prelude::*;
use std::{cmp, io};
use std::time::Instant;
use crate::block::Block;

pub struct Blockchain {
    chain: Vec<Block>
}

impl Blockchain {
    pub fn new() -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new()
        };
        blockchain.create_genesis_block();
        blockchain
    }

    fn create_genesis_block(&mut self) {
        let genesis_block = Block::new("Genesis Block".to_string(), 4,None);
        self.chain.push(genesis_block);
        println!("genesis block (diff: 4) created!");
    }

    pub fn mine_latest(&mut self) {
        let mut nonce = 0;
        println!("Please enter a difficulty for the block:");
        let mut difficulty = String::new();
        io::stdin().read_line(&mut difficulty).expect("error reading");
        let difficulty: i32 = difficulty.trim().parse().expect("Invalid input");
        println!("your difficulty is: {}", difficulty);
        println!("Please enter Data for new block!");
        let mut data = String::new();
        io::stdin().read_line(&mut data).expect("error reading");
        let divisor = cmp::min(i32::pow(10, difficulty as u32) / 100, 10000);
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
        new_block.nonce = nonce as i64;
        self.chain.push(new_block);
    }
    pub fn mine_latest_parallel(&mut self) {
        println!("Please enter a difficulty for the block:");
        let mut difficulty = String::new();
        io::stdin().read_line(&mut difficulty).expect("error reading");
        let difficulty: i32 = difficulty.trim().parse().expect("Invalid input");
        println!("your difficulty is: {}", difficulty);
        println!("Please enter Data for new block!");
        let mut data = String::new();
        io::stdin().read_line(&mut data).expect("error reading");

        let mut new_block = Block::new(data, difficulty, self.chain.last());
        let correct_string: &str = &*"0".repeat(difficulty as usize);

        let previous_block = self.chain.last().unwrap();
        let base_block = previous_block.index.to_string()
            + &*previous_block.timestamp.to_string()
            + &*previous_block.data.to_string()
            + &*previous_block.nonce.to_string()
            + &*previous_block.previous_hash.to_string();

        println!("Mining block #{} of {} difficulty", new_block.index, difficulty);
        let now = Instant::now();

        let (found_nonce, hash) = (0..i32::MAX).into_par_iter().find_any(|&n| {
            let hash: String = Block::calculate_hash(base_block.clone() + &*n.to_string());
            hash.starts_with(correct_string)
        }).unwrap();

        let timer: f64 = (now.elapsed().as_millis() as f64) / 1000f64;
        println!(
            "\nBlock Mined in {} Seconds!\nNonce: {}\nHash: {}",
            timer, found_nonce, hash
        );
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