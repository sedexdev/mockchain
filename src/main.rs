// modules
mod mods;

// 3rd party crates
use chrono::{Timelike, Utc};

// imports
use crate::mods::block::Block;
use crate::mods::file::FileOps;

// constants
const BLOCKCHAIN_PATH: &str = "./src/data/blockchain.json";

fn main() {
    FileOps::init(false);

    let dt = Utc::now();
    let timestamp = match dt.with_hour(dt.hour() + 1) {
        Some(val) => val.to_string(),
        None => String::from("Could not extract timestamp"),
    };

    let test_block = Block {
        timestamp,
        hash: String::from("0".repeat(64)),
        previous_hash: String::from("None"),
        nonce: 0,
        transactions: [],
        merkle_root: String::from("None"),
    };

    FileOps::write(BLOCKCHAIN_PATH, "blockchain", &test_block);
    FileOps::write(BLOCKCHAIN_PATH, "blockchain", &test_block);
    FileOps::write(BLOCKCHAIN_PATH, "blockchain", &test_block);
    FileOps::write(BLOCKCHAIN_PATH, "blockchain", &test_block);

    println!("{}", FileOps::parse(BLOCKCHAIN_PATH));
}
