use std::fmt;

use sha2::{Digest, Sha256};

use crate::transaction::Transaction;

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(
        index: u64,
        timestamp: u64,
        transactions: Vec<Transaction>,
        previous_hash: String,
        nonce: u64,
    ) -> Block {
        let block = Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash: String::new(),
            nonce
        };
        let hash = block.hash_block();
        Block {
            hash,
            ..block
        }
    }

    pub fn hash_block(&self) -> String {
        let mut hasher = Sha256::new();
        let input = format!(
            "{}{}{:?}{}{}",
            self.index, self.timestamp, self.transactions, self.previous_hash, self.nonce, 
        );
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Block #{} [Hash: {}, Prev. Hash: {}, Nonce: {}]",
            self.index, self.hash, self.previous_hash, self.nonce
        )
    }
}