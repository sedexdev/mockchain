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
    blockchain: [Value; 0]
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain {
            blockchain: []
        }
    }
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
    transactions: [Value; 0]
}

impl Transactions {
    pub fn new() -> Transactions {
        Transactions {
            transactions: []
        }
    }
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
    wallets: [Value; 0]
}

impl Wallets {
    pub fn new() -> Wallets {
        Wallets {
            wallets: []
        }
    }
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
    keypairs: [Value; 0]
}

impl KeyPairs {
    pub fn new() -> KeyPairs {
        KeyPairs {
            keypairs: []
        }
    }
}
