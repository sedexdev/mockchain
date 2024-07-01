// std library
use std::fs;
use std::path::Path;

// 3rd party crates
use serde::Serialize;
use serde_json::{from_str, to_string, to_value, Value};

// imports
use super::{
    base::{Blockchain, KeyPairs, SigningData, Transactions, Wallets},
    log::{Log, LogLevel},
};
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
    /// initialises the data files when a user first
    /// runs the app or decides to re-initialise the
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
                Err(e) => {
                    Log::new_panic(LogLevel::ERROR, 7, None);
                    panic!(
                        "Error creating /.mockchain/data/ directory under $HOME: {}",
                        e
                    );
                }
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
            Err(e) => {
                Log::new_panic(LogLevel::ERROR, 8, Some(vec![file_name.to_string()]));
                panic!("Error initialising data file: {}.json: {}", &file_name, e);
            }
        };
        match fs::write(data_file, data) {
            Ok(_) => {}
            Err(e) => {
                Log::new_panic(LogLevel::ERROR, 9, Some(vec![file_name.to_string()]));
                panic!("Failed to write {}.json: {}", &file_name, e);
            }
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
            Err(e) => {
                Log::new_panic(LogLevel::ERROR, 10, Some(vec![base.to_string()]));
                panic!("Failed to parse given object to serde_json Value: {}", e);
            }
        };
        // parse data from base file
        let mut base_data = FileOps::parse(path);
        let data = match base_data[base].as_array_mut() {
            Some(d) => d,
            None => {
                Log::new_panic(LogLevel::ERROR, 2, Some(vec![format!("{}.json", base)]));
                panic!("Failed to read {}.json, has the data been modified or the file moved or deleted?", base);
            }
        };
        data.push(value);
        // write data back to file (full overwrite with new data appended)
        match fs::write(path, base_data.to_string()) {
            Ok(_) => {}
            Err(e) => {
                Log::new_panic(LogLevel::ERROR, 9, Some(vec![base.to_string()]));
                panic!("Failed to write {}.json: {}", base, e);
            }
        };
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
            None => {
                Log::new_panic(LogLevel::ERROR, 2, Some(vec!["wallets.json".to_string()]));
                panic!("Failed to read wallets.json, has the data been modified or the file moved or deleted?");
            }
        };
        for wallet in wallets {
            if wallet["address"].to_string() == address {
                if let Ok(value) = to_value(balance) {
                    wallet["balance"] = value;
                    match fs::write(WALLETS_PATH.as_path(), base_data.to_string()) {
                        Ok(_) => {}
                        Err(e) => {
                            Log::new_panic(LogLevel::ERROR, 9, Some(vec!["wallets".to_string()]));
                            panic!("Failed to write wallets.json: {}", e);
                        }
                    };
                    break;
                } else {
                    Log::new_panic(LogLevel::ERROR, 11, Some(vec![balance.to_string()]));
                    panic!(
                        "Failed to parse balance to serde_json Value; given value: {}",
                        &balance
                    );
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
            Err(e) => {
                Log::new_panic(LogLevel::ERROR, 12, Some(vec![format!("{:?}", path)]));
                panic!(
                    "Error parsing data file content at {}: {}",
                    format!("{:?}", path),
                    e
                );
            }
        };
        let value = match from_str(&json_str) {
            Ok(val) => val,
            Err(e) => {
                Log::new_panic(LogLevel::ERROR, 13, Some(vec![json_str.clone()]));
                panic!("Poorly formatted JSON found: {}", e);
            }
        };
        value
    }
}
