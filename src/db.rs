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

/// Query documents from a collection based on TOC and basic filter
pub fn query(db_path: &str, collection: &str, filter: &Value) -> Result<Vec<Value>> {
    let mut file = File::open(db_path)?;

    // Check header
    let mut header = [0u8; 8];
    file.read_exact(&mut header)?;
    if &header != HEADER_MAGIC {
        anyhow::bail!("Invalid JasDB header");
    }

    // Load TOC
    let mut toc_buf = vec![0u8; TOC_RESERVED_SIZE];
    file.read_exact(&mut toc_buf)?;
    let toc: HashMap<String, u64> = bincode::deserialize(&toc_buf).unwrap_or_default();

    let start = match toc.get(collection) {
        Some(offset) => *offset,
        None => return Ok(vec![]),
    };

    file.seek(SeekFrom::Start(start))?;
    let mut results = vec![];

    while let Ok(mut len_buf) = read_exact_4(&mut file) {
        let len = u32::from_le_bytes(len_buf);
        let mut doc_buf = vec![0u8; len as usize];
        file.read_exact(&mut doc_buf)?;

        let doc: Value = serde_json::from_slice(&doc_buf)?;
        if filter_match(&doc, filter) {
            results.push(doc);
        }
    }

    Ok(results)
}

/// Updates matching documents in a collection.
/// Rewrites the collection block with updated documents.
/// Returns the count of documents updated.
pub fn update(db_path: &str, collection: &str, filter: &Value, update: &Value) -> Result<usize> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(db_path)?;

    // Read and validate header
    let mut header = [0u8; 8];
    file.read_exact(&mut header)?;
    if &header != HEADER_MAGIC {
        anyhow::bail!("Invalid JasDB header");
    }

    // Read TOC block
    let mut toc_buf = vec![0u8; TOC_RESERVED_SIZE];
    file.read_exact(&mut toc_buf)?;
    let toc: HashMap<String, u64> = bincode::deserialize(&toc_buf).unwrap_or_default();

    let offset = match toc.get(collection) {
        Some(off) => *off,
        None => return Ok(0),
    };

    file.seek(SeekFrom::Start(offset))?;

    let mut updated = 0;
    let mut buffer = vec![];

    while let Ok(_) = file.read_exact(&mut [0u8; 4]) {
        file.seek(SeekFrom::Current(-4))?;

        let mut len_buf = [0u8; 4];
        if file.read_exact(&mut len_buf).is_err() {
            break;
        }

        let len = u32::from_le_bytes(len_buf) as usize;
        let mut doc_buf = vec![0u8; len];
        if file.read_exact(&mut doc_buf).is_err() {
            break;
        }

        let mut doc: Value = serde_json::from_slice(&doc_buf)?;
        if filter_match(&doc, filter) {
            doc = update.clone();
            updated += 1;
        }

        let new_raw = serde_json::to_vec(&doc)?;
        let new_len = new_raw.len() as u32;
        buffer.extend(&new_len.to_le_bytes());
        buffer.extend(&new_raw);
    }

    // Rewrite and truncate any leftover data
    file.set_len(offset)?;
    file.seek(SeekFrom::Start(offset))?;
    file.write_all(&buffer)?;

    Ok(updated)
}

/// Deletes matching documents from a collection.
/// Physically rewrites only the non-deleted documents from the original offset.
/// Returns count of deleted documents.
pub fn delete(db_path: &str, collection: &str, filter: &Value) -> Result<usize> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(db_path)?;

    // Validate header
    let mut header = [0u8; 8];
    file.read_exact(&mut header)?;
    if &header != HEADER_MAGIC {
        anyhow::bail!("Invalid JasDB header");
    }

    // Load TOC
    let mut toc_buf = vec![0u8; TOC_RESERVED_SIZE];
    file.read_exact(&mut toc_buf)?;
    let toc: HashMap<String, u64> = bincode::deserialize(&toc_buf).unwrap_or_default();

    let offset = match toc.get(collection) {
        Some(off) => *off,
        None => return Ok(0),
    };

    file.seek(SeekFrom::Start(offset))?;

    let mut new_docs = vec![];
    let mut deleted = 0;
    let mut temp_buf = vec![];

    while let Ok(_) = file.read_exact(&mut [0u8; 4]) {
        file.seek(SeekFrom::Current(-4))?;

        let mut len_buf = [0u8; 4];
        if file.read_exact(&mut len_buf).is_err() {
            break;
        }

        let len = u32::from_le_bytes(len_buf) as usize;
        let mut doc_buf = vec![0u8; len];
        if file.read_exact(&mut doc_buf).is_err() {
            break;
        }

        if let Ok(doc) = serde_json::from_slice::<Value>(&doc_buf) {
            if filter_match(&doc, filter) {
                deleted += 1;
            } else {
                temp_buf.extend(&len_buf);
                temp_buf.extend(&doc_buf);
            }
        }
    }

    // Truncate + rewrite new buffer
    file.set_len(offset)?;
    file.seek(SeekFrom::Start(offset))?;
    file.write_all(&temp_buf)?;

    Ok(deleted)
}


/// Reads exactly 4 bytes and returns array
fn read_exact_4(file: &mut File) -> std::io::Result<[u8; 4]> {
    let mut buf = [0u8; 4];
    match file.read_exact(&mut buf) {
        Ok(_) => Ok(buf),
        Err(e) => Err(e),
    }
}

/// Very basic matcher: checks that all filter keys equal the values in the doc
fn filter_match(doc: &Value, filter: &Value) -> bool {
    if let (Some(doc_map), Some(filter_map)) = (doc.as_object(), filter.as_object()) {
        for (key, val) in filter_map {
            if doc_map.get(key) != Some(val) {
                return false;
            }
        }
        true
    } else {
        false
    }
}
