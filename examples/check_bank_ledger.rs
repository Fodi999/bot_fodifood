//! Check FODI Bank Ledger Database
//!
//! Usage: cargo run --example check_bank_ledger

use sled::Db;
use serde_json::Value;

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
    println!("📋 All records in database:\n");

    let mut balance_count = 0;
    let mut tx_count = 0;
    let mut other_count = 0;
    
    for item in db.iter() {
        match item {
            Ok((key, value)) => {
                let key_str = String::from_utf8_lossy(&key);
                
                if key_str.starts_with("balance:") {
                    balance_count += 1;
                    let user_id = key_str.strip_prefix("balance:").unwrap();
                    
                    // Try to parse balance
                    if let Ok(balance_json) = serde_json::from_slice::<Value>(&value) {
                        println!("💰 User Balance: {}", user_id);
                        println!("   {}", serde_json::to_string_pretty(&balance_json).unwrap());
                        println!();
                    } else {
                        println!("💰 User Balance: {} (raw data)", user_id);
                    }
                } else if key_str.starts_with("tx:") {
                    tx_count += 1;
                    let tx_id = key_str.strip_prefix("tx:").unwrap();
                    
                    if let Ok(tx_json) = serde_json::from_slice::<Value>(&value) {
                        println!("📝 Transaction: {}", tx_id);
                        println!("   {}", serde_json::to_string_pretty(&tx_json).unwrap());
                        println!();
                    }
                } else {
                    other_count += 1;
                    println!("🔑 Other Key: {}", key_str);
                    println!("   Value length: {} bytes", value.len());
                    println!();
                }
            }
            Err(e) => eprintln!("❌ Error reading key: {}", e),
        }
    }
    
    println!("\n📊 Summary:");
    println!("   💰 Total user balances: {}", balance_count);
    println!("   📝 Total transactions: {}", tx_count);
    println!("   🔑 Other keys: {}", other_count);
    println!("   📦 Total records: {}", balance_count + tx_count + other_count);
}
