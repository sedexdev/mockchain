use std::fmt::Debug;
use std::str;

// 3rd party crates
use hex::{decode, encode};
use p256::{
    ecdsa::{
        signature::{Signer, Verifier}, Signature, SigningKey, VerifyingKey,
    },
    SecretKey
};
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

    /// Signs a transaction on the blockchain using
    /// the account holders private key
    /// 
    /// # Visibility
    /// public
    /// 
    /// # Args
    /// ```
    /// hash: String        -> transaction hash to sign
    /// private_key: String -> private key to sign with 
    /// ```
    /// 
    /// # Returns
    /// ```
    /// (String, String)
    /// ```
    pub fn sign(hash: &String, private_key: String) -> (String, String) {
        let key_bytes = decode(&private_key).unwrap();
        let signing_key = match SigningKey::from_slice(key_bytes.as_slice()) {
            Ok(key) => key,
            Err(e) => panic!("Cannot decode signing key: {}", e),
        };
        let signature: Signature = signing_key.sign(&hash.as_bytes());
        (encode(signature.to_bytes()), encode(signing_key.to_bytes()))
    }
    
    /// Verifies a transaction on the blockchain using
    /// the account holders public key
    /// 
    /// # Visibility
    /// public
    /// 
    /// # Args
    /// ```
    /// signature: String -> signature to verify
    /// hash: String      -> transaction hash to verify
    /// ```
    /// 
    /// # Returns
    /// ```
    /// bool
    /// ```
    pub fn verify(signature: Signature, signing_key: SigningKey, hash: String) -> bool {
        let verifying_key = VerifyingKey::from(&signing_key);
        let verified = match verifying_key.verify(&hash.as_bytes(), &signature) {
            Ok(_res) => true,
            Err(_) => false,
        };
        verified
    }

    /// Extract Signature and SigningKey objects from encoded
    /// hex strings
    /// 
    /// # Visibility
    /// public
    /// 
    /// # Args
    /// ```
    /// signature: String   -> hex encoded Signture object
    /// signing_key: String -> hex encoded SigningKey object
    /// ```
    /// 
    /// # Returns
    /// ```
    /// (Signature, SigningKey)
    /// ```
    pub fn extract(signature: String, signing_key: String) -> (Signature, SigningKey){
        // decode and extract Signature and SigningKey objects
        let sig = match decode(signature) {
            Ok(bytes) => match Signature::from_slice(bytes.as_slice()) {
                Ok(s) => s,
                Err(e) => panic!("Cannot decode signature: {}", e),
            },
            Err(e) => panic!("Cannot decode signature: {}", e), 
        };
        let sign = match decode(signing_key) {
            Ok(bytes) => match SigningKey::from_slice(bytes.as_slice()) {
                Ok(s) => s,
                Err(e) => panic!("Cannot decode signing key: {}", e),
            },
            Err(e) => panic!("Cannot decode signing key: {}", e), 
        };
        (sig, sign)
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

    #[test]
    fn test_sign_extract_verify() {
        // test hash and private key
        let test_hash = "0".repeat(64);
        let test_private_key = String::from("4cae0e746defac95cba2dd5cdb440bb54d102713aeedcad19a483851c0a5ef21");
        
        // get a signature and signing key by signing the test hash
        let (sig, key) = KeyPair::sign(&test_hash, test_private_key);

        // extract the Signature and SigningKey
        let (signature, signing_key) = KeyPair::extract(sig, key);

        // assert verification of hash signature
        assert!(KeyPair::verify(signature, signing_key, test_hash));
    }
}
