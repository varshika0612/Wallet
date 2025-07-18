mod wallet;
mod blockchain;

use wallet::{Wallet, hash_password, TransactionRecord};
use blockchain::BlockChain;

use clap::{Parser, Subcommand};
use std::fs;
use std::io::{self, Write};
use chrono::Utc;
use rpassword;

#[derive(Parser)]
#[command(name = "wallet-cli")]
#[command(about = "A wallet CLI with login and wallet actions")]
struct Cli {
    #[command(subcommand)]
    command: TopLevelCommand,
}

#[derive(Subcommand)]
enum TopLevelCommand {
    #[command(subcommand)]
    Account(AccountCommand),
    #[command(subcommand)]
    Wallet(WalletCommand),
}

#[derive(Subcommand)]
enum AccountCommand {
    /// Create a new account
    CreateAccount,
    /// Login to an existing account
    Login { username: String },
    /// Logout current session
    Logout,
}

#[derive(Subcommand)]
enum WalletCommand {
    /// Send coins
    Send { to: String, amount: u64 },
    /// Receive coins
    Receive { from: String, amount: u64 },
    /// Check balance
    Balance,
    /// Show transaction history
    History,
    /// Generate QR code for wallet address
    QrCode,
}

fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn set_logged_in_user(username: &str) {
    fs::write("session.txt", username).unwrap();
}

fn get_logged_in_user() -> Option<String> {
    fs::read_to_string("session.txt").ok().map(|s| s.trim().to_string())
}

fn logout_user() {
    let _ = fs::remove_file("session.txt");
}

fn update_blockchain(blockchain: &BlockChain, data: &str) {
    let mut blockchain = blockchain.clone();
    blockchain.add_block(data);
    if !blockchain.is_valid() {
        println!("Blockchain is invalid after adding new block.");
        return;
    }
    println!("New block added to the blockchain with data: {}", data);
    blockchain.save();
}

fn main() {
    let cli = Cli::parse();
    let mut blockchain = BlockChain::load().unwrap_or_else(BlockChain::new);

    match &cli.command {
        TopLevelCommand::Account(account_cmd) => match account_cmd {
            AccountCommand::CreateAccount => {
                let username = prompt("Enter username for new account: ");
                if Wallet::load(&username).is_some() {
                    println!("Account '{}' already exists.", username);
                } else {
                    let password = rpassword::prompt_password("Enter password: ").unwrap();
                    let wallet = Wallet::create(&username, &password);
                    set_logged_in_user(&username);
                    println!("Account created for '{}' with balance {}. You are now logged in.", wallet.username, wallet.balance);
                }
            }
            AccountCommand::Login { username } => {
                if let Some(wallet) = Wallet::load(username) {
                    let password = rpassword::prompt_password("Enter password: ").unwrap();
                    if wallet.verify_password(&password) {
                        set_logged_in_user(username);
                        println!("Logged in as '{}'.", wallet.username);
                    } else {
                        println!("Incorrect password.");
                    }
                } else {
                    println!("No account found for username '{}'.", username);
                }
            }
            AccountCommand::Logout => {
                logout_user();
                println!("Logged out.");
            }
        },
        TopLevelCommand::Wallet(wallet_cmd) => {
            let username = match get_logged_in_user() {
                Some(u) => u,
                None => {
                    println!("You must be logged in to use wallet commands.");
                    return;
                }
            };

            match wallet_cmd {
                WalletCommand::Send { to, amount } => {
                    let mut sender_wallet = match Wallet::load(&username) {
                        Some(w) => w,
                        None => {
                            println!("Sender account not found.");
                            return;
                        }
                    };

                    if sender_wallet.balance < *amount {
                        println!("Insufficient balance.");
                        return;
                    }

                    let mut receiver_wallet = match Wallet::load(to) {
                        Some(w) => w,
                        None => {
                            println!("Receiver account '{}' not found.", to);
                            return;
                        }
                    };

                    let note = prompt("Add a note (optional): ");
                    let note = if note.is_empty() { None } else { Some(note) };

                    sender_wallet.balance -= amount;
                    receiver_wallet.balance += amount;

                    let timestamp = Utc::now().timestamp() as u64;

                    sender_wallet.transactions.push(TransactionRecord {
                        description: format!("Sent {} coins to {}", amount, to),
                        note: note.clone(),
                        timestamp,
                    });

                    receiver_wallet.transactions.push(TransactionRecord {
                        description: format!("Received {} coins from {}", amount, username),
                        note,
                        timestamp,
                    });

                    sender_wallet.save();
                    receiver_wallet.save();

                    update_blockchain(&blockchain, &format!("Sent {} coins from '{}' to '{}'", amount, username, to));
                    println!("Sent {} coins from '{}' to '{}'.", amount, username, to);
                    println!("Your new balance: {}", sender_wallet.balance);
                }

                WalletCommand::Receive { from, amount } => {
                    let mut receiver_wallet = match Wallet::load(&username) {
                        Some(w) => w,
                        None => {
                            println!("Receiver account not found.");
                            return;
                        }
                    };

                    let mut sender_wallet = match Wallet::load(from) {
                        Some(w) => w,
                        None => {
                            println!("Sender account '{}' not found.", from);
                            return;
                        }
                    };

                    if sender_wallet.balance < *amount {
                        println!("Sender '{}' has insufficient balance.", from);
                        return;
                    }

                    sender_wallet.balance -= amount;
                    receiver_wallet.balance += amount;

                    sender_wallet.save();
                    receiver_wallet.save();

                    let timestamp = Utc::now().timestamp() as u64;

                    sender_wallet.transactions.push(TransactionRecord {
                        description: format!("Sent {} coins to {}", amount, username),
                        note: None,
                        timestamp,
                    });

                    receiver_wallet.transactions.push(TransactionRecord {
                        description: format!("Received {} coins from {}", amount, from),
                        note: None,
                        timestamp,
                    });

                    update_blockchain(&blockchain, &format!("Sent {} coins from '{}' to '{}'", amount, from, username));
                    println!("Received {} coins from '{}' to '{}'.", amount, from, username);
                    println!("Your new balance: {}", receiver_wallet.balance);
                }

                WalletCommand::Balance => {
                    if let Some(wallet) = Wallet::load(&username) {
                        println!("'{}' balance: {}", wallet.username, wallet.balance);
                    } else {
                        println!("No account found for username '{}'.", username);
                    }
                }

                WalletCommand::History => {
                    if let Some(wallet) = Wallet::load(&username) {
                        if wallet.transactions.is_empty() {
                            println!("No transaction history found for '{}'.", wallet.username);
                        } else {
                            println!("Transaction history for '{}':", wallet.username);
                            for transaction in &wallet.transactions {
                                let note_part = transaction.note.as_ref()
                                    .map(|n| format!(" (Note: {})", n))
                                    .unwrap_or_default();
                                println!("- {}{}", transaction.description, note_part);
                            }
                        }
                    } else {
                        println!("No account found for username '{}'.", username);
                    }
                }

                WalletCommand::QrCode => {
                    if let Some(wallet) = Wallet::load(&username) {
                        match wallet.generate_qr() {
                            Ok(()) => {
                                println!("QR code generated successfully!");
                            }
                            Err(e) => {
                                println!("Error generating QR code: {}", e);
                            }
                        }
                    } else {
                        println!("No account found for username '{}'.", username);
                    }
                }
            }
        }
    }
}

// Add this to your wallet.rs file:
impl Wallet {
    // ... existing functions ...
    pub fn get_address(&self) -> String {
        format!("wallet_{}", self.username)
    }
    
    pub fn generate_qr(&self) -> Result<(), Box<dyn std::error::Error>> {
        use qrcode::QrCode;
        use image::Luma;
        
        let address = self.get_address();
        let code = QrCode::new(&address)?;
        let image = code.render::<Luma<u8>>().build();
        
        let filename = format!("{}_address_qr.png", self.username);
        image.save(&filename)?;
        
        println!("QR code saved as: {}", filename);
        println!("Your wallet address: {}", address);
        Ok(())
    }
}
