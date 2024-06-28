// modules
mod mods;

// std library
use std::path::Path;
use std::{thread, time};

use mods::constants::{
    BLOCKCHAIN_PATH, KEYPAIRS_PATH, SIGNING_DATA_PATH, TRANSACTIONS_PATH, WALLETS_PATH,
};
use mods::wallet::Wallet;
// imports
use mods::{
    block::Block,
    file::FileOps,
    helpers::{create_transaction, create_wallet, mine_block, verify_chain},
    messaging::{display_msg, Message},
    repl::Repl,
};

fn main() {
    if !Path::new(BLOCKCHAIN_PATH).exists() {
        FileOps::init(false);

        // sleep to allow init
        let half_sec = time::Duration::from_millis(500);
        thread::sleep(half_sec);
    
        Block::add_genesis_block();
    }

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
                }
                _ => display_msg(Message::Failure(
                    "Please enter a valid integer to select an option".to_string(),
                    None,
                )),
            },
            None => display_msg(Message::Failure(
                "Please enter a valid integer to select an option".to_string(),
                None,
            )),
        };
    }
}

// Options helper functions

fn option1() {
    print!("Add a name for this wallet: ");
    let _ = match Repl::get_input() {
        Some(name) => {
            if Wallet::name_exists(&name) {
                display_msg(Message::Failure(
                    "Wallet with name '{}' already exists".to_string(),
                    Some(vec![name.clone()]),
                ));
                return;
            }
            display_msg(Message::Success(
                "Creating wallet for '{}'".to_string(),
                Some(vec![name.clone()]),
            ));
            create_wallet(name);
            display_msg(Message::Success("Wallet created".to_string(), None));
        }
        None => display_msg(Message::Failure("Invalid name".to_string(), None)),
    };
}

fn option2() {
    print!("Name of account mining this block: ");
    let _ = match Repl::get_input() {
        Some(name) => {
            if !Wallet::name_exists(&name) {
                display_msg(Message::Failure(
                    "No wallet found under name '{}'".to_string(),
                    Some(vec![name.clone()]),
                ));
            } else {
                mine_block(name.clone());
                display_msg(Message::Success(
                    "New block mined successfully. A reward transaction has been added for '{}'"
                        .to_string(),
                    Some(vec![name.clone()]),
                ));
            }
        }
        None => display_msg(Message::Failure("Invalid name".to_string(), None)),
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
                display_msg(Message::Failure(
                    "No wallet found under name '{}'".to_string(),
                    Some(vec![name.clone()]),
                ));
                return;
            }
            senders_name = name;
        }
        None => display_msg(Message::Failure("Invalid name".to_string(), None)),
    };
    print!("Name on recipients wallet: ");
    let _ = match Repl::get_input() {
        Some(name) => {
            if !Wallet::name_exists(&name) {
                display_msg(Message::Failure(
                    "No wallet found under name '{}'".to_string(),
                    Some(vec![name.clone()]),
                ));
                return;
            }
            recipients_name = name;
        }
        None => display_msg(Message::Failure("Invalid name".to_string(), None)),
    };
    print!("Amount: ");
    let _ = match Repl::get_input() {
        Some(val) => {
            amount = val;
            if val <= 0 {
                display_msg(Message::Failure("Choose an amount greater than 0".to_string(), None));
                return;
            }
            if Wallet::get_balance(&senders_name) < amount {
                display_msg(Message::Failure(
                    "Not enough funds to send {} from {}'s account".to_string(),
                    Some(vec![amount.to_string(), senders_name.clone()]),
                ));
                return;
            }
            display_msg(Message::Success(
                "Adding new pending transaction\n".to_string(),
                None,
            ));
            println!(
                "\tSenders public key: {}",
                Wallet::get_wallet_address(&senders_name).unwrap()
            );
            println!(
                "\tRecipients public key: {}",
                Wallet::get_wallet_address(&recipients_name).unwrap()
            );
            println!("\tAmount: {}\n", &amount);
            create_transaction(senders_name, recipients_name, amount);
            display_msg(Message::Success(
                "Transaction added successfully".to_string(),
                None,
            ));
        }
        None => display_msg(Message::Failure(
            "Please enter a positive whole number".to_string(),
            None,
        )),
    };
}

fn option9() {
    display_msg(Message::Warning("!! This action will wipe out the current blockchain and transaction data. Continue? (y/n) ".to_string(), None));
    let wipe: String = match Repl::get_input() {
        Some(val) => val,
        None => "Invalid option".to_string(),
    };
    match wipe.as_str() {
        "y" => {
            display_msg(Message::Warning(
                "Would you like to preserve existing wallets? (y/n) ".to_string(),
                None,
            ));
            let keep: String = match Repl::get_input() {
                Some(val) => val,
                None => "Invalid option".to_string(),
            };
            match keep.as_str() {
                "y" => {
                    display_msg(Message::Success(
                        "Re-initializing blockchain...".to_string(),
                        None,
                    ));
                    FileOps::init(true);
                    display_msg(Message::Success(
                        "Blockchain init completed successfully".to_string(),
                        None,
                    ));
                    display_msg(Message::Success(
                        "Wallet data has been preserved".to_string(),
                        None,
                    ));
                }
                "n" => {
                    display_msg(Message::Success(
                        "Re-initializing blockchain...".to_string(),
                        None,
                    ));
                    FileOps::init(false);
                    display_msg(Message::Success(
                        "Blockchain init completed successfully".to_string(),
                        None,
                    ));
                    display_msg(Message::Success(
                        "Wallet data has been deleted".to_string(),
                        None,
                    ));
                }
                "Invalid option" => display_msg(Message::Failure(wipe, None)),
                _ => display_msg(Message::Failure(wipe, None)),
            }
        }
        "n" => display_msg(Message::Success("Operation cancelled".to_string(), None)),
        "Invalid option" => display_msg(Message::Failure(wipe, None)),
        _ => display_msg(Message::Failure(wipe, None)),
    };
}
