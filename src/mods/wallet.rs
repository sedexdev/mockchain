// 3rd party crates
use serde::Serialize;

// imports
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
    /// path: &str    -> file path to check
    /// name: &String -> name to check for
    /// ```
    /// 
    /// # Returns
    /// ```
    /// bool
    /// ```
    pub fn name_exists(path: &str, name: &String) -> bool {
        let mut json_obj = FileOps::parse(path);
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
    /// path: &str   -> file path to check
    /// name: String -> name to get address of
    /// ```
    /// 
    /// # Returns
    /// ```
    /// Option<String>
    /// ```
    pub fn get_wallet_address(path: &str, name: String) -> Option<String> {
        if !Wallet::name_exists(path, &name) {
            None
        } else {
            let mut json_obj = FileOps::parse(path);
            let wallets = json_obj["wallets"].as_array_mut().unwrap();

            let mut wallet_name = String::from("");

            for wallet in wallets {
                if wallet["name"] == name {
                    wallet_name.push_str(wallet["address"].to_string().as_str());
                }
            }
            Some(wallet_name)
        }
    }

    /// Adds value to the wallet balance after
    /// a transaction
    /// 
    /// # Visibility
    /// public
    /// 
    /// # Args
    /// ```
    /// amount: u32 -> amount to increment the balance by
    /// ```
    /// 
    /// # Returns
    /// Nothing
    pub fn increment_balance(&mut self, amount: i32) {
        self.balance += amount;
    }

    /// Subtracts value from the wallet balance after
    /// a transaction
    /// 
    /// # Visibility
    /// public
    /// 
    /// # Args
    /// ```
    /// amount: u32 -> amount to decrement the balance by
    /// ```
    /// 
    /// # Returns
    /// ```
    /// Result<bool, bool>
    /// ```
    pub fn decrement_balance(&mut self, amount: i32) -> Result<bool, bool> {
        let cp_balance = self.balance.clone();
        if cp_balance - &amount >= 0 {
            self.balance -= amount;
            return Ok(true);
        }
        Err(false)
    }

    /// Gets the current balance of this Wallet
    /// 
    /// # Visibility
    /// public
    /// 
    /// # Args
    /// None
    /// 
    /// # Returns
    /// ```
    /// i32
    /// ```
    pub fn get_balance(&self) -> i32 {
        self.balance.clone()
    }
}


// Testing
#[cfg(test)]
mod test_wallet {
    use super::*;

    use std::{thread, time};

    use crate::mods::constants::WALLETS_PATH_TEST;
    use crate::mods::file::FileOps;

    #[test]
    fn test_name_exists() {
        let wallet = Wallet {
            name: String::from("Bingo"),
            address: String::from("0".repeat(130)),
            balance: 100
        };

        FileOps::write(WALLETS_PATH_TEST, "wallets", &wallet);

        // sleep to allow file init and exists tests
        let one_sec = time::Duration::from_millis(1000);
        thread::sleep(one_sec);

        assert!(Wallet::name_exists(WALLETS_PATH_TEST, &wallet.name));
        assert!(!Wallet::name_exists(WALLETS_PATH_TEST, &String::from("TEST ACCOUNT")));
    }

    #[test]
    fn test_get_wallet_address() {
        let wallet = Wallet {
            name: String::from("Bingo2"),
            address: String::from("0".repeat(130)),
            balance: 100
        };

        FileOps::write(WALLETS_PATH_TEST, "wallets", &wallet);

        // sleep to allow file init and exists tests
        let one_sec = time::Duration::from_millis(1000);
        thread::sleep(one_sec);

        let address = match Wallet::get_wallet_address(WALLETS_PATH_TEST, String::from("Bingo2")) {
            Some(addr) => addr,
            None => String::from("Address not found"),
        }; 

        assert_eq!(132, address.len());
    }

    #[test]
    fn test_increment_balance() {

    }

    #[test]
    fn test_decrement_balance() {

    }

    #[test]
    fn test_get_balance() {

    }
}
