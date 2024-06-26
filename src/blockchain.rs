use std::{cmp, io};
use std::fs::File;
use std::io::Write;
use std::time::{Duration, Instant};
use crate::block::Block;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Blockchain {
    pub(crate) chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let blockchain = Blockchain {
            chain: Vec::new()
        };
        blockchain
    }
    pub fn mine_latest(&mut self, index: Option<u64>, mut file: File) {
        let (data, difficulty, last_block) = match index {
            Some(idx) if idx >= 0 && idx < self.chain.len() as u64 => {
                println!("Recalculating block!");
                let this_block = &self.chain[idx as usize];
                let last_block = if idx >= 0 {
                    Some(&self.chain[idx as usize - 1])
                } else {
                    None
                };
                (this_block.data.clone(), this_block.difficulty, last_block)
            }
            _ => {
                println!("Please enter a difficulty for the block:");
                let mut difficulty = String::new();
                io::stdin().read_line(&mut difficulty).expect("error reading");
                let difficulty: u64 = difficulty.trim().parse().expect("Invalid input");
                println!("Please enter Data for new difficulty {} block!", difficulty);
                let mut data = String::new();
                io::stdin().read_line(&mut data).expect("error reading");
                let data = data.trim().to_string();
                let last_block = self.chain.last();
                (data, difficulty, last_block)
            }
        };
        let mut new_block = Block::new(data, difficulty, last_block);
        let correct_string: &str = &*"0".repeat(difficulty as usize);

        println!("Would you like to parallelize mining? (true/1 or false/0)");
        let mut parallelize_input = String::new();
        io::stdin().read_line(&mut parallelize_input).expect("error reading");
        let parallelize_input = parallelize_input.trim().to_lowercase();

        let parallelize = match parallelize_input.as_str() {
            "true" | "1" => true,
            "false" | "0" => false,
            _ => {
                println!("Invalid input. Defaulting to non-parallelized mining.");
                false
            }
        };

        if parallelize {
            let input = Arc::new(new_block.index.to_string()
                + &new_block.timestamp.to_string()
                + &new_block.data
                + &new_block.previous_hash);
            let num_threads = num_cpus::get();
            let found = Arc::new(AtomicBool::new(false));
            let found_nonce = Arc::new(std::sync::Mutex::new(None));
            println!("Mining block #{} of {} difficulty, {} threads used", new_block.index, difficulty, num_threads);
            let now = Instant::now(); // Start the timer
            let threads: Vec<_> = (0..num_threads)
                .map(|i| {
                    let input = Arc::clone(&input);
                    let found = Arc::clone(&found);
                    let found_nonce = Arc::clone(&found_nonce);
                    let correct_string = correct_string.to_owned();
                    thread::spawn(move || {
                        let mut current_nonce = i as i64;
                        let update_interval = Duration::from_millis(100);
                        let mut last_update = Instant::now();
                        loop {
                            if found.load(Ordering::Relaxed) {
                                break;
                            }
                            let hash = Block::calculate_hash(input.to_string() + &current_nonce.to_string());
                            if last_update.elapsed() >= update_interval {
                                println!("Thread: {} Nonce: {} Hash: {}", i + 1, current_nonce, hash);
                                last_update = Instant::now();
                            }
                            if hash.starts_with(&correct_string) {
                                found.store(true, Ordering::Relaxed);
                                *found_nonce.lock().unwrap() = Some(current_nonce);
                                break;
                            }
                            current_nonce += num_threads as i64;
                        }
                    })
                })
                .collect();

            for thread in threads {
                thread.join().unwrap();
            }
            let found_nonce_value = *found_nonce.lock().unwrap();
            match found_nonce_value {
                Some(nonce_value) => {
                    let timer: f64 = (now.elapsed().as_millis() as f64) / 1000f64;
                    println!("\nBlock Mined in {} Seconds with Parallelism!\nNonce: {}\nHash: {}", timer, nonce_value, Block::calculate_hash(input.to_string() + &nonce_value.to_string()));
                    new_block.nonce = nonce_value as u64;
                    self.chain.push(new_block);
                    let json = serde_json::to_string(&self.chain).unwrap();
                    println!("{:#?}", json);
                    file.write_all(json.as_bytes()).unwrap();
                }
                None => {
                    println!("No valid nonce found.");
                }
            }
        } else {
            let divisor = cmp::min(i64::pow(10, (difficulty - 2) as u32), 1000);
            let base_block = new_block.index.to_string()
                + &new_block.timestamp.to_string()
                + &new_block.data
                + &new_block.previous_hash;
            println!("Mining block #{} of {} difficulty", new_block.index, difficulty);
            let now = Instant::now();
            let mut nonce: i64 = 0;
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
            new_block.nonce = nonce as u64;
            self.chain.push(new_block);
            let json = serde_json::to_string(&self.chain).unwrap();
            println!("{:#?}", json);
            let name2 = format!("{:?}.json",file);
            let mut json_file = File::create(name2).unwrap();
            json_file.write_all(json.as_bytes()).unwrap();
        }
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
    pub fn remove_block(&mut self, index: usize,mut file: File) {
        if index < self.chain.len() {
            self.chain.remove(index);
            println!("Block removed successfully");
            for i in index .. self.chain.len(){
                println!("recalculating hash!");
                self.mine_latest(Some(i as u64),file.try_clone().unwrap());
            }
        } else {
            println!("Invalid block index");
        }
    }
}