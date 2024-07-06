mod models;

use std::thread;
use std::time::Duration;

fn main() {
    let difficulty = 1;
    let mut blockchain = models::blockchain::Blockchain::new(difficulty);
    let nonce: u64 = 1000;

    // Générer un bloc toutes les secondes
    for i in 0.. {
        blockchain.add_block(nonce + i);
        thread::sleep(Duration::from_secs(1));
    }
}
