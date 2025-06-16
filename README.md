# Wallet CLI

A simple command-line wallet application in Rust with user registration, login, wallet management, and a basic blockchain for transaction history.

## Features

- **User Registration & Login:** Secure account creation and authentication with password hashing (Argon2).
- **Wallet Management:** Check balance, send and receive coins, and view transaction history.
- **Blockchain:** All transactions are recorded on a simple blockchain for transparency.
- **Session Management:** Users stay logged in until they logout.
- **Secure Password Input:** Passwords are never shown on the screen.

## Usage

### Build and Run

```sh
cargo build
cargo run -- [COMMAND]
```

### Commands

#### Account Management

- `createaccount`  
  Register a new user account.

- `login`  
  Log in to an existing account.  
  _(You will be prompted to enter your username and password interactively.)_

- `logout`  
  Log out of the current session.

#### Wallet Operations

- `send --to <username> --amount <amount>`  
  Send coins to another user.

- `receive --from <username> --amount <amount>`  
  Receive coins from another user.

- `balance`  
  Show your current wallet balance.

- `history`  
  Show your transaction history.

## File Structure

- `src/main.rs` — CLI logic and command handling
- `src/wallet.rs` — Wallet struct, password hashing, and wallet file management
- `src/blockchain.rs` — Blockchain struct and logic
- `src/blockchain/block.rs` — Block struct and logic

## Data Storage

- User credentials are stored in `users.txt`
- Wallets are stored as JSON files per user
- Blockchain is stored in `blockchain.json`
- Session info is stored in `session.txt`

## Security

- Passwords are hashed using Argon2 and never stored in plain text.
- Password input is hidden using the `rpassword` crate.

## Requirements

- Rust (edition 2021)
- [See `Cargo.toml` for dependencies](Cargo.toml)

## License

MIT

---

*This project is for educational purposes and
