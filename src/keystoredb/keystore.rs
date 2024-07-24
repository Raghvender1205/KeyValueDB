use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write, Seek, SeekFrom};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::keystoredb::kvpair::KVPair;


pub struct KVStore {
    map: Arc<Mutex<HashMap<String, KVPair>>>,
    file: Arc<Mutex<File>>,
    stop_compaction: Arc<AtomicBool>,
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
                if kv.expires_at.is_none() || kv.expires_at.unwrap() > current_timestamp() {
                    map.insert(kv.key.clone(), kv);
                }
            }
        }

        let store = KVStore {
            map: Arc::new(Mutex::new(map)),
            file: Arc::new(Mutex::new(file)),
            stop_compaction: Arc::new(AtomicBool::new(false)),
        };

        let store_clone = store.clone();
        thread::spawn(move || {
            store_clone.compact_file_periodically();
        });

        Ok(store)
    }       

    fn clone(&self) -> Self {
        KVStore {
            map: Arc::clone(&self.map),
            file: Arc::clone(&self.file),
            stop_compaction: Arc::clone(&self.stop_compaction),
        }
    }

    pub fn set(&self, key: String, value: String, ttl: Option<Duration>) -> io::Result<()> {
        let expires_at = ttl.map(|duration| current_timestamp() + duration.as_secs());
        let kv = KVPair { key: key.clone(), value: value.clone(), expires_at };
        let mut map = self.map.lock().unwrap();
        map.insert(key.clone(), kv.clone()); // Clone kv before inserting into the map

        let mut file = self.file.lock().unwrap();
        writeln!(file, "{}", serde_json::to_string(&kv)?)?;
        let _ = file.sync_all();
        Ok(())
    }


    pub fn get(&self, key: &str) -> Option<String> {
        let mut map = self.map.lock().unwrap();
        if let Some(kv) = map.get(key) {
            if kv.expires_at.is_none() || kv.expires_at.unwrap() > current_timestamp() {
                return Some(kv.value.clone());
            } else {
                map.remove(key);
            }
        }
        None
    }


    pub fn delete(&self, key: &str) -> Option<String> {
        let mut map = self.map.lock().unwrap();
        if let Some(kv) = map.remove(key) {
            self.write_snapshot().unwrap();
            return Some(kv.value);
        }
        None
    }


    pub fn list_keys(&self) -> Vec<String> {
        let map = self.map.lock().unwrap();
        map.keys().cloned().collect()
    }

    fn write_snapshot(&self) -> io::Result<()> {
        let map = self.map.lock().unwrap();
        let mut file = self.file.lock().unwrap();
        file.set_len(0)?;
        file.seek(SeekFrom::Start(0))?;

        for kv in map.values() {
            writeln!(file, "{}", serde_json::to_string(&kv)?)?;
        }
        file.sync_all()
    }

    fn compact_file_periodically(&self) {
        while !self.stop_compaction.load(Ordering::SeqCst) {
            thread::sleep(Duration::from_secs(60));
            self.compact_file().unwrap();
        }
    }

    fn compact_file(&self) -> io::Result<()> {
        let mut map = self.map.lock().unwrap();

        let new_map: HashMap<String, KVPair> = map.clone().into_iter()
            .filter(|(_, kv)| kv.expires_at.is_none() || kv.expires_at.unwrap() > current_timestamp())
            .collect();

        *map = new_map;
        self.write_snapshot()?;
        Ok(())
    }
}


impl Drop for KVStore {
    fn drop(&mut self) {
        self.stop_compaction.store(true, Ordering::SeqCst);
    }
}


fn current_timestamp() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}