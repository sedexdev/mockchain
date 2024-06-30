// std library
use std::fs;
use std::path::Path;

// 3rd party crates
use serde::Serialize;
use serde_json::{from_str, to_string, to_value, Value};

// imports
use super::base::{Blockchain, KeyPairs, SigningData, Transactions, Wallets};
use crate::{
    BLOCKCHAIN_PATH, DATA_PATH, KEYPAIRS_PATH, SIGNING_DATA_PATH, TRANSACTIONS_PATH, WALLETS_PATH,
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
        if !Path::new(DATA_PATH.as_path()).exists() {
            match fs::create_dir_all(DATA_PATH.as_path()) {
                Ok(_) => {}
                Err(e) => panic!("Error creating data directory: {}", e),
            };
        }

        FileOps::init_helper(
            &Blockchain { blockchain: [] },
            BLOCKCHAIN_PATH.as_path(),
            "blockchain",
        );

        FileOps::init_helper(
            &Transactions { transactions: [] },
            TRANSACTIONS_PATH.as_path(),
            "transactions",
        );

        FileOps::init_helper(
            &SigningData { signing_data: [] },
            SIGNING_DATA_PATH.as_path(),
            "signing",
        );

        if !preserve_accounts {
            FileOps::init_helper(
                &KeyPairs { keypairs: [] },
                KEYPAIRS_PATH.as_path(),
                "keypairs",
            );

            FileOps::init_helper(&Wallets { wallets: [] }, &WALLETS_PATH.as_path(), "wallets");
        }
    }

    /// Init helper
    ///
    /// # Visibility
    /// private
    ///
    /// # Args
    /// ```
    /// obj: T           -> object to be serialized and written
    /// data_file: &Path -> data file path
    /// file_name: &str  -> data file name
    /// ```
    ///
    /// # Returns
    /// Nothing
    fn init_helper<T: Serialize>(obj: T, data_file: &Path, file_name: &str) {
        let data = match to_string(&obj) {
            Ok(val) => val,
            Err(e) => panic!("Error initializing data file: {}.json: {}", &file_name, e),
        };
        match fs::write(data_file, data) {
            Ok(_) => {}
            Err(e) => panic!("Failed to write {}.json: {}", &file_name, e),
        };
    }

    /// Writes the current state of the blockchain to
    /// file in blockchain.json
    ///
    /// # Visibility
    /// public
    ///
    /// # Args
    /// ```
    /// path -> &Path path slice
    /// base -> &str slice of base struct name
    /// obj  -> an object T that implements Serialize
    /// ```
    ///
    /// # Returns
    /// Nothing
    ///
    pub fn write<T: Serialize>(path: &Path, base: &str, obj: T) {
        // convert the obj into a serde_json::Value
        let value = match to_value(&obj) {
            Ok(val) => val,
            Err(_) => Value::String(String::from("result: failed")),
        };
        if value.as_str() == Some("result: failed") {
            println!("Failed to convert file at '{:?}' to JSON string", path);
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
        let mut base_data = FileOps::parse(WALLETS_PATH.as_path());
        let wallets = match base_data["wallets"].as_array_mut() {
            Some(data) => data,
            None => panic!("Wallet data not found, has the file been moved or deleted?"),
        };
        for wallet in wallets {
            if wallet["address"].to_string() == address {
                if let Ok(value) = to_value(balance) {
                    wallet["balance"] = value;
                    match fs::write(WALLETS_PATH.as_path(), base_data.to_string()) {
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
    /// path -> &Path path slice
    /// ```
    ///
    /// # Returns
    /// ```
    /// Value
    /// ```
    pub fn parse(path: &Path) -> Value {
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
