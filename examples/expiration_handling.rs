use keyvalue_db::keystoredb::keystore::KVStore;
use std::io;
use std::time::Duration;

fn main() -> io::Result<()> {
    let mut store = KVStore::new("kv_store_expiration.db")?;

    store.set("session".to_string(), "active".to_string(), Some(Duration::from_secs(5)))?;
    println!("Get session: {:?}", store.get("session"));

    std::thread::sleep(Duration::from_secs(6));
    println!("Get expired session: {:?}", store.get("session"));

    Ok(())
}
