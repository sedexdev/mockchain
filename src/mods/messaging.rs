/// Message enum for output
///
/// # Visibility
/// public
///
/// # Variants
/// ```
/// Success
/// Warning
/// Failure
/// ```
#[derive(Debug)]
pub enum Message {
    Success(String, Option<Vec<String>>),
    Warning(String, Option<Vec<String>>),
    Failure(String, Option<Vec<String>>),
}

/// Displays a message
///
/// # Visibility
/// public
///
/// # Args
/// ```
/// level: MessageLevel -> level
/// message: String       -> message to display
/// ```
///
/// # Returns
/// Nothing
pub fn display_msg(level: Message) {
    match level {
        Message::Success(m, v) => {
            match v {
                Some(val) => {
                    let expanded = replace(m, val);
                    println!("[+] {}", expanded);
                }
                None => println!("[+] {}", m),
            };
        }
        Message::Warning(m, v) => {
            match v {
                Some(val) => {
                    let expanded = replace(m, val);
                    print!("[!] {}", expanded);
                }
                None => println!("[!] {}", m),
            };
        }
        Message::Failure(m, v) => {
            match v {
                Some(val) => {
                    let expanded = replace(m, val);
                    println!("[-] {}", expanded);
                }
                None => println!("[-] {}", m),
            };
        }
    }
}

/// Replaces {} placeholder value with vector element
///
/// # Visibility
/// private
///
/// # Args
/// ```
/// message: String  -> message to update
/// vec: Vec<String> -> values to add to String
/// ```
///
/// # Returns
/// ```
/// String
/// ```
fn replace(message: String, vec: Vec<String>) -> String {
    let mut result = String::from(message);
    for value in &vec {
        if let Some(pos) = result.find("{}") {
            result.replace_range(pos..pos + 2, value);
        }
    }
    result
}
