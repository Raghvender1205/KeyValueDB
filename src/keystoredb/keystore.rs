use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter};
use std::time::{Duration, SystemTime};

#[derive(Serialize, Deserialize)]
struct Entry {
    value: String,
    expires_at: Option<SystemTime>,
}

pub struct KVStore {
    map: HashMap<String, Entry>,
    file_path: String,
    unsaved_changes: bool,
}

impl KVStore {
    pub fn new(file_path: &str) -> io::Result<Self> {
        let file = OpenOptions::new().read(true).write(true).create(true).open(file_path)?;
        let reader = BufReader::new(file);
        let map: HashMap<String, Entry> = match serde_json::from_reader(reader) {
            Ok(map) => map,
            Err(_) => HashMap::new(),
        };
        Ok(KVStore {
            map,
            file_path: file_path.to_string(),
            unsaved_changes: false,
        })
    }

    pub fn set(&mut self, key: String, value: String, ttl: Option<Duration>) -> io::Result<()> {
        let expires_at = ttl.map(|d| SystemTime::now() + d);
        let entry = Entry { value, expires_at };
        self.map.insert(key, entry);
        self.unsaved_changes = true;
        self.save_if_needed()
    }

    pub fn get(&self, key: &str) -> Option<String> {
        if let Some(entry) = self.map.get(key) {
            if let Some(expiry) = entry.expires_at {
                if expiry < SystemTime::now() {
                    return None;
                }
            }
            return Some(entry.value.clone());
        }
        None
    }

    pub fn delete(&mut self, key: &str) -> Option<String> {
        let value = self.map.remove(key)?;
        self.unsaved_changes = true;
        self.save_if_needed().ok()?;
        Some(value.value)
    }

    pub fn list_keys(&self) -> Vec<String> {
        self.map.keys().cloned().collect()
    }

    pub fn write_snapshot(&self) -> io::Result<()> {
        let snapshot_file = format!("{}_snapshot.json", self.file_path);
        let file = File::create(&snapshot_file)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &self.map)?;
        println!("Snapshot written to {}", snapshot_file);
        Ok(())
    }

    fn save(&self) -> io::Result<()> {
        let file = File::create(&self.file_path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &self.map)?;
        Ok(())
    }

    fn save_if_needed(&mut self) -> io::Result<()> {
        if self.unsaved_changes {
            self.save()?;
            self.unsaved_changes = false;
        }
        Ok(())
    }
}