use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};
use chrono::Utc;
use crate::models::blockchain::Blockchain;

// `Block`, A struct that represents a block in a Blockchain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block
{
   pub index: u64,
   pub timestamp: u64,
   pub proof_of_work: u64,
   pub previous_hash: String,
   pub hash: String,
}

impl Block
{
    pub fn new(index: u64, proof_of_work: u64, previous_hash: String) -> Self
    {
        let mut block = Block
        {
            index,
            timestamp: Utc::now().timestamp_millis() as u64,
            proof_of_work,
            previous_hash,
            hash: String::default(),
        };
        block.hash = block.calculate_hash();
        block
    }

    // Mine block hash.
    pub fn mine(&mut self, _blockchain: Blockchain)
    {
        self.proof_of_work += 20;
        self.hash = self.calculate_hash();
    }

    pub fn calculate_hash(&self) -> String
    {
        let mut block_data = self.clone();
        block_data.hash = String::default();
        let serialized_block_data = serde_json::to_string(&block_data).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(serialized_block_data);
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}
