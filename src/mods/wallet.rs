// 3rd party crates
use serde::Serialize;

/// Defines a Wallet object with name, address, and balance
/// 
/// # Visibility
/// public
/// 
/// # Fields
/// ```
/// name: String
/// address: String
/// balance: u32
/// ``` 
/// 
/// # Derives
/// ```
/// serde::Serialize, Debug
/// ```
#[derive(Serialize, Debug)]
pub struct Wallet {
    pub name: String,
    pub address: String,
    pub balance: i32,
}
