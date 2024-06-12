// std library
use std::fs;
use std::path::Path;

// 3rd party crates
use serde::Serialize;
use serde_json::{to_string, to_value, Value};

// imports
use crate::mods::base::{Blockchain, KeyPairs, Transactions, Wallets};

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

// Testing
#[cfg(test)]
mod test_file {
    use super::*;

    use std::{thread, time};

    use crate::mods::block::Block;
    use crate::mods::constants::{
        BLOCKCHAIN_PATH_TEST,
        KEYPAIRS_PATH_TEST,
        TRANSACTIONS_PATH_TEST,
        WALLETS_PATH_TEST
    };

    #[test]
    fn test_init() {
        let data_files = [BLOCKCHAIN_PATH_TEST, KEYPAIRS_PATH_TEST, TRANSACTIONS_PATH_TEST, WALLETS_PATH_TEST];
        FileOps::init(false, data_files);
        assert_eq!(4, fs::read_dir("./src/data/test_data").unwrap().count());
    }
    
    #[test]
    fn test_exists() {
        // sleep to allow file creation
        let one_sec = time::Duration::from_millis(1000);
        thread::sleep(one_sec);

        assert!(FileOps::exists(BLOCKCHAIN_PATH_TEST));
        assert!(FileOps::exists(KEYPAIRS_PATH_TEST));
        assert!(FileOps::exists(TRANSACTIONS_PATH_TEST));
        assert!(FileOps::exists(WALLETS_PATH_TEST));
    }

    #[test]
    fn test_write_and_parse() {

        // sleep to allow file init and exists tests
        let two_secs = time::Duration::from_millis(2000);
        thread::sleep(two_secs);

        // create a Block for testing
        let test_block = Block {
            timestamp: String::from("01/01/9999:00:00:00"),
            hash: String::from("0".repeat(64)),
            previous_hash: String::from("None"),
            nonce: 0,
            transactions: [],
            merkle_root: String::from("None"),
        };
        // write Block to test file
        FileOps::write(BLOCKCHAIN_PATH_TEST, "blockchain", &test_block);

        // parse the Block from the test file
        let mut json_obj = FileOps::parse(BLOCKCHAIN_PATH_TEST);

        // assert "blockchain" Array was updated with 1 element
        assert_eq!(1, json_obj["blockchain"].as_array_mut().unwrap().len());

        // sleep again to allow operations to complete
        thread::sleep(two_secs);

        // re-initialize files after testing
        let data_files = [BLOCKCHAIN_PATH_TEST, KEYPAIRS_PATH_TEST, TRANSACTIONS_PATH_TEST, WALLETS_PATH_TEST];
        FileOps::init(false, data_files);
    }
}
