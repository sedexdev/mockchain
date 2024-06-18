// std library
use std::fs;
use std::path::Path;

// 3rd party crates
use serde::Serialize;
use serde_json::{to_string, to_value, Value};

// imports
use super::{base::{Blockchain, KeyPairs, Transactions, Wallets}, wallet::Wallet};

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
    /// data_files: [&str; 4]   -> data file paths for writing
    /// ```
    /// 
    /// # Returns
    /// Nothing
    pub fn init(preserve_accounts: bool, data_files: [&str; 4]) {
        for file in data_files {
            if file.contains("blockchain") {
                let bc = to_string(&Blockchain {blockchain: []}).unwrap();
                fs::write(file, bc).expect(format!("[-] Failed to write '{}'", file).as_str());
            }
            if file.contains("transactions") {
                let t = to_string(&Transactions {transactions: []}).unwrap();
                fs::write(file, t).expect(format!("[-] Failed to write '{}'", file).as_str());
            }
            if !preserve_accounts {
                if file.contains("keypairs") {
                        let kp = to_string(&KeyPairs {keypairs: []}).unwrap();
                        fs::write(file, kp).expect(format!("[-] Failed to write '{}'", file).as_str());
                }
                if file.contains("wallets") {
                        let w = to_string(&Wallets {wallets: []}).unwrap();
                        fs::write(file, w).expect(format!("[-] Failed to write '{}'", file).as_str());
                }
            }
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
    pub fn exists(path: &str) -> bool {
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
            base_data[base].as_array_mut().unwrap().push(value);
            // write data back to file (full overwrite with new data appended) 
            fs::write(path, base_data.to_string()).expect("Failed to write file");
        }
    }

    /// Write a new value to the balance field of an account
    /// 
    /// # Visibility
    /// public
    /// 
    /// # Args
    /// ```
    /// path: &str   -> path to write to
    /// name: String -> name of account to lookup
    /// balance: i64 -> new balance to write
    /// ```
    /// 
    /// # Returns
    /// Nothing
    pub fn write_balance(path: &str, name: String, balance: i64) {
        if !Wallet::name_exists(path, &name) {
            println!("No account found for '{}'", name);
        } else {
            let mut base_data = FileOps::parse(path);
            let wallets = base_data["wallets"].as_array_mut().unwrap();
            for wallet in wallets {
                if wallet["name"] == name {
                    wallet["balance"] = to_value(balance).unwrap();
                    fs::write(path, base_data.to_string()).expect("Failed to write file");
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
        serde_json::from_str(&json_str).expect("Poorly formatted JSON found")
    }
}
