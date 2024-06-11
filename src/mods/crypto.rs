// 3rd party crates
use hex::encode;
use p256::SecretKey;
use rand_core::OsRng;
use serde::Serialize;

// imports
use crate::mods::file::FileOps;

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

impl KeyPair {

    /// Creates a new key pair including a public
    /// and private key
    /// 
    /// # Visibility
    /// public
    /// 
    /// # Args
    /// ```
    /// name: String -> name of the account for this key pair
    /// ```
    /// 
    /// # Returns
    /// Nothing
    pub fn generate(name: String, path: &str) {
        // private key first
        let secret = SecretKey::random(&mut OsRng);
        let private_key = encode(&secret.to_bytes());
        // then public key
        let public_key = encode(secret.public_key().to_sec1_bytes());
        let key_pair = KeyPair {
            name,
            public_key,
            private_key,
        };
        FileOps::write(path, "keypairs", key_pair);
    }

    /// Gets a key from keypairs.json file
    /// 
    /// # Visibility
    /// public
    /// 
    /// # Args
    /// ```
    /// name: String -> name of account to get key from
    /// key: String  -> [public | private]
    /// path:&str    -> file to read key from
    /// ```
    /// 
    /// # Returns
    /// ```
    /// String
    /// ```
    pub fn get_key(name: String, key: String, path: &str) -> String {
        let mut json_obj = FileOps::parse(path);
        let key_arr = json_obj["keypairs"].as_array_mut().unwrap();
        for k in key_arr {
            if k["name"] == name {
                let mut id = key.to_owned();
                id.push_str("_key");
                return k[id].to_string();
            }
        }
        format!("No keypair found under {name}").to_string()
    }
}


// Testing
#[cfg(test)]
mod test_crypto {
    use super::*;

    use std::{thread, time};

    use crate::mods::constants::KEYPAIRS_PATH_TEST;

    #[test]
    fn test_generate() {
        let name = String::from("TEST");
        KeyPair::generate(name, KEYPAIRS_PATH_TEST);
        let mut json_obj = FileOps::parse(KEYPAIRS_PATH_TEST);
        assert_eq!("TEST", json_obj["keypairs"].as_array_mut().unwrap()[0]["name"]);
    }

    #[test]
    fn test_get_key() {
        let one_sec = time::Duration::from_millis(1000);
        thread::sleep(one_sec);

        let name = String::from("TEST2");
        KeyPair::generate(name, KEYPAIRS_PATH_TEST);
        assert_eq!(66, KeyPair::get_key(String::from("TEST2"), String::from("private"), KEYPAIRS_PATH_TEST).len());
    }

    #[test]
    fn test_get_key_fails() {
        let two_sec = time::Duration::from_millis(2000);
        thread::sleep(two_sec);

        let name = String::from("TEST3");
        KeyPair::generate(name, KEYPAIRS_PATH_TEST);
        assert!(KeyPair::get_key(String::from("TEST10"), String::from("private"), KEYPAIRS_PATH_TEST).contains("No keypair found"));
    }
}
