use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct KVPair {
    key: String,
    value: String,
}

pub struct KVStore {
    map: Arc<Mutex<HashMap<String, String>>>,
    file: Arc<Mutex<File>>,
}

impl KVStore {
    pub fn new(filename: &str) -> io::Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(filename)?;
        
        let mut map = HashMap::new();
        let reader = BufReader::new(&file);

        for line in reader.lines() {
            let line = line?;
            if let Ok(kv) = serde_json::from_str::<KVPair>(&line) {
                map.insert(kv.key, kv.value);
            }
        }

        Ok(KVStore {
            map: Arc::new(Mutex::new(map)),
            file: Arc::new(Mutex::new(file)),
        })
    }       

    pub fn set(&self, key: String, value: String) -> io::Result<()> {
        let mut map = self.map.lock().unwrap();
        map.insert(key.clone(), value.clone());

        let kv = KVPair { key, value };
        let mut file = self.file.lock().unwrap();
        writeln!(file, "{}", serde_json::to_string(&kv)?)?;
        file.sync_all();
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let map = self.map.lock().unwrap();
        map.get(key).cloned()
    }

    pub fn delete(&self, key: &str) -> Option<String> {
        let mut map = self.map.lock().unwrap();
        map.remove(key)
    }
    
}


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