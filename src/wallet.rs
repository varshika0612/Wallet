use serde::{Serialize, Deserialize};
use sha2::{Digest, Sha256};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Wallet {
    pub username: String,
    pub password_hash: String,
    pub balance: u64,
    pub transactions: Vec<TransactionRecord>, // Changed from Vec<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionRecord {
    pub description: String,
    pub note: Option<String>, // NEW: User can add notes
    pub timestamp: u64,       // NEW: When transaction happened
}

impl Wallet {
    pub fn filename(username: &str) -> String {
        format!("{}_wallet.json", username)
    }
    
    pub fn create(username: &str, password: &str) -> Self {
        let wallet = Wallet {
            username: username.to_string(),
            password_hash: hash_password(password),
            balance: 100,
            transactions: Vec::new(),
        };
        let filename = Wallet::filename(username);
        let data = serde_json::to_string_pretty(&wallet).unwrap();
        fs::write(filename, data).unwrap();
        wallet
    }
    
    pub fn load(username: &str) -> Option<Self> {
        let filename = Wallet::filename(username);
        let data = fs::read_to_string(filename).ok()?;
        serde_json::from_str(&data).ok()
    }
    
    pub fn save(&self) {
        let filename = Wallet::filename(&self.username);
        let data = serde_json::to_string_pretty(self).unwrap();
        fs::write(filename, data).unwrap();
    }
    
    pub fn verify_password(&self, password: &str) -> bool {
        use argon2::{Argon2, PasswordVerifier};
        use argon2::password_hash::PasswordHash;
        
        // Try Argon2 verification first
        if let Ok(parsed_hash) = PasswordHash::new(&self.password_hash) {
            if Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok() {
                return true;
            }
        }
        
        // Fallback to SHA256 for old passwords
        self.password_hash == hash_password(password)
    }
  
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

pub fn hash_password(password: &str) -> String {
    use argon2::{Argon2, PasswordHasher};
    use argon2::password_hash::{rand_core::OsRng, SaltString};
    
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    
    match argon2.hash_password(password.as_bytes(), &salt) {
        Ok(hash) => hash.to_string(),
        Err(_) => {
            // Fallback to old method if Argon2 fails
            use sha2::{Digest, Sha256};
            let mut hasher = Sha256::new();
            hasher.update(password.as_bytes());
            format!("{:x}", hasher.finalize())
        }
    }
}
