use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "wallet-cli")]
#[command(about = "A wallet CLI with login and wallet actions")]
struct Cli {
    #[command(subcommand)]
    command: TopLevelCommand,
}

#[derive(Subcommand)]
enum TopLevelCommand {
    /// Account management: login or create
    Account(AccountCommand),
    /// Wallet actions: send, receive
    Wallet(WalletCommand),
}

#[derive(Subcommand)]
enum AccountCommand {
    /// Create a new account
    CreateAccount,
    /// Login to an existing account
    Login { username: String },
}

#[derive(Subcommand)]
enum WalletCommand {
    /// Send coins
    Send { to: String, amount: u64 },
    /// Receive coins
    Receive { from: String, amount: u64 },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        TopLevelCommand::Account(account_cmd) => match account_cmd {
            AccountCommand::CreateAccount => {
                println!("Creating account...");
                // Your code here
            }
            AccountCommand::Login { username } => {
                println!("Logging in as {}", username);
                // Your code here
            }
        },
        TopLevelCommand::Wallet(wallet_cmd) => match wallet_cmd {
            WalletCommand::Send { to, amount } => {
                println!("Sending {} coins to {}", amount, to);
                // Your code here
            }
            WalletCommand::Receive { from, amount } => {
                println!("Receiving {} coins from {}", amount, from);
                // Your code here
            }
        },
    }
}
