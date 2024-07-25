use keyvalue_db::keystoredb::keystore::KVStore; 
use std::io;

fn main() -> io::Result<()> {
    let mut store = KVStore::new("kv_store_snapshot.db")?;

    store.set("key1".to_string(), "value1".to_string(), None)?;
    store.set("key2".to_string(), "value2".to_string(), None)?;

    store.write_snapshot()?;
    println!("Snapshot written.");

    Ok(())
}