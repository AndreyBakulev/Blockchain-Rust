use crate::block::Block;
use crate::block::Transaction;
//use crate when importing a file relative to the crate root (main.rs)
#[derive(Debug)]
pub struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Self { chain: vec![Self::genesis()] }
    }

    fn genesis() -> Block {
        Block::new(0, vec![], String::from("0"))
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let previous_block = self.chain.last().unwrap();
        let new_block = Block::new(previous_block.index + 1, transactions, previous_block.hash.clone());
        self.chain.push(new_block);
    }
}