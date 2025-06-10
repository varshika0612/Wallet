use serde::{Serialize, Deserialize};
use sha2::{Digest, Sha256};
use std::fs;
use chrono::Utc;
pub mod block;
use block::Block;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockChain {
    pub blocks: Vec<Block>,
}

impl BlockChain {
    /// Create a new blockchain with the genesis block
    pub fn new() -> Self {
        BlockChain {
            blocks: vec![Block::genesis()],
        }
    }

    /// Add a new block to the chain
    pub fn add_block(&mut self, data: &str) {
        let previous_block = self.blocks.last().unwrap();
        let new_block = Block::new(
            previous_block.index + 1,
            previous_block.hash.clone(),
            data,
        );
        self.blocks.push(new_block);
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.blocks.len() {
            let prev = &self.blocks[i - 1];
            let curr = &self.blocks[i];
            if curr.previous_hash != prev.hash {
                return false;
            }
            let check_hash = Block::calculate_hash(
                curr.index,
                &curr.previous_hash,
                curr.timestamp,
                &curr.data,
                curr.nonce,
            );
            if curr.hash != check_hash {
                return false;
            }
        }
        true
    }

    pub fn save(&self) {
        let data = serde_json::to_string_pretty(self).unwrap();
        fs::write("blockchain.json", data).unwrap();
    }

    pub fn load() -> Option<Self> {
        let data = fs::read_to_string("blockchain.json").ok()?;
        serde_json::from_str(&data).ok()
    }
}
