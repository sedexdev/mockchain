// 3rd party crates
use serde::Serialize;
use serde_json::{to_string, to_value, Value};

// imports
use super::{
    crypto::{get_merkle_root, hash_block},
    file::FileOps,
    helpers::get_timestamp,
    log::{Log, LogLevel},
};
use crate::{BLOCKCHAIN_PATH, TRANSACTIONS_PATH};

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
        let timestamp = get_timestamp();
        let transactions = match to_string(&Value::Array([].to_vec())) {
            Ok(val) => val,
            Err(e) => {
                Log::new_panic(LogLevel::ERROR, 1, Some(vec!["Value::String".to_string()]));
                panic!(
                    "Unable to parse genesis block transaction to json_serde Value::String: {}",
                    e
                )
            }
        };
        let transactions = match to_value(transactions) {
            Ok(val) => val,
            Err(e) => {
                Log::new_panic(LogLevel::ERROR, 1, Some(vec!["Value".to_string()]));
                panic!(
                    "Unable to parse genesis block transaction to json_serde Value: {}",
                    e
                )
            }
        };
        let hash = hash_block(
            &String::from("0"),
            &String::from("N/A"),
            &transactions.to_string(),
        );
        let merkle_root = get_merkle_root(TRANSACTIONS_PATH.as_path());
        let genesis_block = Block {
            timestamp,
            hash,
            previous_hash: String::from("N/A"),
            nonce: 0,
            transactions,
            merkle_root,
        };
        FileOps::write(BLOCKCHAIN_PATH.as_path(), "blockchain", genesis_block);
    }
}
