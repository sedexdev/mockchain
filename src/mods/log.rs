// std library
use std::fs;
use std::io::prelude::*;
use std::path::Path;

// 3rd party crates
use phf::phf_map;

// imports
use super::helpers::get_timestamp;
use crate::{LOG_FILE_PATH, LOG_PATH};

static LOG_MESSAGE_MAP: phf::Map<u8, &str> = phf_map! {
    1u8 => "Log file created",
    2u8 => "Data files initialised",
    3u8 => "Genesis block mined and appended to blockchain",
    4u8 => "Application data reset; wallet and key pair data preserved",
    5u8 => "Application data reset; wallet and key pair data deleted",
    6u8 => "New ECDSA key pair created and appended to 'keypairs.json'",
    7u8 => "New wallet initialised and appended to 'wallets.json'",
    8u8 => "Mining new block...",
    9u8 => "...mining started with a current difficulty of 2; block hash must start with 2 leading zeros",
    10u8 => "...SHA256 block hash computed",
    11u8 => "...SHA256 merkle root of block transactions computed",
    12u8 => "...all pending transactions including rewards processed and paid",
    13u8 => "...new block appended to blockchain successfully",
    14u8 => "...transaction data cleared",
    15u8 => "...reward transaction appended to 'transactions.json'",
    16u8 => "...mining complete",
    17u8 => "Processing new transaction...",
    18u8 => "...read senders and recipients public keys to start transaction",
    19u8 => "...SHA256 transaction computed",
    20u8 => "...extracted senders private key",
    21u8 => "...transaction signed with senders private key using ECDSA",
    22u8 => "...signing data appended to 'signing.json'",
    23u8 => "...new transaction appended to 'transactions.json'",
    24u8 => "Starting blockchain verification...",
    25u8 => "...bad SHA256 block hash in chain; this chain has been tampered with, verification failed",
    26u8 => "...block hashing is consistent",
    27u8 => "...bad SHA256 transaction hash in chain; this chain has been tampered with, verification failed",
    28u8 => "...transaction hashing is consistent",
    29u8 => "...bad transaction signature in chain; a transaction could not be verified using ECDSA verification, verification failed",
    30u8 => "...transaction signatures are consistent",
    31u8 => "...blockchain verification completed successfully",
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
    /// level: LogLevel   -> log level being recorded
    /// message: msg_key   -> log entry message
    /// ```
    ///
    /// # Returns
    /// Nothing
    pub fn new(level: LogLevel, msg_key: u8) {
        Log::record(&Log {
            level,
            timestamp: get_timestamp(),
            message: Log::get_log_message(msg_key),
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

    /// Get a valid log message from a HashMap of
    /// messages using <i8, String> as the key/value
    ///
    /// # Visibility
    /// public
    ///
    /// # Args
    /// ```
    /// key: u8 -> HashMap key to lookup
    /// ```
    ///
    /// # Returns
    /// ```
    /// String
    /// ```
    pub fn get_log_message(key: u8) -> String {
        if let Some(msg) = LOG_MESSAGE_MAP.get(&key) {
            msg.to_string()
        } else {
            "** UNABLE TO PROVIDE ENTRY DETAILS **".to_string()
        }
    }
}
