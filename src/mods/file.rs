// std library
use std::fs;
use std::path::Path;

// 3rd party crates
use serde::Serialize;
use serde_json::{from_str, to_string, to_value, Value};

// imports
use super::base::{Blockchain, KeyPairs, SigningData, Transactions, Wallets};
use super::constants::{
    BLOCKCHAIN_PATH, KEYPAIRS_PATH, SIGNING_DATA_PATH, TRANSACTIONS_PATH, WALLETS_PATH,
};

/// File operations for working with JSON
///
/// # Visibility
/// public
///
/// # Fields
/// None
///
/// # Derives
/// ```
/// serde::Serialize, Debug
/// ```
#[derive(Serialize, Debug)]
pub struct FileOps {}

impl FileOps {
    /// Initializes the data files when a user first
    /// runs the app or decides to re-initialize the
    /// blockchain
    ///
    /// # Visibility
    /// public
    ///
    /// # Args
    /// ```
    /// preserve_accounts: bool -> option to preserve wallet and key data
    /// ```
    ///
    /// # Returns
    /// Nothing
    pub fn init(preserve_accounts: bool) {
        if !Path::new("./src/data/").exists() {
            match fs::create_dir("./src/data/") {
                Ok(_) => {}
                Err(e) => panic!("Error creating data directory: {}", e),
            };
        }
        let bc = match to_string(&Blockchain { blockchain: [] }) {
            Ok(val) => val,
            Err(e) => panic!("Error initializing data file: {}", e),
        };
        match fs::write(BLOCKCHAIN_PATH, bc) {
            Ok(_) => {}
            Err(e) => panic!("Failed to write blockchain.json: {}", e),
        };

        let t = match to_string(&Transactions { transactions: [] }) {
            Ok(val) => val,
            Err(e) => panic!("Error initializing data file: {}", e),
        };
        match fs::write(TRANSACTIONS_PATH, t) {
            Ok(_) => {}
            Err(e) => panic!("Failed to write transaction.json: {}", e),
        };

        let sd = match to_string(&SigningData { signing_data: [] }) {
            Ok(val) => val,
            Err(e) => panic!("Error initializing data file: {}", e),
        };
        match fs::write(SIGNING_DATA_PATH, sd) {
            Ok(_) => {}
            Err(e) => panic!("Failed to write signing.json: {}", e),
        };

        if !preserve_accounts {
            let kp = match to_string(&KeyPairs { keypairs: [] }) {
                Ok(val) => val,
                Err(e) => panic!("Error initializing data file: {}", e),
            };
            match fs::write(KEYPAIRS_PATH, kp) {
                Ok(_) => {}
                Err(e) => panic!("Failed to write keypairs.json: {}", e),
            };

            let w = match to_string(&Wallets { wallets: [] }) {
                Ok(val) => val,
                Err(e) => panic!("Error initializing data file: {}", e),
            };
            match fs::write(WALLETS_PATH, w) {
                Ok(_) => {}
                Err(e) => panic!("Failed to write wallets.json: {}", e),
            };
        }
    }

    /// Writes the current state of the blockchain to
    /// file in blockchain.json
    ///
    /// # Visibility
    /// public
    ///
    /// # Args
    /// ```
    /// path -> &str path slice
    /// base -> &str slice of base struct name
    /// obj  -> an object T that implements Serialize
    /// ```
    ///
    /// # Returns
    /// Nothing
    ///
    pub fn write<T: Serialize>(path: &str, base: &str, obj: T) {
        // convert the obj into a serde_json::Value
        let value = match to_value(&obj) {
            Ok(val) => val,
            Err(_) => Value::String(String::from("result: failed")),
        };
        if value.as_str() == Some("result: failed") {
            println!("Failed to convert file at '{}' to JSON string", path);
        } else {
            // parse data from base file
            let mut base_data = FileOps::parse(path);
            let data = match base_data[base].as_array_mut() {
                Some(d) => d,
                None => panic!("Data not found while writing, has the file been moved or deleted?"),
            };
            data.push(value);
            // write data back to file (full overwrite with new data appended)
            match fs::write(path, base_data.to_string()) {
                Ok(_) => {}
                Err(e) => panic!("Failed to write file: {}", e),
            };
        }
    }

    /// Write a new value to the balance field of an account
    ///
    /// # Visibility
    /// public
    ///
    /// # Args
    /// ```
    /// address: String -> name of account to lookup
    /// balance: i32    -> new balance to write
    /// ```
    ///
    /// # Returns
    /// Nothing
    pub fn write_balance(address: String, balance: i32) {
        let mut base_data = FileOps::parse(WALLETS_PATH);
        let wallets = match base_data["wallets"].as_array_mut() {
            Some(data) => data,
            None => panic!("Wallet data not found, has the file been moved or deleted?"),
        };
        for wallet in wallets {
            if wallet["address"].to_string() == address {
                if let Ok(value) = to_value(balance) {
                    wallet["balance"] = value;
                    match fs::write(WALLETS_PATH, base_data.to_string()) {
                        Ok(_) => {}
                        Err(e) => panic!("Failed to write file: {}", e),
                    };
                    break;
                } else {
                    panic!("Failed to write new balance");
                };
            }
        }
    }

    /// Parse a JSON string into a serde_json Value Object
    ///
    /// # Visibility
    /// public
    ///
    /// # Args
    /// ```
    /// path -> &str path slice
    /// ```
    ///
    /// # Returns
    /// ```
    /// Value
    /// ```
    pub fn parse(path: &str) -> Value {
        let json_str = match fs::read_to_string(Path::new(path)) {
            Ok(content) => content,
            Err(e) => panic!("Error reading file content: {}", e),
        };
        let value = match from_str(&json_str) {
            Ok(val) => val,
            Err(e) => panic!("Poorly formatted JSON found: {}", e),
        };
        value
    }
}
