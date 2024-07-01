# What is Mockchain?

Mockchain is a terminal based, menu-driven program written in Rust that allows you to create a basic blockchain simulation environment.
Some of the core concepts of blockchain technology are covered including:

- Mining blocks to add to the chain
- Generating tokens
- Creating transactions between accounts
- Wallet creation
- Public key cryptography and hashing
- Digital signing of transaction hashes
- Chain verification

# What is the purpose of Mockchain?

It provides a simple options menu with clear outputs to explain what is happening when you perform an action e.g create a new wallet or add a new transaction.
This tool is educational, exposes all data to the user so they can see what is going on, and uses JSON data files for persistance and object representation.

# Running Mockchain

### From a clone

- Install [Rust](https://www.rust-lang.org/tools/install)
- Clone this repo to your local machine and run <code>cargo run</code> in the root directory

### As a binary (TBC)

- Download the appropriate binary file from [/bin](https://github.com/sedexdev/mockchain/tree/main/src/bin) for you platform
- Add the location of the executable to your system path and run inside a terminal

### Install via Cargo (TBC)

- Run <code>cargo install mockchain</code> to install via Rust's package manager

# Usage

- The available integer options throughout the application are:
  - 0 -> Show options menu
  - 1 -> Create a wallet
  - 2 -> Mine a block
  - 3 -> Add a new transaction
  - 4 -> Display the blockchain
  - 5 -> Display pending transactions
  - 6 -> Display wallets
  - 7 -> Display key pairs
  - 8 -> Display signatures
  - 9 -> Re-initialise blockchain
  - 10 -> Verify blockchain
  - 11 -> Exit

As a text-based, menu-driven terminal app there is only so much you can display on the screen. The following directories will be created
under your HOME directory (currently Windows/MacOS/Linux HOME folder locations are supported using the [dirs](https://crates.io/crates/dirs) crate):

- **.mockchain/data/**
  - blockchain.json
  - keypairs.json
  - signing.json
  - transactions.json
  - wallets.json
- **.mockchain/log/**
  - log.txt

It is recommended that you read the output in these files to see what is going when you perform an action (e.g. mine a new block). The
log file has more detailed descriptions of what is happening behind the scenes, while the JSON data files hold information relevant to
the blockchain and the accounts associated with it.

# License

[MIT](https://github.com/sedexdev/mockchain_v2/blob/main/LICENSE)
