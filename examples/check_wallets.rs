//! Check Wallets Database
//!
//! Usage: cargo run --example check_wallets

fn main() {
    println!("👛 Checking Wallets Database...\n");
    
    let db = match sled::open("data/wallets.db") {
        Ok(db) => db,
        Err(e) => {
            eprintln!("❌ Failed to open database: {}", e);
            return;
        }
    };

    println!("✅ Database opened successfully\n");
    println!("📋 All wallets:\n");

    let mut wallet_count = 0;
    
    for item in db.iter() {
        match item {
            Ok((key, value)) => {
                let key_str = String::from_utf8_lossy(&key);
                wallet_count += 1;
                
                println!("👛 Wallet #{}: {}", wallet_count, key_str);
                
                // Try to parse as JSON
                if let Ok(wallet_json) = serde_json::from_slice::<serde_json::Value>(&value) {
                    println!("   {}", serde_json::to_string_pretty(&wallet_json).unwrap());
                } else {
                    println!("   Raw data: {} bytes", value.len());
                }
                println!();
            }
            Err(e) => eprintln!("❌ Error reading wallet: {}", e),
        }
    }
    
    println!("📊 Summary:");
    println!("   Total wallets: {}", wallet_count);
}
