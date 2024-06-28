// 3rd party crates
use chrono::Utc;

// imports
use super::{
    block::Block,
    constants::{
        BLOCKCHAIN_PATH, KEYPAIRS_PATH, SIGNING_DATA_PATH, TRANSACTIONS_PATH, WALLETS_PATH,
    },
    crypto::{get_merkle_root, hash_block, hash_transaction, KeyPair},
    file::FileOps,
    signing_data::Signing,
    transaction::Transaction,
    wallet::Wallet,
};

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
    // get wallet public keys
    let from_address = match Wallet::get_wallet_address(&from) {
        Some(key) => key.replace("\"", ""),
        None => return,
    };
    let to_address = match Wallet::get_wallet_address(&to) {
        Some(key) => key.replace("\"", ""),
        None => return,
    };

    // get transaction hash
    let hash = hash_transaction(&from_address, &to_address, &amount.to_string());

    // get senders private key
    let mut json_data = FileOps::parse(KEYPAIRS_PATH);
    let key_data = match json_data["keypairs"].as_array_mut() {
        Some(arr) => arr,
        None => panic!("Failed to read key data from 'keypairs.json'"),
    };

    let mut private_key = String::from("");
    for key_pair in key_data {
        if key_pair["name"] == from {
            private_key.push_str(
                key_pair["private_key"]
                    .as_str()
                    .expect("Failed to fetch private key while creating transaction"),
            );
        }
    }

    // get the transaction signature and the signing key
    let (signature, signing_key) = KeyPair::sign(&hash, private_key);

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
    FileOps::write(TRANSACTIONS_PATH, "transactions", transaction);
    FileOps::write(SIGNING_DATA_PATH, "signing_data", signing_data);
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
    let mut base_data = FileOps::parse(BLOCKCHAIN_PATH);
    let blockchain = match base_data["blockchain"].as_array_mut() {
        Some(data) => data,
        None => panic!("Failed to parse blockchain data from 'blockchain.json'"),
    };

    let last_block = &blockchain[blockchain.len() - 1];
    // components of Block hash
    let mut nonce = 0;
    let previous_hash = &last_block["hash"].to_string().replace("\"", "");
    let mut transactions = FileOps::parse(TRANSACTIONS_PATH);
    // set mining difficulty
    let leading_zeros = String::from("0".repeat(2));
    // get block hash
    let mut hash = hash_block(
        &nonce.to_string(),
        &previous_hash.clone(),
        &transactions.to_string(),
    );

    // compute the correct hash to mine a new Block (00...98de872911a5e etc)
    while !hash.starts_with(&leading_zeros) {
        nonce += 1;
        hash = hash_block(
            &nonce.to_string(),
            &previous_hash.clone(),
            &transactions.to_string(),
        );
    }

    // get the current timestamp
    let now = Utc::now();
    let timestamp = now.to_rfc3339();

    // get the merkle root of this Blocks Transactions
    let merkle_root = get_merkle_root(TRANSACTIONS_PATH);

    // pay all transactions
    let transaction_data = transactions["transactions"].as_array_mut().unwrap();
    for t in transaction_data {
        if t["from_address"] == "REWARD" {
            Wallet::update_balance(
                t["to_address"].to_string(),
                t["amount"].as_i64().unwrap() as i32,
                "add",
            );
        } else {
            Wallet::update_balance(
                t["to_address"].to_string(),
                t["amount"].as_i64().unwrap() as i32,
                "add",
            );
            Wallet::update_balance(
                t["from_address"].to_string(),
                t["amount"].as_i64().unwrap() as i32,
                "subtract",
            );
        }
    }

    let block = Block {
        timestamp,
        hash,
        previous_hash: previous_hash.to_string(),
        nonce,
        transactions,
        merkle_root,
    };

    FileOps::write(BLOCKCHAIN_PATH, "blockchain", block);
    Transaction::clear();
    Transaction::add_reward(name);
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
    let mut bc_base_data = FileOps::parse(BLOCKCHAIN_PATH);
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
            return false;
        }

        // validate the current block hash
        let hash = hash_block(
            &current_block["nonce"].to_string(),
            &current_block["previous_hash"].to_string().replace("\"", ""),
            &current_block["transactions"].to_string(),
        );

        if current_block["hash"].to_string().replace("\"", "") != hash {
            return false;
        }

        // validate transactions
        let transactions = &current_block["transactions"]["transactions"]
            .as_array_mut()
            .unwrap();
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
                return false;
            }

            // get signing key for this transaction
            let mut sd_base_data = FileOps::parse(SIGNING_DATA_PATH);
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
                        return false;
                    }
                }
            }
        }
    }
    true
}
