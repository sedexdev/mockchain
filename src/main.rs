// modules
mod mods;

// std library
use std::{thread, time};

// imports
use mods::{
    block::Block,
    crypto::KeyPair, 
    file::FileOps,
    helpers::{
        create_transaction,
        create_wallet,
        mine_block,
        verify_chain,
    }
};

fn main() {
    FileOps::init(false);

    // sleep to allow init
    let one_sec = time::Duration::from_millis(1000);
    thread::sleep(one_sec);

    Block::add_genesis_block();
    
    create_wallet(String::from("TEST"));
    create_wallet(String::from("TEST2"));

    // sleep to allow wallet creation
    let one_sec = time::Duration::from_millis(1000);
    thread::sleep(one_sec);

    let from_address = KeyPair::get_key(String::from("TEST"), String::from("public"));
    let to_address = KeyPair::get_key(String::from("TEST2"), String::from("public"));

    create_transaction(
        String::from("TEST"),
        from_address,
        to_address,
        10,
    );

    mine_block(String::from("TEST"));
    mine_block(String::from("TEST2"));
    mine_block(String::from("TEST"));
    mine_block(String::from("TEST2"));
    mine_block(String::from("TEST"));

    // sleep to allow everything to update
    let one_sec = time::Duration::from_millis(1000);
    thread::sleep(one_sec);

    if verify_chain() {
        println!("[+] CHAIN IS VALID");
    } else {
        println!("[-] CHAIN IS INVALID");
    }; 
}
