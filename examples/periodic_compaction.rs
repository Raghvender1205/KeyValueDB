use keyvalue_db::keystoredb::keystore::KVStore;
use std::io;
use std::time::Duration;

fn main() -> io::Result<()> {
    let mut store = KVStore::new("kv_store_compaction.db")?;

    store.set("temp".to_string(), "to be expired".to_string(), Some(Duration::from_secs(3)))?;
    println!("Get temp: {:?}", store.get("temp"));

    std::thread::sleep(Duration::from_secs(5));
    println!("Get after sleep: {:?}", store.get("temp"));

    println!("List all keys after sleep: {:?}", store.list_keys());

    Ok(())
}
