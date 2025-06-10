use serde::{Serialize, Deserialize};
use sha2::{Digest, Sha256};
use chrono::Utc;
#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct Block {
    pub index: u64,
    pub previous_hash: String,
    pub timestamp: u64,
    pub data: String,
    pub hash: String,
    pub nonce: u64,
}
impl Block{
    pub fn new(index:u64,previous_hash:String,data:&str) -> Self {
        let timestamp = Utc::now().timestamp() as u64;
        let mut nonce=0;
        let mut hash=Block::calculate_hash(index, &previous_hash, timestamp, data, nonce);
        while !hash.starts_with("00") {
            nonce += 1;
            hash = Block::calculate_hash(index, &previous_hash, timestamp, data, nonce);
        }
        Block{
            index,
            previous_hash,
            timestamp,
            data: data.to_string(),
            hash,
            nonce,
        }
    }
    pub fn calculate_hash(index: u64, previous_hash: &str, timestamp: u64, data: &str, nonce: u64) -> String {
        let mut hasher = Sha256::new();
        hasher.update(index.to_string());
        hasher.update(previous_hash.as_bytes());
        hasher.update(timestamp.to_string());
        hasher.update(data.as_bytes());
        hasher.update(nonce.to_string());
        format!("{:x}", hasher.finalize())
    }
    pub fn genesis() -> Self {
        Block::new(0, String::from("0"), "Genesis Block")
    }
}