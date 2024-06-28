/*
    Base structures for initializing data files
*/

// 3rd party crates
use serde::Serialize;
use serde_json::Value;

/// Blockchain base structure
///
/// # Visibility
/// public
///
/// # Fields
/// ```
/// blockchain: [Value; 0]
/// ```
///
/// # Derives
/// ```
/// serde::Serialize, Debug
/// ```
#[derive(Serialize, Debug)]
pub struct Blockchain {
    pub blockchain: [Value; 0],
}

/// Transactions base structure
///
/// # Visibility
/// public
///
/// # Fields
/// ```
/// transactions: [Value; 0]
/// ```
///
/// # Derives
/// ```
/// serde::Serialize, Debug
/// ```
#[derive(Serialize, Debug)]
pub struct Transactions {
    pub transactions: [Value; 0],
}

/// Wallets base structure
///
/// # Visibility
/// public
///
/// # Fields
/// ```
/// wallets: [Value; 0]
/// ```
///
/// # Derives
/// ```
/// serde::Serialize, Debug
/// ```
#[derive(Serialize, Debug)]
pub struct Wallets {
    pub wallets: [Value; 0],
}

/// Keypairs base structure
///
/// # Visibility
/// public
///
/// # Fields
/// ```
/// keypairs: [Value; 0]
/// ```
///
/// # Derives
/// ```
/// serde::Serialize, Debug
/// ```
#[derive(Serialize, Debug)]
pub struct KeyPairs {
    pub keypairs: [Value; 0],
}

/// SigningData base structure
///
/// # Visibility
/// public
///
/// # Fields
/// ```
/// signing_data: [Value; 0]
/// ```
///
/// # Derives
/// ```
/// serde::Serialize, Debug
/// ```
#[derive(Serialize, Debug)]
pub struct SigningData {
    pub signing_data: [Value; 0],
}
