use crate::models::block::Block;
use chrono::Utc;

#[derive(Debug, Clone)]
pub struct Blockchain {
    pub genesis_block: Block,
    pub chain: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let genesis_block = Block {
            index: 0,
            timestamp: Utc::now().timestamp_millis() as u64,
            proof_of_work: 0,
            previous_hash: String::default(),
            hash: String::default(),
        };

        let blockchain = Blockchain {
            genesis_block: genesis_block.clone(),
            chain: vec![genesis_block],
            difficulty,
        };

        blockchain
    }

    pub fn add_block(&mut self, nonce: u64) {
        let new_block = Block::new(
            self.chain.len() as u64,
            nonce,
            self.chain[self.chain.len() - 1].hash.clone(),
        );

        let mut mined_block = new_block.clone();
        mined_block.mine(self.clone()); // Passer une copie de Blockchain
        self.chain.push(mined_block);
        println!("New block added to chain -> {:?}\n", new_block);
    }

    // Nouvelle méthode pour accéder au bloc genesis
    pub fn get_genesis_block(&self) -> &Block {
        &self.genesis_block
    }

    // Nouvelle méthode pour accéder à la difficulté
    pub fn get_difficulty(&self) -> usize {
        self.difficulty
    }
}
