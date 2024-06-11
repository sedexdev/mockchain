// modules
mod mods;

// imports
use crate::mods::constants::{
    BLOCKCHAIN_PATH,
    KEYPAIRS_PATH,
    TRANSACTIONS_PATH,
    WALLETS_PATH
};
use crate::mods::file::FileOps;
use crate::mods::crypto::KeyPair;

fn main() {
    let data_files = [BLOCKCHAIN_PATH, KEYPAIRS_PATH, TRANSACTIONS_PATH, WALLETS_PATH];
    FileOps::init(false, data_files);

    KeyPair::generate(String::from("TEST"), KEYPAIRS_PATH);
}
