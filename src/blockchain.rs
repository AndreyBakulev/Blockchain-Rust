use rayon::prelude::*;
use std::{cmp, io};
use std::time::Instant;
use crate::block::Block;

pub struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new()
        };
        blockchain
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
        let mut new_block = Block::new(data, difficulty, self.chain.last());
        let correct_string: &str = &*"0".repeat(difficulty as usize);
        let base_block = new_block.index.to_string()
            + &new_block.timestamp.to_string()
            + &new_block.data
            + &new_block.previous_hash;
        //println!("Base: {}", base_block);
        println!("Mining block #{} of {} difficulty", new_block.index, difficulty);
        let now = Instant::now();
        loop {
            let hash: String = Block::calculate_hash(base_block.clone() + &*nonce.to_string());
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
        let mut nonce: i64 = 0;
        println!("Please enter a difficulty for the block:");
        let mut difficulty = String::new();
        io::stdin().read_line(&mut difficulty).expect("error reading");
        let difficulty: i32 = difficulty.trim().parse().expect("Invalid input");
        println!("Please enter Data for new difficulty {} block!", difficulty);
        let mut data = String::new();
        io::stdin().read_line(&mut data).expect("error reading");
        let divisor = cmp::min(i32::pow(10, difficulty as u32) / 100, 10000) as i64;
        let mut new_block = Block::new(data, difficulty, self.chain.last());
        let correct_string: &str = &*"0".repeat(difficulty as usize);
        let base_block = new_block.index.to_string()
            + &new_block.timestamp.to_string()
            + &new_block.data
            + &new_block.previous_hash;
        //println!("Base: {}", base_block);
        println!("Mining block #{} of {} difficulty", new_block.index, difficulty);
        let now = Instant::now();
        loop {
            let hash: String = Block::calculate_hash(base_block.clone() + &*nonce.to_string());
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
    pub fn validate_chain(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];
            let previous_hash = Block::calculate_hash(
                previous_block.index.to_string() + &*previous_block.timestamp.to_string() + &*previous_block.data.to_string() + &*previous_block.previous_hash.to_string() + &*previous_block.nonce.to_string()
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