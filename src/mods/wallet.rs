// 3rd party crates
use serde::Serialize;

// imports
use super::constants::WALLETS_PATH;
use super::file::FileOps;

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

impl Wallet {

    /// Checks to see if a name has already been used
    /// 
    /// # Visibility
    /// public
    /// 
    /// # Args
    /// ```
    /// name: &String -> name to check for
    /// ```
    /// 
    /// # Returns
    /// ```
    /// bool
    /// ```
    pub fn name_exists(name: &String) -> bool {
        let mut json_obj = FileOps::parse(WALLETS_PATH);
        let wallets = json_obj["wallets"].as_array_mut().unwrap(); 
        for wallet in wallets {
            if wallet["name"] == *name {
                return true;
            }
        }
        false
    }

    /// Reads the public key address of a wallet from
    /// wallets.json and returns it as a String
    /// 
    /// # Visibility
    /// public
    /// 
    /// # Args
    /// ```
    /// name: String -> name to get address of
    /// ```
    /// 
    /// # Returns
    /// ```
    /// Option<String>
    /// ```
    pub fn get_wallet_address(name: &String) -> Option<String> {
        if !Wallet::name_exists(name) {
            None
        } else {
            let mut json_obj = FileOps::parse(WALLETS_PATH);
            let wallets = json_obj["wallets"].as_array_mut().unwrap();

            let mut wallet_name = String::from("");

            for wallet in wallets {
                if wallet["name"] == *name {
                    wallet_name.push_str(wallet["address"].to_string().as_str());
                }
            }
            Some(wallet_name)
        }
    }

    /// Updates the value of the wallet balance after
    /// a transaction has been added to a block
    /// 
    /// # Visibility
    /// public
    /// 
    /// # Args
    /// ```
    /// address: String -> wallet address to update
    /// amount: i32     -> amount to increment balance by
    /// op: &str        -> "add" | "subtract" 
    /// ```
    /// 
    /// # Returns
    /// Nothing
    pub fn update_balance(address: String, amount: i32, op: &str) {
        let mut base_data = FileOps::parse(WALLETS_PATH);
        let wallets = base_data["wallets"].as_array_mut().unwrap();
        for wallet in wallets {
            if wallet["address"].to_string() == address {
                let mut balance = wallet["balance"].as_i64().unwrap() as i32;
                if op == "add" { balance += amount; } 
                if op == "subtract" { balance -= amount; }
                FileOps::write_balance(address, balance);
                break;
            }
        }
    }

    /// Gets the current balance of this Wallet
    /// 
    /// # Visibility
    /// public
    /// 
    /// # Args
    /// ```
    /// name: &String -> name of account to lookup
    /// ```
    /// 
    /// # Returns
    /// ```
    /// i32
    /// ```
    pub fn get_balance(name: &String) -> i32 {
        let mut balance: i32 = 0;
        let mut base_data = FileOps::parse(WALLETS_PATH);
        let wallets = base_data["wallets"].as_array_mut().unwrap();
        for wallet in wallets {
            if wallet["name"] == *name {
                balance = wallet["balance"].as_i64().unwrap() as i32;
                break;
            }
        }
        balance
    }
}
