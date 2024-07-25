use keyvalue_db::keystoredb::keystore::KVStore;
use std::io;
use std::time::Duration;

fn main() -> io::Result<()> {
    let mut store = KVStore::new("kv_store.db")?;

    store.set("name".to_string(), "Alice".to_string(), None)?;
    println!("Get name: {:?}", store.get("name"));

    store.set("name".to_string(), "Bob".to_string(), None)?;
    println!("Get updated name: {:?}", store.get("name"));

    store.set("temp".to_string(), "Temporary".to_string(), Some(Duration::from_secs(10)))?;
    println!("Get temp: {:?}", store.get("temp"));
    std::thread::sleep(Duration::from_secs(11));
    println!("Get expired temp: {:?}", store.get("temp"));

    store.delete("name");
    println!("Get deleted name: {:?}", store.get("name"));

    println!("List all keys: {:?}", store.list_keys());

    Ok(())
}
