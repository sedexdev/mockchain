use std::fmt::Debug;
use std::str;

// 3rd party crates
use hex::{decode, encode};
use p256::{
    ecdsa::{
        signature::{Signer, Verifier},
        Signature, SigningKey, VerifyingKey,
    },
    SecretKey,
};
use rand_core::OsRng;
use rs_merkle::{algorithms::Sha256, Hasher, MerkleTree};
use serde::Serialize;
use sha256::digest;

// imports
use super::constants::{DELIMITER, KEYPAIRS_PATH};
use super::file::FileOps;

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
    /// ```
    /// KeyPair
    /// ```
    pub fn generate(name: String) -> KeyPair {
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
        key_pair
    }

    /// Gets a key from keypairs.json file
    ///
    /// # Visibility
    /// public
    ///
    /// # Args
    /// ```
    /// name: String -> name of account to get key from
    /// key: String  -> ["public" | "private"]
    /// ```
    ///
    /// # Returns
    /// ```
    /// String
    /// ```
    pub fn get_key(name: String, key: String) -> String {
        let mut base_data = FileOps::parse(KEYPAIRS_PATH);
        let keypairs = match base_data["keypairs"].as_array_mut() {
            Some(data) => data,
            None => panic!("Key pair data not found, has the file been moved or deleted?"),
        };
        for k in keypairs {
            if k["name"] == name {
                let mut id = String::from("");
                id.push_str(key.to_owned().as_str());
                id.push_str("_key");
                // replace " in returned Value with empty string
                return k[id].to_string().replace("\"", "");
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
        let key_bytes = match decode(&private_key) {
            Ok(key) => key,
            Err(e) => panic!(
                "Failed to decode private key while signing transaction: {}",
                e
            ),
        };
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
    /// signature: String   -> hex encoded Signature object
    /// signing_key: String -> hex encoded SigningKey object
    /// ```
    ///
    /// # Returns
    /// ```
    /// (Signature, SigningKey)
    /// ```
    pub fn extract(signature: String, signing_key: String) -> (Signature, SigningKey) {
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

/// Creates a SHA256 hash of the components of
/// a block. The delimiter aims to prevent an
/// attack where the string components of the
/// hash are combined in a different segments
/// e.g.
///
/// ```
/// digest("abc" + "def") == digest("ab" + "cdef")
/// ```
///
/// # Visibility
/// public
///
/// # Args
/// ```
/// nonce: &String        -> block nonce value  
/// prev_hash: &String    -> hash of the previous block
/// transactions: &String -> JSON serialized String of transactions
/// ```
///
/// # Returns
/// ```
/// String
/// ```
pub fn hash_block(nonce: &String, prev_hash: &String, transactions: &String) -> String {
    let mut values: String = String::from("");
    values.push_str(nonce.as_str());
    values.push_str(DELIMITER);
    values.push_str(prev_hash.as_str());
    values.push_str(DELIMITER);
    values.push_str(transactions.as_str());
    digest(values)
}

/// Creates a SHA256 hash of the components of
/// a transaction. The delimiter aims to prevent an
/// attack where the string components of the
/// hash are combined in a different segments
/// e.g.
///
/// ```
/// digest("abc" + "def") == digest("ab" + "cdef")
/// ```
///
/// # Visibility
/// public
///
/// # Args
/// ```
/// from_address: &String -> the senders private key
/// to_address: &String   -> the recipients public key
/// amount: &String       -> amount being sent
/// ```
///
/// # Returns
/// ```
/// String
/// ```
pub fn hash_transaction(from_address: &String, to_address: &String, amount: &String) -> String {
    let mut values: String = String::from("");
    values.push_str(from_address.as_str());
    values.push_str(DELIMITER);
    values.push_str(to_address.as_str());
    values.push_str(DELIMITER);
    values.push_str(amount.as_str());
    digest(values)
}

/// Creates a Merkle Root by hashing all the transactions
/// that are going to be added to a block
///
/// # Visibility
/// public
///
/// # Args
/// ```
/// path: &str -> path to read transactions from
/// ```
///
/// # Returns
/// ```
/// String
/// ```
pub fn get_merkle_root(path: &str) -> String {
    let mut base_data = FileOps::parse(path);
    let transactions = match base_data["transactions"].as_array_mut() {
        Some(data) => data,
        None => panic!("Transaction data not found, has the file been moved or deleted?"),
    };
    if transactions.len() > 0 {
        let mut hashes = Vec::new();
        for t in transactions {
            hashes.push(Sha256::hash(t["hash"].to_string().as_bytes()));
        }
        let merkle_tree = MerkleTree::<Sha256>::from_leaves(&hashes);
        if let Some(root) = merkle_tree.root_hex() {
            root
        } else {
            String::from("None")
        }
    } else {
        String::from("None")
    }
}

// Testing
#[cfg(test)]
mod test_crypto {
    use super::*;

    use crate::mods::transaction::Transaction;
    use serde_json::to_string;

    #[test]
    fn test_sign_extract_verify() {
        // test hash and private key
        let test_hash = "0".repeat(64);
        let test_private_key =
            String::from("4cae0e746defac95cba2dd5cdb440bb54d102713aeedcad19a483851c0a5ef21");

        // get a signature and signing key by signing the test hash
        let (sig, key) = KeyPair::sign(&test_hash, test_private_key);

        // extract the Signature and SigningKey
        let (signature, signing_key) = KeyPair::extract(sig, key);

        // assert verification of hash signature
        assert!(KeyPair::verify(signature, signing_key, test_hash));
    }

    #[test]
    fn test_hash_block() {
        let transactions: [Transaction; 1] = [Transaction {
            hash: String::from("2".repeat(64)),
            from_address: String::from("2".repeat(130)),
            to_address: String::from("3".repeat(130)),
            amount: 10,
            signature: String::from("4".repeat(128)),
        }];

        let transaction_string = match to_string(&transactions) {
            Ok(val) => val,
            Err(e) => panic!(
                "Error parsing transaction struct to serde_json Value::String: {}",
                e
            ),
        };

        assert_eq!(
            "c47b8a851113808578895f8f783961e38d9a0c481f1f921d90ac9c4905eca797",
            hash_block(
                &String::from("165"),
                &String::from("1").repeat(64),
                &transaction_string,
            )
        );
    }

    #[test]
    fn test_hash_transactions() {
        let transactions: [Transaction; 1] = [Transaction {
            hash: String::from("2".repeat(64)),
            from_address: String::from("2".repeat(130)),
            to_address: String::from("3".repeat(130)),
            amount: 10,
            signature: String::from("4".repeat(128)),
        }];

        assert_eq!(
            "72426b1405464c9f600c859b8c4a9d9097e3a2f60850d8fbeb89c7985507bbc3",
            hash_transaction(
                &transactions[0].from_address,
                &transactions[0].to_address,
                &transactions[0].amount.to_string()
            )
        );
    }
}
