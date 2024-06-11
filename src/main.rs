// modules
mod mods;

// imports
use mods::crypto::KeyPair;
use crate::mods::constants::{
    BLOCKCHAIN_PATH,
    KEYPAIRS_PATH,
    TRANSACTIONS_PATH,
    WALLETS_PATH
};
use crate::mods::file::FileOps;

fn main() {
    let data_files = [BLOCKCHAIN_PATH, KEYPAIRS_PATH, TRANSACTIONS_PATH, WALLETS_PATH];
    FileOps::init(false, data_files);
}
