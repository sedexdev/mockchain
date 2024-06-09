// modules
mod mods;

// imports
use crate::mods::block::Block;
use crate::mods::constants::{
    BLOCKCHAIN_PATH,
    KEYPAIRS_PATH,
    TRANSACTIONS_PATH,
    WALLETS_PATH
};
use crate::mods::crypto::KeyPair;
use crate::mods::file::FileOps;
use crate::mods::transaction::Transaction;
use crate::mods::wallet::Wallet;

fn main() {
    FileOps::init(false);

    let test_block = Block {
        timestamp: String::from("01/01/9999:00:00:00"),
        hash: String::from("0".repeat(64)),
        previous_hash: String::from("None"),
        nonce: 0,
        transactions: [],
        merkle_root: String::from("None"),
    };

    FileOps::write(BLOCKCHAIN_PATH, "blockchain", &test_block);
    println!("{}", FileOps::parse(BLOCKCHAIN_PATH));

    let test_transaction = Transaction {
        hash: String::from("0".repeat(64)),
        from_address: String::from("1".repeat(64)),
        to_address: String::from("2".repeat(64)),
        amount: 10,
        signature: String::from("4".repeat(64))
    };

    FileOps::write(TRANSACTIONS_PATH, "transactions", &test_transaction);
    println!("{}", FileOps::parse(TRANSACTIONS_PATH));
    
    let test_wallet = Wallet {
        name: String::from("TEST"),
        address: "0".repeat(64),
        balance: 50,
    };
    
    FileOps::write(WALLETS_PATH, "wallets", &test_wallet);
    println!("{}", FileOps::parse(WALLETS_PATH));
    
    let test_keypair = KeyPair {
        name: String::from("TEST"),
        public_key: "0".repeat(64),
        private_key: "1".repeat(64),
    };
    
    FileOps::write(KEYPAIRS_PATH, "keypairs", &test_keypair);
    println!("{}", FileOps::parse(KEYPAIRS_PATH));

}
