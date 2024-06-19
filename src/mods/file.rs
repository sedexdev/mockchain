// std library
use std::fs;
use std::path::Path;

// 3rd party crates
use serde::Serialize;
use serde_json::{to_string, to_value, Value};

// imports
use super::{base::{Blockchain, KeyPairs, SigningData, Transactions, Wallets}, wallet::Wallet};
use super::constants::{
    BLOCKCHAIN_PATH,
    KEYPAIRS_PATH,
    SIGNING_DATA_PATH,
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
        let bc = to_string(&Blockchain {blockchain: []}).unwrap();
        fs::write(BLOCKCHAIN_PATH, bc).expect(format!("[-] Failed to write 'blockchain.json'").as_str());

        let t = to_string(&Transactions {transactions: []}).unwrap();
        fs::write(TRANSACTIONS_PATH, t).expect(format!("[-] Failed to write 'transactions.json'").as_str());

        if !preserve_accounts {
            let kp = to_string(&KeyPairs {keypairs: []}).unwrap();
            fs::write(KEYPAIRS_PATH, kp).expect(format!("[-] Failed to write 'keypairs.json'").as_str());

            let w = to_string(&Wallets {wallets: []}).unwrap();
            fs::write(WALLETS_PATH, w).expect(format!("[-] Failed to write 'wallets.json'").as_str());

            let sd = to_string(&SigningData {signing_data: []}).unwrap();
            fs::write(SIGNING_DATA_PATH, sd).expect(format!("[-] Failed to write 'signing.json'").as_str());
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
    /// name: String -> name of account to lookup
    /// balance: i64 -> new balance to write
    /// ```
    /// 
    /// # Returns
    /// Nothing
    pub fn write_balance(name: String, balance: i64) {
        if !Wallet::name_exists(&name) {
            println!("No account found for '{}'", name);
        } else {
            let mut base_data = FileOps::parse(WALLETS_PATH);
            let wallets = base_data["wallets"].as_array_mut().unwrap();
            for wallet in wallets {
                if wallet["name"] == name {
                    wallet["balance"] = to_value(balance).unwrap();
                    fs::write(WALLETS_PATH, base_data.to_string()).expect("Failed to write file");
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
