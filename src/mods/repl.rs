// std
use std::str::FromStr;
use std::fmt::{Debug, Display};

// 3rd party crates
use text_io::try_read;

/// Basis structure for displaying and reading
/// information during application runtime
/// 
/// # Visibility
/// public
/// 
/// # Fields
/// ```
/// option: i32 - users menu choice
/// ```
pub struct Repl {
    option: i32
}

impl Repl {
    
    /// Prints the options availble in the simulation
    /// 
    /// # Visibility
    /// public
    /// 
    /// # Args
    /// None
    /// 
    /// # Returns
    /// Nothing
    pub fn print_intro() {
        println!("\n\t\t\t-- WELCOME TO MOCKCHAIN --\n");
        println!("This program is a text based blockchain simulation designed to aid understanding");
        println!("of how blockchains work.\n");
    }
    
    /// Prints the options available in the simulation
    /// 
    /// # Visibility
    /// public
    /// 
    /// # Args
    /// None
    /// 
    /// # Returns
    /// Nothing
    pub fn print_options() {
        println!("OPTIONS\n");
        println!("1. Create a wallet");
        println!("2. Mine a block");
        println!("3. Add a new transaction");
        println!("4. Display the blockchain");
        println!("5. Display pending transactions");
        println!("6. Display wallets");
        println!("7. Display key pairs");
        println!("8. Display signatures");
        println!("9. Re-initialize blockchain");
        println!("10. Exit\n");
    }
    
    /// Gets user input from the console and performs
    /// error checking. Returns the value if checks pass 
    /// 
    /// # Args
    /// None
    /// 
    /// # Returns
    /// ```
    /// Option<T>
    /// ```
    /// 
    /// # Example
    /// ```
    /// let _input: i32 = match get_input() {
    ///     Some(val) => val,
    ///     None => -1,
    /// };
    /// ```
    pub fn get_input<T: Display + FromStr + Debug>() -> Option<T> where <T as FromStr>::Err: Debug {
        let i: Option<T> = match try_read!() {
            Ok(val) => Some(val),
            Err(_) => None,
        };
    
        if i.is_none() {
            None
        } else {
            i
        }
    }
}
