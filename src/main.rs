// modules
mod mods;

// std library
use std::{thread, time};
use std::collections::HashMap;

use mods::constants::BLOCKCHAIN_PATH;
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
    },
    repl::Repl,
};

fn main() {
    // let mut choices = HashMap::new();
    // choices.insert(1, create_wallet);
    // choices.insert(2, mine_block);    
    // choices.insert(3, create_transaction);    
    // choices.insert(4, mine_block);    
    // choices.insert(5, mine_block);    
    // choices.insert(6, mine_block);    
    // choices.insert(7, mine_block);    
    // choices.insert(8, mine_block);    
    // choices.insert(9, mine_block);    
    // choices.insert(10, mine_block);    


    FileOps::init(false);

    // sleep to allow init
    let half_sec = time::Duration::from_millis(500);
    thread::sleep(half_sec);

    Block::add_genesis_block();
    Repl::print_intro();

    loop {
        Repl::print_options();

        mine_block("TEST".to_string());

        // sleep to allow mining
        let one_sec = time::Duration::from_millis(1000);
        thread::sleep(one_sec);

        println!("{}", FileOps::parse(BLOCKCHAIN_PATH));

        break;
        
    }
}


// Use these notes to construct app logic

/*

    FileOps::init(false);

    Repl::print_intro();
    Repl::print_options();

    print!("Please enter a choice: ");
    let input: i32 = match Repl::get_input() {
        Some(val) => val,
        None => panic!("[-] Error getting command line input, value passed was not an integer"),
    };
    println!("Input = {:#?}", input);

    print!("Please enter a string: ");
    let input2: String = match Repl::get_input() {
        Some(val) => val,
        None => panic!("[-] Error getting command line string input"),
    };
    println!("Input = {:#?}", input2);

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

*/