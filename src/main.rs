mod blockchain;
mod block;

use std::io;
use blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new();
    loop {
        println!("===========================================");
        println!("=====CPU Bitcoin miner by Andrey Bakulev===");
        println!("===========================================");
        println!("==================Options==================");
        println!("==========1.Add block to chain=============");
        println!("======2.Verify Integrity of the chain======");
        println!("=======3.Retrieve info from a block========");
        println!("======4.Adjust the chain's difficulty======");
        println!("=============5.Delete a Block==============");
        println!("================6.Exit=====================");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice: i32 = choice.trim().parse().expect("Invalid input");
        match choice {
            1 => {
                println!("Please select mining mode:\nParallel (1)    Linear(2)");
                let mut parallel = String::new();
                io::stdin().read_line(&mut parallel).expect("Failed to read input");
                let parallel: i32 = parallel.trim().parse().expect("Invalid input");
                if parallel == 1 {
                    blockchain.mine_latest_parallel();
                } else if parallel == 2 {
                    blockchain.mine_latest();
                } else {
                    println!("Put in a valid number you ape");
                }
            }
            2 => {
                if blockchain.validate_chain() {
                    println!("Chain is valid!");
                } else {
                    println!("Chain is not valid!");
                }
            }
            3 => {
                println!("Enter the index of the block to retrieve:");
                let mut index = String::new();
                io::stdin().read_line(&mut index).expect("Failed to read input");
                let index: usize = index.trim().parse().expect("Invalid input");

                blockchain.retrieve_block(index);
            }
            4 => {
                println!("This does nothing as of now!");
            }
            5 => {
                println!("Enter the index of the block to remove:");
                let mut index = String::new();
                io::stdin().read_line(&mut index).expect("Failed to read input");
                let index: usize = index.trim().parse().expect("Invalid input");

                blockchain.remove_block(index);
            }
            6 => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid choice, please try again");
            }
        }
    }
}

/*
Notes:
TODO{
    Add json here
    figure out parallelization w rust
    Networking with netter {
    find free cloud db (azure aws google cloud)
    }
    look at Rust CUDA
}
PROBLEMS{
    parallel counting is weird, maybe make my own?
    printing out unevenly (counting up by like 13k instead of 10k) only at high numbers
}
LEVELS:
0: single threaded running (DONE)
1: parallel running
2: parallel running with cuda (separate repo)
3: rust
4: tauri (netter)
*/