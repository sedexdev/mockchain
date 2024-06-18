// std library
use std::fs;
use std::path::Path;

// 3rd party crates
use serde::Serialize;
use serde_json::{from_str, to_string, to_value, Value};

// imports
use super::{base::{Blockchain, KeyPairs, Transactions, Wallets}, wallet::Wallet};
use super::constants::{
    BLOCKCHAIN_PATH,
    KEYPAIRS_PATH,
    TRANSACTIONS_PATH,
    WALLETS_PATH
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
        let bc = match to_string(&Blockchain {blockchain: []}) {
            Ok(val) => val,
            Err(e) => panic!("[-] Failed to convert Blockchain to JSON serializable string: {}", e),
        };
        fs::write(BLOCKCHAIN_PATH, bc);

        let t = match to_string(&Transactions {transactions: []}) {
            Ok(val) => val,
            Err(e) => panic!("[-] Failed to convert Transaction to JSON serializable string: {}", e),
        };
        fs::write(TRANSACTIONS_PATH, t);

        if !preserve_accounts {
            let kp = match to_string(&KeyPairs {keypairs: []}) {
                Ok(val) => val,
                Err(e) => panic!("[-] Failed to convert KeyPairs to JSON serializable string: {}", e),
            };
            fs::write(KEYPAIRS_PATH, kp);

            let w = match to_string(&Wallets {wallets: []}) {
                Ok(val) => val,
                Err(e) => panic!("[-] Failed to convert Wallets to JSON serializable string: {}", e),
            };
            fs::write(WALLETS_PATH, w);
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
            match base_data[base].as_array_mut() {
                Some(_) => base_data.push(value),
                None => panic!("[-] Failed to parse data array from '{}'", &path),
            };
            // write data back to file (full overwrite with new data appended) 
            fs::write(path, base_data.to_string());
        }
    }

    /// Write a new value to the balance field of an account
    /// 
    /// # Visibility
    /// public
    /// 
    /// # Args
    /// ```
    /// name: String -> name of account to lookup
    /// balance: i64 -> new balance to write
    /// ```
    /// 
    /// # Returns
    /// Nothing
    pub fn write_balance(name: String, balance: i64) {
        if !Wallet::name_exists(&name) {
            println!("[-] No account found for '{}'", name);
        } else {
            let mut base_data = FileOps::parse(WALLETS_PATH);
            let wallets = match base_data["wallets"].as_array_mut() {
                Some(arr) => arr,
                None => panic!("[-] Failed to parse data array from '{}'", WALLETS_PATH),
            };
            for wallet in wallets {
                if wallet["name"] == name {
                    wallet["balance"] = match to_value(balance) {
                        Ok(balance) => balance,
                        Err(e) => panic!("[-] Failed to convert wallet balance to JSON serializable value: {}", e),
                    };
                    fs::write(WALLETS_PATH, base_data.to_string());
                    break;
                }
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
            Err(e) => panic!("Error reading file content: {}", e)
        };
        from_str(&json_str)
    }
}
