mod block;
mod blockchain;
mod transaction;

// use block::Block;
use blockchain::BlockChain;
use transaction::Transaction;

fn main() {
    let mut ethereum = BlockChain::new();
    ethereum.set_current_transactions(random_transactions());
    ethereum.new_block(0, None);
    println!("{:#?}", ethereum.chain);
}





fn random_transactions() -> Vec<Transaction> {
    let transactions = vec![
        Transaction {
            sender: String::from("Alice"),
            receiver: String::from("Bob"),
            amount: 1.0,
        },
        Transaction {
            sender: String::from("Bob"),
            receiver: String::from("Charlie"),
            amount: 2.0,
        },
        Transaction {
            sender: String::from("Mide"),
            receiver: String::from("Aghahowa"),
            amount: 2.0,
        },
        Transaction {
            sender: String::from("Al Rho"),
            receiver: String::from("Franklin"),
            amount: 2.0,
        },
        Transaction {
            sender: String::from("Ebuka"),
            receiver: String::from("Wizzy"),
            amount: 2.0,
        },
    ];
    transactions
}
