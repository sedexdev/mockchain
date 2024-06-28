// 3rd party crates
use serde::Serialize;

/// Define a Signing object
///
/// # Visibility
/// public
///
/// # Fields
/// ```
/// name: String        -> name of signing account
/// hash: String        -> transaction hash
/// signing_key: String -> signing key (encoded as hex byte string)
/// signature: String   -> signature (encoded as hex byte string)
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
    pub signature: String,
}
