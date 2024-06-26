mod blockchain;
mod block;

use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use blockchain::Blockchain;
use crate::block::Block;

fn main() {
    let mut blockchain = Blockchain::new();
    loop {
        println!("===========================================");
        println!("=====CPU Bitcoin miner by Andrey Bakulev===");
        println!("===========================================");
        println!("==================Options==================");
        println!("=========1.Create New Blockchain===========");
        println!("========2.Load Existing Blockchain=========");

        let mut choice1 = String::new();
        io::stdin().read_line(&mut choice1).expect("Failed to read input");
        let choice1: i32 = choice1.trim().parse().expect("Invalid input");
        let mut json_string: String = String::new();
        match choice1 {
            1 => {
                println!("Give a name to your new Blockchain!");
                io::stdin().read_line(&mut json_string).expect("Failed to read input");
                let name2 = format!("{}.json",json_string);
                let mut file = File::create(name2.clone()).unwrap();
            }
            2 => {
                println!("what is the name of your Blockchain?");
                io::stdin().read_line(&mut json_string).expect("Failed to read input");
                let name2 = format!("{}.json",json_string);
                let file_name = Path::new(&name2);
                if file_name.exists() {
                    println!("File Successfully found!");
                    let mut file = File::open(file_name).unwrap();
                    let mut contents = String::new();
                    file.read_to_string(&mut contents).unwrap();
                    println!("Contents: {}", contents);
                    let chain: Vec<Block> = serde_json::from_str(&contents).unwrap();
                    blockchain.chain = chain;
                    if let Some(last_block) = blockchain.chain.last() {
                        println!("Last block data: {}", last_block.data);
                    }
                } else {
                    println!("Blockchain with name: {} Not found!",json_string);
                }
            }
            _ => {
                println!("Please enter 1 or 2 monkey!!!");
            }
        }
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
        let formatter = format!("{}.json",json_string);
        let json_file: File = File::create(formatter).unwrap();
        match choice {
            1 => {
                println!("Name of JSON: {}", json_string.clone());
                blockchain.mine_latest(None,json_file);
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

                blockchain.remove_block(index,json_file);
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
    to recalculate block, add index param into mine and check for some/none
    Add json here
    Networking with netter {
    find free cloud db (azure aws google cloud)
    }
    look at Rust CUDA
}
PROBLEMS{

}
LEVELS:
0: single threaded running (DONE)
1: parallel running
2: parallel running with cuda (separate repo)
3: rust
4: tauri (netter)
*/