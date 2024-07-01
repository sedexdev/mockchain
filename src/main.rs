#[macro_use]
extern crate lazy_static;

// modules
mod mods;

// std library
use std::path::{Path, PathBuf};
use std::{thread, time};

// 3rd party crates
use dirs::home_dir;

// imports
use mods::{
    block::Block,
    file::FileOps,
    helpers::{create_transaction, create_wallet, mine_block, verify_chain},
    log::{Log, LogLevel},
    messaging::{display_msg, Message},
    repl::Repl,
    wallet::Wallet,
};

// static references to data file paths

lazy_static! {
    #[derive(Debug)]
    static ref HOME: PathBuf = {
        match home_dir() {
            Some(dir) => dir,
            None => panic!("Home directory not found to write data files, aborting"),
        }
    };
}

lazy_static! {
    #[derive(Debug)]
    static ref DATA_PATH: PathBuf = HOME.as_path().join(".mockchain").join("data");
}

lazy_static! {
    #[derive(Debug)]
    static ref LOG_PATH: PathBuf = HOME.as_path().join(".mockchain").join("log");
}

lazy_static! {
    #[derive(Debug)]
    static ref BLOCKCHAIN_PATH: PathBuf = DATA_PATH.as_path().join("blockchain.json");
}

lazy_static! {
    #[derive(Debug)]
    static ref KEYPAIRS_PATH: PathBuf = DATA_PATH.as_path().join("keypairs.json");
}

lazy_static! {
    #[derive(Debug)]
    static ref SIGNING_DATA_PATH: PathBuf = DATA_PATH.as_path().join("signing.json");
}

lazy_static! {
    #[derive(Debug)]
    static ref TRANSACTIONS_PATH: PathBuf = DATA_PATH.as_path().join("transactions.json");
}

lazy_static! {
    #[derive(Debug)]
    static ref WALLETS_PATH: PathBuf = DATA_PATH.as_path().join("wallets.json");
}

lazy_static! {
    #[derive(Debug)]
    static ref LOG_FILE_PATH: PathBuf = LOG_PATH.as_path().join("log.txt");
}

fn main() {
    if !Path::new(BLOCKCHAIN_PATH.as_path()).exists() {
        Log::init();

        Log::new(LogLevel::INFO, 1);

        FileOps::init(false);

        Log::new(LogLevel::INFO, 2);

        // sleep to allow init
        let half_sec = time::Duration::from_millis(500);
        thread::sleep(half_sec);

        Block::add_genesis_block();

        Log::new(LogLevel::INFO, 3);
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
                4 => println!("\n{:#?}\n", FileOps::parse(BLOCKCHAIN_PATH.as_path())),
                5 => println!("\n{:#?}\n", FileOps::parse(TRANSACTIONS_PATH.as_path())),
                6 => println!("\n{:#?}\n", FileOps::parse(WALLETS_PATH.as_path())),
                7 => println!("\n{:#?}\n", FileOps::parse(KEYPAIRS_PATH.as_path())),
                8 => println!("\n{:#?}\n", FileOps::parse(SIGNING_DATA_PATH.as_path())),
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
                display_msg(Message::Failure(
                    "Choose an amount greater than 0".to_string(),
                    None,
                ));
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
            if let Some(key) = Wallet::get_wallet_address(&senders_name) {
                println!("\tSenders public key: {}", key);
            }
            if let Some(key) = Wallet::get_wallet_address(&recipients_name) {
                println!("\tRecipients public key: {}", key);
            }
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
    fn helper(preserve: bool) {
        display_msg(Message::Success(
            "Re-initialising blockchain...".to_string(),
            None,
        ));
        FileOps::init(preserve);
        display_msg(Message::Success(
            "Blockchain init completed successfully".to_string(),
            None,
        ));
        let (msg, msg_key) = match preserve {
            true => ("Wallet data has been preserved", 4),
            false => ("Wallet data has been deleted", 5),
        };
        display_msg(Message::Success(msg.to_string(), None));
        Log::new(LogLevel::WARNING, msg_key);
    }

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
                    helper(true);
                }
                "n" => {
                    helper(false);
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
