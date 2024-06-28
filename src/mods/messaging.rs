/// MessageLevel enum for output
/// 
/// # Variants
/// ```
/// Success
/// Warning
/// Failure
/// ```
#[derive(Debug)]
enum MessageLevel {
    Success,
    Warning,
    Failure,
}

/// Message struct
/// 
/// # Visibility
/// public
/// 
/// # Fields
/// ```
/// level: MessageLevel
/// message: String
/// ```
pub struct Message {
    level: MessageLevel,
    message: String,
}

impl Message {
    
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
    pub fn display_msg(level: MessageLevel, message: String) {
        match level {
            MessageLevel::Success => println!("[+] {}", message),
            MessageLevel::Warning => println!("[!] {}", message),
            MessageLevel::Failure => println!("[-] {}", message),
        }
    }
}
