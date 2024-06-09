// std library
use std::fs;
use std::path::Path;

// 3rd party crates
use serde::Serialize;
use serde_json::{to_string, to_value, Value};

// imports
use crate::mods::base::{Blockchain, KeyPairs, Transactions, Wallets};

// constants
const BLOCKCHAIN_PATH: &str = "./src/data/blockchain.json";
const TRANSACTIONS_PATH: &str = "./src/data/transactions.json";
const WALLETS_PATH: &str = "./src/data/wallets.json";
const KEYPAIRS_PATH: &str = "./src/data/keypairs.json";

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
        let bc = to_string(&Blockchain {blockchain: []}).unwrap();
        fs::write(BLOCKCHAIN_PATH, bc).expect("[-] Failed to write blockchain.json");
        
        let t = to_string(&Transactions {transactions: []}).unwrap();
        fs::write(TRANSACTIONS_PATH, t).expect("[-] Failed to write transactions.json");
        
        if !preserve_accounts {
            let w = to_string(&Wallets {wallets: []}).unwrap();
            fs::write(WALLETS_PATH, w).expect("[-] Failed to write wallets.json");
            
            let kp = to_string(&KeyPairs {keypairs: []}).unwrap();
            fs::write(KEYPAIRS_PATH, kp).expect("[-] Failed to write keypairs.json");
        }
    }

    /// Check if a file already exists
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
    /// bool
    /// ```
    pub fn _exists(path: &str) -> bool {
        Path::new(path).exists()
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
            // convert base array into mut Vec and push on the converted obj
            base_data[base].as_array_mut().unwrap().push(value);
            // write data back to file (full overwrite with new data appended) 
            fs::write(path, base_data.to_string()).expect("Failed to write file");
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
        serde_json::from_str(&json_str).expect("Poorly formatted JSON found")
    }
}
