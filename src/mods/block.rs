// 3rd party crates
use chrono::Utc;
use serde::Serialize;
use serde_json::{to_string, to_value, Value};

// imports
use super::{
    constants::{BLOCKCHAIN_PATH, TRANSACTIONS_PATH},
    crypto::{get_merkle_root, hash_block},
    file::FileOps,
};

/// Defines a block to append to the chain
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
/// serde::Serialize, Debug
/// ```
#[derive(Serialize, Debug)]
pub struct Block {
    // num: u32,
    pub timestamp: String,
    pub hash: String,
    pub previous_hash: String,
    pub nonce: u32,
    pub transactions: Value,
    pub merkle_root: String,
}

impl Block {
    /// Creates and writes the genesis block to
    /// the blockchain
    ///
    /// # Visibility
    /// public
    ///
    /// # Args
    /// None
    ///
    /// # Returns
    /// Nothing
    pub fn add_genesis_block() {
        let now = Utc::now();
        let timestamp = now.to_rfc3339();
        let transactions = to_string(&Value::Array([].to_vec())).unwrap();
        let hash = hash_block(&String::from("0"), &String::from("N/A"), &transactions);
        let merkle_root = get_merkle_root(TRANSACTIONS_PATH);
        let genesis_block = Block {
            timestamp,
            hash,
            previous_hash: String::from("N/A"),
            nonce: 0,
            transactions: to_value(transactions).unwrap(),
            merkle_root,
        };
        FileOps::write(BLOCKCHAIN_PATH, "blockchain", genesis_block);
    }
}
