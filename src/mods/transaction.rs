// std library
use std::fs;

// 3rd party crates
use serde::Serialize;
use serde_json::to_string;

// imports
use super::{
    base::Transactions,
    crypto::{hash_transaction, KeyPair},
    file::FileOps,
    signing_data::Signing,
};
use crate::{SIGNING_DATA_PATH, TRANSACTIONS_PATH};

/// Define a Transaction object
///
/// # Visibility
/// public
///
/// # Fields
/// ```
/// hash: String,
/// from_address: String,
/// to_address: String,
/// amount: i32,
/// signature: String
/// ```
///
/// # Derives
/// ```
/// serde::Serialize, Debug
/// ```
#[derive(Serialize, Debug)]
pub struct Transaction {
    pub hash: String,
    pub from_address: String,
    pub to_address: String,
    pub amount: i32,
    pub signature: String,
}

impl Transaction {
    /// Generates a reward Transaction after a Block has
    /// been mined. Once the current transactions have been
    /// cleared this transaction is added as the first in
    /// a new list of transactions
    ///
    /// # Visibility
    /// public
    ///
    /// # Args
    /// ```
    /// name: String -> name of miner of last Block
    /// ```
    ///
    /// # Returns
    /// Nothing
    pub fn add_reward(name: String) {
        let to_address = KeyPair::get_key(name.clone(), String::from("public"));
        let hash = hash_transaction(&String::from("REWARD"), &to_address, &String::from("50"));

        let private_key = KeyPair::get_key(name.clone(), String::from("private"));
        let (signature, signing_key) = KeyPair::sign(&hash, private_key);

        let signing_data = Signing {
            name: name.clone(),
            hash: hash.clone(),
            signing_key,
            signature: signature.clone(),
        };

        let reward = Transaction {
            hash,
            from_address: String::from("REWARD"),
            to_address,
            amount: 50,
            signature,
        };

        FileOps::write(TRANSACTIONS_PATH.as_path(), "transactions", reward);
        FileOps::write(SIGNING_DATA_PATH.as_path(), "signing_data", signing_data);
    }

    /// Clears all current transactions after mining
    ///
    /// # Visibility
    /// public
    ///
    /// # Args
    /// None
    ///
    /// # Returns
    /// Nothing
    pub fn clear() {
        let t = match to_string(&Transactions { transactions: [] }) {
            Ok(val) => val,
            Err(e) => panic!(
                "Failed to parse transactions to String before clearing: {}",
                e
            ),
        };
        match fs::write(TRANSACTIONS_PATH.as_path(), t) {
            Ok(_) => {}
            Err(e) => panic!("Failed to write 'transactions.json': {}", e),
        };
    }
}
