use sled::Db;

fn main() {
    println!("📊 Checking FODI Bank Ledger Database...\n");
    
    let db = match sled::open("data/fodi_ledger.db") {
        Ok(db) => db,
        Err(e) => {
            eprintln!("❌ Failed to open database: {}", e);
            return;
        }
    };

    println!("✅ Database opened successfully\n");
    println!("📋 All keys in database:\n");

    let mut balance_count = 0;
    let mut tx_count = 0;
    
    for item in db.iter() {
        match item {
            Ok((key, value)) => {
                let key_str = String::from_utf8_lossy(&key);
                
                if key_str.starts_with("balance:") {
                    balance_count += 1;
                    let user_id = key_str.strip_prefix("balance:").unwrap();
                    
                    // Try to parse balance
                    if let Ok(balance_json) = serde_json::from_slice::<serde_json::Value>(&value) {
                        println!("💰 User: {}", user_id);
                        println!("   Balance: {}", serde_json::to_string_pretty(&balance_json).unwrap());
                        println!();
                    }
                } else if key_str.starts_with("tx:") {
                    tx_count += 1;
                } else {
                    println!("🔑 Key: {}", key_str);
                }
            }
            Err(e) => eprintln!("❌ Error reading key: {}", e),
        }
    }
    
    println!("\n📊 Summary:");
    println!("   Total balances: {}", balance_count);
    println!("   Total transactions: {}", tx_count);
}
