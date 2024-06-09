// 3rd party crates
use serde::Serialize;

/// Define a Transaction object
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
pub struct Transaction {
    pub hash: String,
    pub from_address: String,
    pub to_address: String,
    pub amount: i32,
    pub signature: String
}
