Wallet CLI Application
A simple, secure, and extensible command-line cryptocurrency wallet built in Rust. Implements user account management, on-disk JSON wallet storage, a basic proof-of-work blockchain, and secure password hashing with Argon2.

Features
Account creation, login, and logout

Balance inquiry, send, and receive commands

Immutable blockchain ledger with SHA-256 proof-of-work

Secure password storage using Argon2id with random salts

JSON-based on-disk persistence per user

Transaction history tracking

Plug-and-play extensibility for:

QR code generation

BIP-39 mnemonics

Transaction notes and more

Getting Started
Prerequisites
Rust (stable toolchain)

cargo package manager

Git

Clone and Build
bash
git clone https://github.com/varshika0612/Wallet.git
cd Wallet
cargo build --release
A release binary will be available at target/release/wallet.

Usage
Invoke the wallet CLI with a subcommand and appropriate flags:

bash
./target/release/wallet [OPTIONS]
Common Commands
Command	Description
createaccount	Create a new wallet user
login	Log in to an existing wallet user
logout	Log out of the current session
balance	Show current balance
send	Send N coins to recipient R
receive	Receive N coins from sender S
history	Display transaction history
Example Usage
bash
# Create and log in
./wallet createaccount --username alice
./wallet login --username alice

# Check balance and send coins
./wallet balance
./wallet send --to bob --amount 50

# View history and log out
./wallet history
./wallet logout
Configuration & Extensibility
QR Code Generation (Optional)
Enable QR code support by adding dependencies in Cargo.toml:

text
qrcode = "0.13"
image = "0.25"
Then add the QrCode subcommand to main.rs and call generate_qr() on your wallet instance.

Transaction Notes (Optional)
Extend the TransactionRecord struct in wallet.rs with a note field. Update the send command to prompt for an optional note.

Mnemonic Backups (BIP-39) (Optional)
Integrate the bip39 crate for 12-word mnemonic generation and storage to support wallet recovery.

Security
Argon2id password hashing with per-user random salts

SHA-256 hashing for block integrity

Proof-of-work consensus to prevent tampering

Offline JSON storage for data isolation

For additional hardening, consider adding:

CI-based cargo audit

Clippy with -D warnings

Unit/integration tests

Contributing
Contributions are welcome! Please fork the repository and open a pull request with:

Descriptive branch name (e.g., feature/qr-code)

Unit tests covering new functionality

Updated documentation in README.md

License
This project is dual-licensed under MIT OR Apache-2.0. See the LICENSE file for details.
