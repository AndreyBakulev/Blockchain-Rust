mod blockchain;
mod block;

use std::io;
use blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new(4);

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
                blockchain.mine_latest();
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
                println!("Enter the new difficulty:");
                let mut difficulty = String::new();
                io::stdin().read_line(&mut difficulty).expect("Failed to read input");
                let difficulty: i32 = difficulty.trim().parse().expect("Invalid input");

                blockchain.set_difficulty(difficulty);
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