use anyhow::Result;
use serde_json::Value;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;
use crate::toc::{TocEntry, TocMap, load_toc, save_toc, set_collection_schema};

const HEADER_MAGIC: &[u8] = b"JASDB01\n";
const TOC_RESERVED_SIZE: usize = 1024;

/// Create new JasDB file with header and empty TOC
pub fn create(db_path: &str) -> Result<()> {
    if Path::new(db_path).exists() {
        println!("⚠️ JasDB file already exists: {}", db_path);
        return Ok(());
    }

    let mut file = File::create(db_path)?;
    file.write_all(HEADER_MAGIC)?;
    file.write_all(&vec![0u8; TOC_RESERVED_SIZE])?;
    Ok(())
}

/// Insert document into collection and update TOC if needed
pub fn insert(db_path: &str, collection: &str, doc: &Value) -> Result<()> {
    let mut file = OpenOptions::new().read(true).write(true).open(db_path)?;

    let mut header = [0u8; 8];
    file.read_exact(&mut header)?;
    if &header != HEADER_MAGIC {
        anyhow::bail!("Invalid JasDB header");
    }

    let mut toc = load_toc(&mut file)?;

    // ✅ Enforce schema if present
    if let Some(entry) = toc.get(collection) {
        println!("🔎 Schema for '{}': {:?}", collection, entry.schema);
        if let Some(schema) = &entry.schema {
            if !crate::utils::validate_against_schema(doc, schema) {
                anyhow::bail!("Document does not match collection schema");
            }
        }
    }

    let offset = file.seek(SeekFrom::End(0))?;
    let raw = serde_json::to_vec(doc)?;
    let len = raw.len() as u32;
    file.write_all(&len.to_le_bytes())?;
    file.write_all(&raw)?;

    if !toc.contains_key(collection) {
        toc.insert(collection.to_string(), TocEntry { offset, schema: None });
        save_toc(&mut file, &toc)?;
    }

    Ok(())
}

/// Query documents in a collection matching a filter
pub fn query(db_path: &str, collection: &str, filter: &Value) -> Result<Vec<Value>> {
    let mut file = File::open(db_path)?;

    let mut header = [0u8; 8];
    file.read_exact(&mut header)?;
    if &header != HEADER_MAGIC {
        anyhow::bail!("Invalid JasDB header");
    }

    let toc = load_toc(&mut file)?;
    let offset = match toc.get(collection) {
        Some(entry) => entry.offset,
        None => return Ok(vec![]),
    };

    file.seek(SeekFrom::Start(offset))?;
    let mut results = vec![];

    while let Ok(len_buf) = read_exact_4(&mut file) {
        let len = u32::from_le_bytes(len_buf);
        let mut buf = vec![0u8; len as usize];
        file.read_exact(&mut buf)?;
        let doc: Value = serde_json::from_slice(&buf)?;
        if filter_match(&doc, filter) {
            results.push(doc);
        }
    }

    Ok(results)
}

/// Update documents in a collection matching a filter
pub fn update(db_path: &str, collection: &str, filter: &Value, update: &Value) -> Result<usize> {
    let mut file = OpenOptions::new().read(true).write(true).open(db_path)?;

    let mut header = [0u8; 8];
    file.read_exact(&mut header)?;
    if &header != HEADER_MAGIC {
        anyhow::bail!("Invalid JasDB header");
    }

    let toc = load_toc(&mut file)?;
    let offset = match toc.get(collection) {
        Some(entry) => entry.offset,
        None => return Ok(0),
    };

    file.seek(SeekFrom::Start(offset))?;
    let mut updated = 0;
    let mut buffer = vec![];

    while let Ok(_) = file.read_exact(&mut [0u8; 4]) {
        file.seek(SeekFrom::Current(-4))?;

        let mut len_buf = [0u8; 4];
        if file.read_exact(&mut len_buf).is_err() { break; }

        let len = u32::from_le_bytes(len_buf) as usize;
        let mut buf = vec![0u8; len];
        if file.read_exact(&mut buf).is_err() { break; }

        let mut doc: Value = serde_json::from_slice(&buf)?;
        if filter_match(&doc, filter) {
            doc = update.clone();
            updated += 1;
        }

        let new_raw = serde_json::to_vec(&doc)?;
        let new_len = new_raw.len() as u32;
        buffer.extend(&new_len.to_le_bytes());
        buffer.extend(&new_raw);
    }

    file.set_len(offset)?;
    file.seek(SeekFrom::Start(offset))?;
    file.write_all(&buffer)?;

    Ok(updated)
}

/// Delete documents in a collection matching a filter
pub fn delete(db_path: &str, collection: &str, filter: &Value) -> Result<usize> {
    let mut file = OpenOptions::new().read(true).write(true).open(db_path)?;

    let mut header = [0u8; 8];
    file.read_exact(&mut header)?;
    if &header != HEADER_MAGIC {
        anyhow::bail!("Invalid JasDB header");
    }

    let toc = load_toc(&mut file)?;
    let offset = match toc.get(collection) {
        Some(entry) => entry.offset,
        None => return Ok(0),
    };

    file.seek(SeekFrom::Start(offset))?;
    let mut temp_buf = vec![];
    let mut deleted = 0;

    while let Ok(_) = file.read_exact(&mut [0u8; 4]) {
        file.seek(SeekFrom::Current(-4))?;

        let mut len_buf = [0u8; 4];
        if file.read_exact(&mut len_buf).is_err() { break; }

        let len = u32::from_le_bytes(len_buf) as usize;
        let mut buf = vec![0u8; len];
        if file.read_exact(&mut buf).is_err() { break; }

        if let Ok(doc) = serde_json::from_slice::<Value>(&buf) {
            if filter_match(&doc, filter) {
                deleted += 1;
            } else {
                temp_buf.extend(&len_buf);
                temp_buf.extend(&buf);
            }
        }
    }

    file.set_len(offset)?;
    file.seek(SeekFrom::Start(offset))?;
    file.write_all(&temp_buf)?;

    Ok(deleted)
}

/// Set or update the schema for a collection
pub fn set_schema(db_path: &str, collection: &str, schema: &Value) -> Result<()> {
    let mut file = OpenOptions::new().read(true).write(true).open(db_path)?;
    let mut header = [0u8; 8];
    file.read_exact(&mut header)?;
    if &header != HEADER_MAGIC {
        anyhow::bail!("Invalid JasDB header");
    }

    set_collection_schema(&mut file, collection, schema)?;
    Ok(())
}

fn read_exact_4(file: &mut File) -> std::io::Result<[u8; 4]> {
    let mut buf = [0u8; 4];
    file.read_exact(&mut buf)?;
    Ok(buf)
}

fn filter_match(doc: &Value, filter: &Value) -> bool {
    if let (Some(d), Some(f)) = (doc.as_object(), filter.as_object()) {
        for (k, v) in f {
            if d.get(k) != Some(v) {
                return false;
            }
        }
        true
    } else {
        false
    }
}
