use crate::block::Block;
use crate::transaction::Transaction;
use chrono::Utc;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct BlockChain {
    pub chain: Vec<Block>,
    pub current_transactions: Vec<Transaction>,
}

impl BlockChain {
    #[allow(dead_code)]
    pub fn new() -> BlockChain {
        BlockChain {
            chain: vec![
                //genesis block
                Block {
                    index: 0,
                    timestamp: 0,
                    transactions: vec![],
                    previous_hash: String::from("0"),
                    hash: String::from("0"),
                    nonce: 0,
                },
            ],
            current_transactions: vec![],
        }
    }

    #[allow(dead_code)]
    pub fn add_block(&mut self, block: Block) {
        self.chain.push(block);
    }

    #[allow(dead_code)]
    pub fn last_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    #[allow(dead_code)]
    pub fn new_block(&mut self, nonce: u64, previous_hash: Option<String>) -> Block {
        let block = Block::new(
            self.chain.len() as u64 + 1,
            Utc::now().timestamp() as u64,
            self.current_transactions.clone(),
            previous_hash.unwrap_or(self.hash(self.last_block())),
            nonce,
        );
        self.current_transactions = vec![];
        self.add_block(block.clone());
        block
    }

    #[allow(dead_code)]
    pub fn set_current_transactions(&mut self, transactions: Vec<Transaction>) {
        self.current_transactions = transactions;
    }

    pub fn hash(&self, block: &Block) -> String {
        let mut hasher = Sha256::new();
        let input = format!(
            "{}{}{:?}{}{}",
            block.index, block.timestamp, block.transactions, block.previous_hash, block.nonce,
        );
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}
