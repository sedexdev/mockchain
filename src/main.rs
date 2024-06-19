// modules
mod mods;

// 3rd party crates
use std::{thread, time};

// imports
use mods::{
    constants::{
        BLOCKCHAIN_PATH,
        KEYPAIRS_PATH,
        SIGNING_DATA_PATH,
        TRANSACTIONS_PATH,
        WALLETS_PATH,
    }, crypto::{hash_transaction, KeyPair}, file::FileOps, signing_data::Signing, transaction::Transaction, wallet::Wallet
};

fn main() {
    FileOps::init(false);

    // sleep to allow init
    let one_sec = time::Duration::from_millis(1000);
    thread::sleep(one_sec);
    
    create_wallet(String::from("TEST"));

    // sleep to allow wallet creation
    let one_sec = time::Duration::from_millis(1000);
    thread::sleep(one_sec);

    let from_address = KeyPair::get_key(String::from("TEST"), String::from("public"));

    create_transaction(
        String::from("TEST"),
        from_address,
        String::from("1".repeat(130)),
        10,
    );
}

/// Creates a wallet
/// 
/// # Args
/// ```
/// name: String -> name of the account
/// ```
/// # Returns
/// Nothing
fn create_wallet(name: String) {
    let key_pair = KeyPair::generate(name.clone());
    let address = key_pair.public_key.clone();
    let wallet = Wallet {
        name,
        address,
        balance: 0, 
    };
    FileOps::write(KEYPAIRS_PATH, "keypairs", key_pair);
    FileOps::write(WALLETS_PATH, "wallets", wallet);
}

/// Creates a Transaction
/// 
/// # Args
/// ```
/// name: String         -> name of sender
/// from_address: String -> senders public key
/// to_address: String   -> recipient public key
/// amount: i32          -> amount
/// ```
/// 
/// # Returns
/// Nothing
fn create_transaction(name: String, from_address: String, to_address: String, amount: i32) {
    // get transaction hash
    let hash = hash_transaction(&from_address, &to_address, &amount.to_string());
    
    // get senders private key
    let mut json_data = FileOps::parse(KEYPAIRS_PATH);
    let key_data = match json_data["keypairs"].as_array_mut() {
        Some(arr) => arr,
        None => panic!("[-] Failed to read key data from 'keypairs.json'"),
    };

    let mut private_key = String::from("");
    for key_pair in key_data {
        if key_pair["name"] == name {
            private_key.push_str(key_pair["private_key"].as_str().expect("[-] Failed to fetch private key"));
        }
    }

    // get the transaction signature and the signing key
    let (signature, signing_key) = KeyPair::sign(&hash, private_key);
    
    let signing_data = Signing {
        name,
        hash: hash.clone(),
        signing_key,
        signature: signature.clone(),
    };
    
    let transaction = Transaction { 
        hash: hash.clone(),
        from_address,
        to_address,
        amount,
        signature: signature.clone(),
    };

    // write objects to file
    FileOps::write(TRANSACTIONS_PATH, "transactions", transaction);
    FileOps::write(SIGNING_DATA_PATH, "signing_data", signing_data);
}
