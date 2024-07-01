// std library
use std::fs;
use std::io::prelude::*;
use std::path::Path;

// 3rd party crates
use phf::phf_map;

// imports
use super::{helpers::get_timestamp, messaging::replace};
use crate::{LOG_FILE_PATH, LOG_PATH};

static LOG_MESSAGE_MAP: phf::Map<u8, &str> = phf_map! {
    1u8 => "Log file created",
    2u8 => "Data files initialised",
    3u8 => "Genesis block mined and appended to blockchain",
    4u8 => "Application data reset; wallet and key pair data preserved",
    5u8 => "Application data reset; wallet and key pair data deleted",
    6u8 => "New ECDSA key pair created for '{}' and appended to 'keypairs.json'",
    7u8 => "New wallet initialised for '{}' and appended to 'wallets.json'",
    8u8 => "Mining new block...",
    9u8 => "...mining started with a current difficulty of {}; block hash must start with <difficulty> leading zeros",
    10u8 => "...SHA256 block hash computed",
    11u8 => "...SHA256 merkle root of block transactions computed",
    12u8 => "...block mining reward of 50 tokens paid to {}",
    13u8 => "...processing transaction; {} tokens sent from {} to {}",
    14u8 => "...all pending transactions including rewards processed and paid",
    15u8 => "...new block appended to blockchain successfully",
    16u8 => "...transaction data cleared",
    17u8 => "...new reward transaction appended to 'transactions.json'",
    18u8 => "...mining complete",
    19u8 => "Processing new transaction...",
    20u8 => "...read {}'s and {}'s public keys to start transaction",
    21u8 => "...SHA256 transaction computed",
    22u8 => "...extracted {}'s private key",
    23u8 => "...transaction signed with {}'s private key using ECDSA",
    24u8 => "...signing data appended to 'signing.json'",
    25u8 => "...new transaction appended to 'transactions.json'",
    26u8 => "Starting blockchain verification...",
    27u8 => "...bad SHA256 block hash in chain on or before {}; this chain has been tampered with, verification failed",
    28u8 => "...block hashing is consistent",
    29u8 => "...bad SHA256 transaction hash in chain; this chain has been tampered with, verification failed",
    30u8 => "...transaction hashing is consistent",
    31u8 => "...bad transaction signature in chain; a transaction signed by {} could not be verified using ECDSA verification, verification failed",
    32u8 => "...transaction signatures are consistent",
    33u8 => "...blockchain verification completed successfully",
};

static LOG_PANIC_MAP: phf::Map<u8, &str> = phf_map! {
    1u8 => "Unable to parse genesis block transaction to json_serde {}: {}",
};

/// Log enum with log level states
///
/// # Visibility
/// public
///
/// # Variants
/// ```
/// Info
/// Warning
/// Error
/// ```
#[derive(Debug)]
pub enum LogLevel {
    INFO,
    WARNING,
    ERROR,
}

/// Log struct defining components of a log entry
///
/// # Visibility
/// public
///
/// # Fields
/// ```
/// level: LogLevel   -> log level being recorded
/// timestamp: String -> time of entry
/// message: String   -> log entry message
/// ```
pub struct Log {
    pub level: LogLevel,
    pub timestamp: String,
    pub message: String,
}

impl Log {
    /// New instance of Log
    ///
    /// # Visibility
    /// public
    ///
    /// # Args
    /// ```
    /// level: LogLevel  -> log level being recorded
    /// message: msg_key -> log entry message
    /// args: Option<Vec<String>> -> optional args to include in the message
    /// ```
    ///
    /// # Returns
    /// Nothing
    pub fn new(level: LogLevel, msg_key: u8, args: Option<Vec<String>>) {
        Log::record(&Log {
            level,
            timestamp: get_timestamp(),
            message: Log::get_log_message(msg_key, args, false),
        });
    }

    /// New instance of Log
    ///
    /// # Visibility
    /// public
    ///
    /// # Args
    /// ```
    /// level: LogLevel           -> log level being recorded
    /// message: msg_key          -> log entry message
    /// args: Option<Vec<String>> -> optional args to include in the message
    /// ```
    ///
    /// # Returns
    /// Nothing
    pub fn new_panic(level: LogLevel, msg_key: u8, args: Option<Vec<String>>) {
        Log::record(&Log {
            level,
            timestamp: get_timestamp(),
            message: Log::get_log_message(msg_key, args, true),
        });
    }

    /// Initialise the log file if it doesn't already exist
    ///
    /// # Visibility
    /// public
    ///
    /// # Args
    /// None
    ///
    /// # Returns
    /// Nothing
    pub fn init() {
        if !Path::new(LOG_FILE_PATH.as_path()).exists() {
            match fs::create_dir_all(LOG_PATH.as_path()) {
                Ok(_) => {}
                Err(e) => panic!("Error creating 'log.txt' file: {}", e),
            };
        }
    }

    /// Writes an entry to the log
    ///
    /// # Visibility
    /// public
    ///
    /// # Args
    /// None
    ///
    /// # Returns
    /// Nothing
    pub fn record(&self) {
        let mut log_file = match fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(LOG_FILE_PATH.as_path())
        {
            Ok(handle) => handle,
            Err(e) => panic!(
                "Failed to open log file, has {:?} been moved or deleted? {}",
                LOG_FILE_PATH.as_path(),
                e
            ),
        };

        let mut entry_text: String = String::new();
        entry_text.push_str(self.timestamp.as_str());
        match self.level {
            LogLevel::INFO => entry_text.push_str(format!(" [*{:?}] ", self.level).as_str()),
            LogLevel::WARNING => entry_text.push_str(format!(" [-{:?}] ", self.level).as_str()),
            LogLevel::ERROR => entry_text.push_str(format!(" [!{:?}] ", self.level).as_str()),
        }
        entry_text.push_str(self.message.as_str());

        if let Err(e) = writeln!(log_file, "{}", entry_text) {
            panic!(
                "Failed to write log entry, has {:?} been moved or deleted? {}",
                LOG_FILE_PATH.as_path(),
                e
            );
        }
    }

    /// Get a valid log message from a HashMap of messages
    /// using <u8, String> as the key/value and an optional
    /// Vec of arguments to show in the message
    ///
    /// # Visibility
    /// public
    ///
    /// # Args
    /// ```
    /// key: u8                   -> HashMap key to lookup
    /// args: Option<Vec<String>> -> optional args to include in the message
    /// panic_flag: bool          -> true if logging a panic, false otherwise
    /// ```
    ///
    /// # Returns
    /// ```
    /// String
    /// ```
    pub fn get_log_message(key: u8, args: Option<Vec<String>>, panic_flag: bool) -> String {
        let none_log: String = "** UNABLE TO PROVIDE ENTRY DETAILS **".to_string();

        let helper = |map: &phf::Map<u8, &str>| {
            if let Some(msg) = map.get(&key) {
                match args {
                    Some(vec) => return replace(msg.to_string(), vec),
                    None => return msg.to_string(),
                }
            } else {
                none_log.clone()
            }
        };

        match panic_flag {
            true => helper(&LOG_PANIC_MAP),
            false => helper(&LOG_MESSAGE_MAP),
        }
    }
}
