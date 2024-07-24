mod keystoredb;

use keystoredb::keystore::KVStore;
use std::io;

fn main() -> io::Result<()> {
    let store = KVStore::new("kv_store.db")?;

    store.set("name".to_string(), "Alice".to_string())?;
    println!("Get name: {:?}", store.get("name"));

    store.set("name".to_string(), "Bob".to_string())?;
    println!("Get updated name: {:?}", store.get("name"));

    store.delete("name");
    println!("Get deleted name: {:?}", store.get("name"));

    Ok(())
}