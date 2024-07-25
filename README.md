# KeyValueDB

A Key-Value store based `database` implemented in Rust.

## Features
This `database` currently have the following features
1. Data Persistence
2. Expiration of entries
3. Snapshot writing
4. Periodic compaction

------

### Key Value Store Initialization
- `KVStore` struct initializes with a file for persistence storage.
- It loads existing `key-value` pairs from the file into a in-memory `HashMap` upon creation.

### Adding Key-Value Pairs
- The `set` method allows adding key-value pairs to the `store`
- Optionally, a `Time-to-Live (TTL)` duration can be specified to set an expiration time for each entry.
- Each new entry is also written to the persistent file.

### Retrieving Entries
- The `get` method retrieves the value associated with a given key.
- If the entry has expired, it removes the entry and returns `None`.

### Deleting Entries
- The `delete` method removes the entry associated with a given `key` and writes the updated state of the file.

###  Listing All Keys
- The `list_keys` method returns a list of all keys currently in the store.

### Periodic File Compaction
- A background thread periodically compacts the file by removing `expired` entries.
- The `compact_file_periodically` method handles this process.

### Writing snapshots
- The `write_snapshot` method writes the current state of the in-memory `HashMap` to the file, overwriting its contents.

### Handling Entry Expiration
- KeyStoreDB checks and handles entry expiration during `retrieval`, `deletion` and `compaction`.

### Graceful Shutdown
- The `Drop` trait implementation ensures that the compaction thread stops gracefully when the `KVStore` instance is dropped.

### Concurrency
- Thread safe access.
- `KeyStoreDB` uses `Arc` and `Mutex` to ensure thread-safe access to the in-memory data structure and the file, allowing multiple threads to interact with the database concurrently.

### Batched Updates
- Efficient Disk I/O
- It performs disk writes in a batched manner, reducing I/O overhead and making `set` and `delete` operation more efficient.  


## TODO
1. Transactional Support
2. Backup and Restore
3. Configuration Management
4. Concurrency Control
5. Compression
6. Data Expiry Notifications
7. Custom Serialization
8. Statistics and Metrics
9. Versioning 
10. Query Support
11. Replication
12. Encryption
13. Sharding
14. Caching
15. Import/Export
16. Custom Key Comparisons
17. Data Integrity Checks