mod blockchain;
mod block;
use blockchain::Blockchain;
use block::Transaction;
fn main() {
    let mut blockchain = Blockchain::new();

    let transaction1 = Transaction {
        sender: String::from("Alice"),
        receiver: String::from("Bob"),
        amount: 1.0,
    };

    let transaction2 = Transaction {
        sender: String::from("Bob"),
        receiver: String::from("Charlie"),
        amount: 0.5,
    };

    let transactions = vec![transaction1, transaction2];
    blockchain.add_block(transactions);

    println!("{:?}", blockchain);
}