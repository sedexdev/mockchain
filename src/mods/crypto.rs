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
}


// Testing
#[cfg(test)]
mod test_crypto {
    use super::*;

    use crate::mods::constants::KEYPAIRS_PATH_TEST;

    #[test]
    fn test_generate() {
        let name = String::from("TEST");
        KeyPair::generate(name, KEYPAIRS_PATH_TEST);
        let mut json_obj = FileOps::parse(KEYPAIRS_PATH_TEST);
        assert_eq!("TEST", json_obj["keypairs"].as_array_mut().unwrap()[0]["name"]);
    }
}
