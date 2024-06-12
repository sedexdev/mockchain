// 3rd party crates
use p256::ecdsa::{Signature, SigningKey};
use serde::Serialize;

/// Define a Signing object
/// 
/// # Visibility
/// public
/// 
/// # Fields
/// ```
/// hash: String,
/// from_address: String,
/// to_address: String,
/// amount: u32,
/// signature: String
/// ```
/// 
/// # Derives
/// ```
/// serde::Serialize, Debug
/// ```
#[derive(Serialize, Debug)]
pub struct Signing {
    pub name: String,
    pub hash: String,
    pub signing_key: String,
    pub signature: String
}
