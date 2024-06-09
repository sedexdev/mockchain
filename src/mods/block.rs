// 3rd party crates
use serde::Serialize;
use serde_json::Value;

/// Creates a block to append to the chain
/// 
/// # Fields
/// ```
/// timestamp: String        -> timestamp of block creation
/// hash: String             -> hash of this Block (default value of "0".repeat(64))
/// previous_hash: String    -> "None" by default,
/// nonce: u32               -> nonce used to produce this block 
/// transactions: [Value; 0] -> transactions in this Block
/// merkle_root: String      -> hash of all transactions in this Block
/// ```
/// 
/// # Derives
/// ```
/// serde::{Serialize, Deserialize}
/// ```
#[derive(Serialize, Debug)]
pub struct Block {
    // num: u32,
    pub timestamp: String,
    pub hash: String,
    pub previous_hash: String,
    pub nonce: u32,
    pub transactions: [Value; 0],
    pub merkle_root: String,
}
