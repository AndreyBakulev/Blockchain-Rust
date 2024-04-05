use std::{cmp, io};
use std::io::Write;
use std::time::Instant;
use crate::block::Block;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

pub struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let blockchain = Blockchain {
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
        let divisor = cmp::min(i64::pow(10, (difficulty - 2) as u32), 1000);
        let mut new_block = Block::new(data, difficulty, self.chain.last());
        let correct_string: &str = &*"0".repeat(difficulty as usize);
        let base_block = new_block.index.to_string()
            + &new_block.timestamp.to_string()
            + &new_block.data
            + &new_block.previous_hash;
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
        println!("Please enter a difficulty for the block:");
        let mut difficulty = String::new();
        io::stdin().read_line(&mut difficulty).expect("error reading");
        let difficulty: i32 = difficulty.trim().parse().expect("Invalid input");
        println!("Please enter Data for new difficulty {} block!", difficulty);
        let mut data = String::new();
        io::stdin().read_line(&mut data).expect("error reading");
        let divisor = cmp::min(i64::pow(10, (difficulty - 2) as u32), 1000);
        let mut new_block = Block::new(data, difficulty, self.chain.last());
        let correct_string: &str = &*"0".repeat(difficulty as usize);
        let input = Arc::new(new_block.index.to_string()
            + &new_block.timestamp.to_string()
            + &new_block.data
            + &new_block.previous_hash);
        let num_threads = num_cpus::get();
        let found = Arc::new(AtomicBool::new(false));
        let found_nonce = Arc::new(std::sync::Mutex::new(None));
        println!("Mining block #{} of {} difficulty, {} threads used", new_block.index, difficulty, num_threads);
        let now = Instant::now(); // Start the timer

        // Print the initial lines for each thread
        for i in 0..num_threads {
            println!("Thread {}: ", i);
        }

        let threads: Vec<_> = (0..num_threads)
            .map(|i| {
                let input = Arc::clone(&input);
                let found = Arc::clone(&found);
                let found_nonce = Arc::clone(&found_nonce);
                let correct_string = correct_string.to_owned();
                thread::spawn(move || {
                    let mut current_nonce = i as i64;
                    loop {
                        if found.load(Ordering::Relaxed) {
                            break;
                        }
                        let hash = Block::calculate_hash(input.to_string() + &current_nonce.to_string());
                        if current_nonce % divisor == 0 {
                            print!("\x1B[{}A\r\x1B[KThread {}: Trying nonce {}, Hash: {}\x1B[{}B", i + 1, i, current_nonce, hash, i + 1);
                            io::stdout().flush().unwrap();
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

        // Move the cursor down to the original position
        print!("\x1B[{}B", num_threads);

        let found_nonce_value = *found_nonce.lock().unwrap();
        match found_nonce_value {
            Some(nonce_value) => {
                let timer: f64 = (now.elapsed().as_millis() as f64) / 1000f64;
                println!("\nBlock Mined in {} Seconds with Parallelism!\nNonce: {}\nHash: {}", timer, nonce_value, Block::calculate_hash(input.to_string() + &nonce_value.to_string()));
                new_block.nonce = nonce_value;
                self.chain.push(new_block);
            }
            None => {
                println!("No valid nonce found.");
            }
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
    pub fn remove_block(&mut self, index: usize) {
        if index < self.chain.len() {
            self.chain.remove(index);
            println!("Block removed successfully");
        } else {
            println!("Invalid block index");
        }
    }
}