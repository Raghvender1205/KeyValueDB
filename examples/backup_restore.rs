use keyvalue_db::keystoredb::keystore::KVStore;

fn main() -> std::io::Result<()> {
    let mut store = KVStore::new("kv_store.db")?;

    // Set a key-value pair
    store.set("name".to_string(), "Alice".to_string(), None)?;
    println!("Get name: {:?}", store.get("name"));

    // Create a backup
    store.backup("kv_store_backup.db")?;
    println!("Backup created.");

    // Update the key-value pair
    store.set("name".to_string(), "Bob".to_string(), None)?;
    println!("Get updated name: {:?}", store.get("name"));

    // Restore from backup
    store.restore("kv_store_backup.db")?;
    println!("Restored from backup.");

    // Verify restoration
    println!("Get name after restore: {:?}", store.get("name"));

    Ok(())
}
