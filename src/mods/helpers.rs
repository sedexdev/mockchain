// 3rd party crates
use chrono::Utc;

// imports
use super::{
    block::Block,
    crypto::{get_merkle_root, hash_block, hash_transaction, KeyPair},
    file::FileOps,
    log::{Log, LogLevel},
    signing_data::Signing,
    transaction::Transaction,
    wallet::Wallet,
};
use crate::{BLOCKCHAIN_PATH, KEYPAIRS_PATH, SIGNING_DATA_PATH, TRANSACTIONS_PATH, WALLETS_PATH};

/// Gets an RFC3339 timestamp
///
/// # Visibility
/// public
///
/// # Args
/// None
///
/// # Returns
/// ```
/// String
/// ```
pub fn get_timestamp() -> String {
    let now = Utc::now();
    let timestamp = now.to_rfc3339();
    timestamp
}

/// Creates a wallet
///
/// # Visibility
/// public
///
/// # Args
/// ```
/// name: String -> name of the account
/// ```
/// # Returns
/// Nothing
pub fn create_wallet(name: String) {
    let key_pair = KeyPair::generate(name.clone());
    Log::new(LogLevel::INFO, 6);
    let address = key_pair.public_key.clone();
    let wallet = Wallet {
        name,
        address,
        balance: 0,
    };
    Log::new(LogLevel::INFO, 7);
    FileOps::write(KEYPAIRS_PATH.as_path(), "keypairs", key_pair);
    FileOps::write(WALLETS_PATH.as_path(), "wallets", wallet);
}

/// Creates a Transaction
///
/// # Visibility
/// public
///
/// # Args
/// ```
/// from: String -> name of sender
/// to: String   -> name of recipient
/// amount: i32  -> amount
/// ```
///
/// # Returns
/// Nothing
pub fn create_transaction(from: String, to: String, amount: i32) {
    Log::new(LogLevel::INFO, 17);
    // get wallet public keys
    let from_address = match Wallet::get_wallet_address(&from) {
        Some(key) => key.replace("\"", ""),
        None => return,
    };
    let to_address = match Wallet::get_wallet_address(&to) {
        Some(key) => key.replace("\"", ""),
        None => return,
    };
    Log::new(LogLevel::INFO, 18);

    // get transaction hash
    let hash = hash_transaction(&from_address, &to_address, &amount.to_string());
    Log::new(LogLevel::INFO, 19);

    // get senders private key
    let mut json_data = FileOps::parse(KEYPAIRS_PATH.as_path());
    let key_data = match json_data["keypairs"].as_array_mut() {
        Some(arr) => arr,
        None => panic!("Failed to read key data from 'keypairs.json'"),
    };

    let mut private_key = String::from("");
    for key_pair in key_data {
        if key_pair["name"] == from {
            let key_value = match key_pair["private_key"].as_str() {
                Some(val) => val,
                None => panic!(
                    "Failed to parse private key from json_serde Value while creating transaction"
                ),
            };
            private_key.push_str(key_value);
        }
    }
    Log::new(LogLevel::INFO, 20);

    // get the transaction signature and the signing key
    let (signature, signing_key) = KeyPair::sign(&hash, private_key);
    Log::new(LogLevel::INFO, 21);

    let signing_data = Signing {
        name: from,
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
    FileOps::write(TRANSACTIONS_PATH.as_path(), "transactions", transaction);
    Log::new(LogLevel::INFO, 23);
    FileOps::write(SIGNING_DATA_PATH.as_path(), "signing_data", signing_data);
    Log::new(LogLevel::INFO, 22);
}

/// Mine the next block in the chain
///
/// # Visibility
/// public
///
/// # Args
/// ```
/// name: String -> name on the miners account
/// ```
/// # Returns
///
/// Nothing
pub fn mine_block(name: String) {
    Log::new(LogLevel::INFO, 8);
    let mut base_data = FileOps::parse(BLOCKCHAIN_PATH.as_path());
    let blockchain = match base_data["blockchain"].as_array_mut() {
        Some(data) => data,
        None => panic!("Failed to parse blockchain data from 'blockchain.json'"),
    };

    let last_block = &blockchain[blockchain.len() - 1];
    // components of Block hash
    let mut nonce = 0;
    let previous_hash = &last_block["hash"].to_string().replace("\"", "");
    let mut base_data = FileOps::parse(TRANSACTIONS_PATH.as_path());
    // set mining difficulty
    let leading_zeros = String::from("0".repeat(2));
    Log::new(LogLevel::INFO, 9);
    // get block hash
    let mut hash = hash_block(
        &nonce.to_string(),
        &previous_hash.clone(),
        &base_data.to_string(),
    );

    // compute the correct hash to mine a new Block (00...98de872911a5e etc)
    while !hash.starts_with(&leading_zeros) {
        nonce += 1;
        hash = hash_block(
            &nonce.to_string(),
            &previous_hash.clone(),
            &base_data.to_string(),
        );
    }
    Log::new(LogLevel::INFO, 10);

    // get the current timestamp
    let timestamp = get_timestamp();

    // get the merkle root of this Blocks Transactions
    let merkle_root = get_merkle_root(TRANSACTIONS_PATH.as_path());
    Log::new(LogLevel::INFO, 11);

    // pay all transactions
    let transactions = match base_data["transactions"].as_array_mut() {
        Some(data) => data,
        None => panic!("Transaction data not found, has the file been moved or deleted?"),
    };

    for t in transactions {
        let amount: i32;
        if let Some(val) = t["amount"].as_i64() {
            amount = val as i32;
        } else {
            panic!("Could not parse transaction amount while mining block");
        };

        if t["from_address"] == "REWARD" {
            Wallet::update_balance(t["to_address"].to_string(), amount, "add");
        } else {
            Wallet::update_balance(t["to_address"].to_string(), amount, "add");
            Wallet::update_balance(t["from_address"].to_string(), amount, "subtract");
        }
    }
    Log::new(LogLevel::INFO, 12);

    let block = Block {
        timestamp,
        hash,
        previous_hash: previous_hash.to_string(),
        nonce,
        transactions: base_data,
        merkle_root,
    };

    FileOps::write(BLOCKCHAIN_PATH.as_path(), "blockchain", block);
    Log::new(LogLevel::INFO, 13);
    Transaction::clear();
    Log::new(LogLevel::INFO, 14);
    Transaction::add_reward(name);
    Log::new(LogLevel::INFO, 15);
    Log::new(LogLevel::INFO, 16);
}

/// Verifies the integrity of the blockchain
///
/// # Visibility
/// public
///
/// # Args
/// None
///
/// # Returns
/// ```
/// bool
/// ```
pub fn verify_chain() -> bool {
    Log::new(LogLevel::INFO, 24);
    let mut bc_base_data = FileOps::parse(BLOCKCHAIN_PATH.as_path());
    let blockchain = match bc_base_data["blockchain"].as_array_mut() {
        Some(data) => data,
        None => panic!("Failed to parse blockchain data from 'blockchain.json'"),
    };

    // loop over each block in the chain
    for i in 1..blockchain.len() {
        let previous_block = blockchain[i - 1].clone();
        let mut current_block = blockchain[i].clone();

        // check hashes match for current record and previous block
        if current_block["previous_hash"] != previous_block["hash"] {
            Log::new(LogLevel::ERROR, 25);
            return false;
        }

        // validate the current block hash
        let hash = hash_block(
            &current_block["nonce"].to_string(),
            &current_block["previous_hash"].to_string().replace("\"", ""),
            &current_block["transactions"].to_string(),
        );

        if current_block["hash"].to_string().replace("\"", "") != hash {
            Log::new(LogLevel::ERROR, 25);
            return false;
        }

        // validate transactions
        let transactions = match current_block["transactions"]["transactions"].as_array_mut() {
            Some(data) => data,
            None => panic!("Transaction data not found, has the file been moved or deleted?"),
        };

        for j in 0..transactions.len() {
            // validate current transaction hash
            let t_hash = hash_transaction(
                &transactions[j]["from_address"]
                    .to_string()
                    .replace("\"", ""),
                &transactions[j]["to_address"].to_string().replace("\"", ""),
                &transactions[j]["amount"].to_string(),
            );

            if transactions[j]["hash"] != t_hash.clone() {
                Log::new(LogLevel::ERROR, 27);
                return false;
            }

            // get signing key for this transaction
            let mut sd_base_data = FileOps::parse(SIGNING_DATA_PATH.as_path());
            let signing_data = match sd_base_data["signing_data"].as_array_mut() {
                Some(data) => data,
                None => panic!("Failed to parse signing data from 'signing.json'"),
            };

            // verify each hash using the signing key
            for s in signing_data {
                if s["hash"] == t_hash.clone() {
                    let (signature, signing_key) = KeyPair::extract(
                        s["signature"].to_string().replace("\"", ""),
                        s["signing_key"].to_string().replace("\"", ""),
                    );
                    if !KeyPair::verify(signature, signing_key, t_hash.clone()) {
                        Log::new(LogLevel::ERROR, 29);
                        return false;
                    }
                }
            }
        }
    }
    Log::new(LogLevel::INFO, 26);
    Log::new(LogLevel::INFO, 28);
    Log::new(LogLevel::INFO, 30);
    Log::new(LogLevel::INFO, 31);
    true
}
