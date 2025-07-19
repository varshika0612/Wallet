# 🪙 Wallet CLI Application

A simple, secure, and extensible command-line cryptocurrency wallet built in Rust.  
Implements user account management, secure on-disk storage, a basic blockchain with proof-of-work, and password hashing using Argon2.

---

## 🚀 Features

- ✅ **Account creation, login, and logout**
- 💰 **Balance inquiry, send, and receive commands**
- 🔒 **Secure password storage** using **Argon2id** with random salts
- ⛓️ **Immutable blockchain ledger** with **SHA-256 proof-of-work**
- 📄 **Transaction history tracking**
- 💾 **JSON-based on-disk persistence per user**
- 🧩 **Plug-and-play extensibility** for:
  - QR Code generation
  - BIP-39 mnemonic phrase backups
  - Transaction notes

---

## 📦 Getting Started

### 🔧 Prerequisites

- [Rust (stable)](https://rust-lang.org/)
- Cargo package manager (comes with Rust)
- Git

### 🛠️ Clone and Build

```bash
git clone https://github.com/varshika0612/Wallet.git
cd Wallet
cargo build --release
```

> The release binary will be available at `target/release/wallet`.

---

## 🧪 Usage

Invoke the wallet CLI with subcommands:

```bash
./target/release/wallet [COMMAND] [OPTIONS]
```

### 📋 Common Commands

| Command        | Description                         |
|----------------|-------------------------------------|
| `createaccount`| Create a new wallet user            |
| `login`        | Log in to an existing user account  |
| `logout`       | Log out of the current session      |
| `balance`      | Show the current wallet balance     |
| `send`         | Send N coins to recipient R         |
| `receive`      | Receive N coins from sender S       |
| `history`      | Display the user's transaction log  |

---

### 🧾 Example Usage

```bash
# Create a new account and log in
./wallet createaccount --username alice
./wallet login --username alice

# Check balance and send coins
./wallet balance
./wallet send --to bob --amount 50

# View history and logout
./wallet history
./wallet logout
```

---

## ⚙️ Configuration & Extensibility

### 📷 QR Code Support (Optional)

Add the following to your `Cargo.toml`:

```toml
qrcode = "0.13"
image = "0.25"
```

Add a `QrCode` subcommand and call `generate_qr()` on your wallet instance for address sharing or payment requests.

---

### 📝 Transaction Notes (Optional)

Add a `note` field to the `TransactionRecord` struct in `wallet.rs`, and modify the `send` command to accept an optional note.

---

### 🔐 Mnemonic Backups (Optional)

Use the [`bip39`](https://crates.io/crates/bip39) crate for 12-word mnemonic generation for wallet recovery:

```toml
bip39 = "1.2"
```

Generate mnemonic phrases and derive private keys or recovery options.

---

## 🛡️ Security

- Argon2id for secure password hashing (with per-user random salts)
- SHA-256 hashing for blockchain integrity
- Proof-of-work to prevent tampering
- Offline JSON storage for isolation

### 🧪 Recommended Additions

- Continuous Integration: `cargo audit` for dependency vulnerability checks
- `clippy` with `-D warnings` to enforce code linting
- Unit and integration tests for core features

---

## 🤝 Contributing

Contributions are welcome! Please:

1. Fork the repo
2. Create a descriptive branch (e.g., `feature/qr-code`)
3. Implement and test your changes
4. Submit a pull request with:
   - Proper description
   - Tests (if applicable)
   - README updates (if applicable)

---


