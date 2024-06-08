// std library imports
use std::fs;
use std::path::Path;

// 3rd party crates
use serde::Serialize;
use serde_json;

// imports
use crate::mods::base::*;

// constants
const BLOCKCHAIN_PATH: &str = "./src/data/blockchain.json";
const TRANSACTIONS_PATH: &str = "./src/data/transactions.json";
const WALLETS_PATH: &str = "./src/data/wallets.json";
const KEYPAIRS_PATH: &str = "./src/data/keypairs.json";

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
    println!();
    init_helper(BLOCKCHAIN_PATH, Blockchain::new());
    init_helper(TRANSACTIONS_PATH, Transactions::new());
    if !preserve_accounts {
        init_helper(WALLETS_PATH, Wallets::new());
        init_helper(KEYPAIRS_PATH, KeyPairs::new());
    }
}

/// file writing helper for init function
/// 
/// # Visibility
/// private
/// 
/// # Args
/// ```
/// path: &str -> file path to create
/// obj: T     -> generic object type that implements Serialize
/// ```
/// 
/// # Returns
/// Nothing
fn init_helper<T: Serialize>(path: &str, obj: T) {
    let json_data = match serde_json::to_string(&obj) {
        Ok(content) => content,
        Err(_) => String::from("FAILED"),
    };
    if json_data == "FAILED" {
        println!("[-] Data serialization failed. Does 'obj' implement Serialize?");
    } else {
        match fs::write(Path::new(path), json_data) {
            Ok(_) => println!("[+] Write '{}' succeeded", path),
            Err(_) => println!("[-] Write '{}' failed", path),
        };
    }
}
