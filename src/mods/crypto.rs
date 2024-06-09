// 3rd party crates
use serde::Serialize;

/// Defines a KeyPair object for storing private and public keys
/// 
/// # Visibility
/// public
/// 
/// # Fields
/// ```
/// name: String
/// public_key: String
/// private_key: String
/// ```
/// 
/// # Derives
/// ```
/// serde::Serialize, Debug
/// ```
#[derive(Serialize, Debug)]
pub struct KeyPair {
    pub name: String,
    pub public_key: String,
    pub private_key: String,
}
