// std library
use std::fs;
use std::io::prelude::*;
use std::path::Path;

// 3rd party crates
use phf::phf_map;

// imports
use crate::{LOG_FILE_PATH, LOG_PATH};

static LOG_MESSAGE_MAP: phf::Map<u8, &str> = phf_map! {
    1u8 => "Log file created",
    2u8 => "",
    3u8 => "",
    4u8 => "",
    5u8 => "",
    6u8 => "",
    7u8 => "",
    8u8 => "",
    9u8 => "",
    10u8 => "",
    11u8 => "",
    12u8 => "",
    13u8 => "",
    14u8 => "",
    15u8 => "",
    16u8 => "",
    17u8 => "",
    18u8 => "",
    19u8 => "",
    20u8 => "",
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
    /// timestamp: String -> time of entry
    /// message: String   -> log entry message
    /// ```
    ///
    /// # Returns
    /// ```
    /// Log
    /// ```
    pub fn new(level: LogLevel, timestamp: String, message: String) -> Log {
        Log {
            level,
            timestamp,
            message,
        }
    }

    /// Initialize the log file if it doesn't already exist
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

    /// Clears the log file after blockchain re-initialization
    ///
    /// # Visibility
    /// public
    ///
    /// # Args
    /// None
    ///
    /// # Returns
    /// None
    pub fn clear() {
        let mut log_file = match fs::OpenOptions::new()
            .write(true)
            .open(LOG_FILE_PATH.as_path())
        {
            Ok(handle) => handle,
            Err(e) => panic!(
                "Failed to open log file, has {:?} been moved or deleted? {}",
                LOG_FILE_PATH.as_path(),
                e
            ),
        };

        if let Err(e) = writeln!(log_file, "") {
            panic!(
                "Failed to clear log file, has {:?} been moved or deleted? {}",
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
