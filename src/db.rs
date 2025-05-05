use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

const HEADER_MAGIC: &[u8] = b"JASDB01\n";
const TOC_RESERVED_SIZE: usize = 1024; // Reserve 1KB for TOC block

/// Create a new JasDB file with binary header and reserved TOC section
pub fn create(db_path: &str) -> Result<()> {
    if Path::new(db_path).exists() {
        println!("⚠️ JasDB file already exists: {}", db_path);
        return Ok(());
    }

    let mut file = File::create(db_path)?;
    file.write_all(HEADER_MAGIC)?;

    let empty_toc = vec![0u8; TOC_RESERVED_SIZE];
    file.write_all(&empty_toc)?;

    Ok(())
}

/// Inserts a JSON document into the specified collection in binary format.
/// Automatically updates the in-file TOC.
pub fn insert(db_path: &str, collection: &str, doc: &Value) -> Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(db_path)?;

    // Verify header
    let mut header = [0u8; 8];
    file.read_exact(&mut header)?;
    if &header != HEADER_MAGIC {
        anyhow::bail!("Invalid JasDB header");
    }

    // Read and parse TOC
    let mut toc_buf = vec![0u8; TOC_RESERVED_SIZE];
    file.read_exact(&mut toc_buf)?;
    let mut toc: HashMap<String, u64> = bincode::deserialize(&toc_buf).unwrap_or_default();

    // Seek to end to get offset
    let offset = file.seek(SeekFrom::End(0))?;

    // Serialize and write the document
    let raw = serde_json::to_vec(doc)?;
    let len = raw.len() as u32;
    file.write_all(&len.to_le_bytes())?;
    file.write_all(&raw)?;

    // Update TOC if new collection
    if !toc.contains_key(collection) {
        toc.insert(collection.to_string(), offset);

        // Re-seek to TOC and write back
        file.seek(SeekFrom::Start(HEADER_MAGIC.len() as u64))?;
        let toc_serialized = bincode::serialize(&toc)?;
        let mut padded = toc_serialized;
        padded.resize(TOC_RESERVED_SIZE, 0);
        file.write_all(&padded)?;
    }

    Ok(())
}

/// Placeholder for query logic
pub fn query(_db_path: &str, _collection: &str, _filter: &Value) -> Result<Vec<Value>> {
    anyhow::bail!("Query not yet implemented for binary format")
}

/// Placeholder for update logic
pub fn update(_db_path: &str, _collection: &str, _filter: &Value, _update: &Value) -> Result<usize> {
    anyhow::bail!("Update not yet implemented for binary format")
}

/// Placeholder for delete logic
pub fn delete(_db_path: &str, _collection: &str, _filter: &Value) -> Result<usize> {
    anyhow::bail!("Delete not yet implemented for binary format")
}

/// No-op for now
fn filter_match(_doc: &Value, _filter: &Value) -> bool {
    false
}
