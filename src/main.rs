// modules
mod mods;

// std library
use std::{thread, time};

use mods::constants::{BLOCKCHAIN_PATH, KEYPAIRS_PATH, SIGNING_DATA_PATH, TRANSACTIONS_PATH, WALLETS_PATH};
use mods::wallet::Wallet;
// imports
use mods::{
    block::Block,
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
    FileOps::init(false);

    // sleep to allow init
    let half_sec = time::Duration::from_millis(500);
    thread::sleep(half_sec);

    Block::add_genesis_block();
    
    Repl::print_intro();
    Repl::print_options();

    loop {

        print!("Select an option: ");
        let _ = match Repl::get_input() {
            Some(choice) => match choice {
                0 => Repl::print_options(),
                1 => option1(),
                2 => option2(),
                3 => option3(),
                4 => println!("\n{:#?}\n", FileOps::parse(BLOCKCHAIN_PATH)),
                5 => println!("\n{:#?}\n", FileOps::parse(TRANSACTIONS_PATH)),
                6 => println!("\n{:#?}\n", FileOps::parse(WALLETS_PATH)),
                7 => println!("\n{:#?}\n", FileOps::parse(KEYPAIRS_PATH)),
                8 => println!("\n{:#?}\n", FileOps::parse(SIGNING_DATA_PATH)),
                9 => option9(),
                10 => println!("VALID CHAIN: {}", verify_chain()),
                11 => {
                    println!("See you again soon! 👋 Your data files will be preserved 😃");
                    break;
                },
                _ => println!("[-] Please enter a valid integer to select an option")
            },
            None => println!("[-] Please enter a valid integer to select an option"),
        };
    }
}

// Options helper functions

fn option1() {
    print!("Add a name for this wallet: ");
    let _ = match Repl::get_input() {
        Some(name) => {
            if Wallet::name_exists(&name) {
                println!("[-] Wallet with name '{}' already exists", &name);
                return;
            }
            println!("[+] Creating wallet for '{}'", &name);
            create_wallet(name);
            println!("[+] Wallet created");
        },
        None => println!("[-] Invalid name"),
    };
}

fn option2() {
    print!("Name of account mining this block: ");
    let _ = match Repl::get_input() {
        Some(name) => {
            if !Wallet::name_exists(&name) {
                println!("[-] No wallet found under name '{}'", &name);
            } else {
                mine_block(name.clone());
                println!("[+] New block mined successfully. A reward transaction has been added for '{}'", &name);
            }
        },
        None => println!("[-] Invalid name"),
    };
}

fn option3() {
    let mut senders_name = String::new();
    let mut recipients_name = String::new();
    let amount: i32;
    print!("Name on senders wallet: ");
    let _ = match Repl::get_input() {
        Some(name) => {
            if !Wallet::name_exists(&name) {
                println!("[-] No wallet found under name '{}'", &name);
                return;
            }
            senders_name = name;
        },
        None => println!("[-] Invalid name"),
    };
    print!("Name on recipients wallet: ");
    let _ = match Repl::get_input() {
        Some(name) => {
            if !Wallet::name_exists(&name) {
                println!("[-] No wallet found under name '{}'", &name);
                return;
            }
            recipients_name = name;
        },
        None => println!("[-] Invalid name"),
    };
    print!("Amount: ");
    let _ = match Repl::get_input() {
        Some(val) => {
            amount = val;
            if Wallet::get_balance(&senders_name) < amount {
                println!("[-] Not enough funds to send {} from {}'s account", &amount, &senders_name);
                return;
            }
            println!("[+] Adding new pending transaction\n");
            println!("\tSenders public key: {}", Wallet::get_wallet_address(&senders_name).unwrap());
            println!("\tRecipients public key: {}", Wallet::get_wallet_address(&recipients_name).unwrap());
            println!("\tAmount: {}", &amount);
            create_transaction(senders_name, recipients_name, amount);
            println!("\n[+] Transaction added successfully");
        },
        None => println!("[-] Please enter a positive whole number"),
    };
}

fn option9() {
    print!("!! This action will wipe out the current blockchain and transaction data. Continue? (y/n) ");
    let wipe: String = match Repl::get_input() {
        Some(val) => val,
        None => "[-] Invalid option".to_string(),
    };
    match wipe.as_str() {
        "y" => {
            print!("Would you like to preserve existing wallets? (y/n) ");
            let keep: String = match Repl::get_input() {
                Some(val) => val,
                None => "[-] Invalid option".to_string(),
            };
            match keep.as_str() {
                "y" => {
                    println!("[+] Re-initializing blockchain...");
                    FileOps::init(true);
                    println!("[+] Blockchain init completed successfully");
                    println!("[+] Wallet data has been preserved");
                },
                "n" => {
                    println!("[+] Re-initializing blockchain...");
                    FileOps::init(false);
                    println!("[+] Blockchain init completed successfully");
                    println!("[+] Wallet data has been deleted");
                },
                "[-] Invalid option" => println!("{}", wipe),
                _ => println!("{}", wipe),
            }
        },
        "n" => println!("[+] Operation cancelled"),
        "[-] Invalid option" => println!("{}", wipe),
        _ => println!("{}", wipe),
    };
}
