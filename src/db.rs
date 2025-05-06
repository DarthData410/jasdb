use anyhow::Result;
use serde_json::Value;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

use crate::toc::{ensure_collection_entry, load_toc, set_collection_schema, validate_collection_schema};
use crate::utils::debug;
use crate::io::{
    read_exact_at, write_exact_at, get_eof, write_at, HEADER_MAGIC, TOC_RESERVED_SIZE
};
use crate::lock::{with_exclusive_access, with_shared_access};

/// Create new JasDB file with header and empty TOC
pub fn create(db_path: &str) -> Result<()> {
    if Path::new(db_path).exists() {
        debug(&format!("⚠️ JasDB file already exists: {}", db_path));
        return Ok(());
    }

    let mut file = File::create(db_path)?;
    write_at(&mut file, 0, HEADER_MAGIC)?; // ✅ Use io::write_at
    write_at(&mut file, HEADER_MAGIC.len() as u64, &vec![0u8; TOC_RESERVED_SIZE])?; // ✅ Write reserved TOC block
    Ok(())
}

/// Insert document into collection and update TOC if needed
pub fn insert(db_path: &str, collection: &str, doc: &Value) -> Result<()> {
    with_exclusive_access(db_path, |file| {
        let mut header = [0u8; 8];
        file.read_exact(&mut header)?;
        if &header != HEADER_MAGIC {
            anyhow::bail!("Invalid JasDB header");
        }

        validate_collection_schema(file, collection, doc)?;

        let offset = get_eof(file)?;
        let raw = serde_json::to_vec(doc)?;
        let len = raw.len() as u32;
        write_exact_at(file, offset, &len.to_le_bytes())?;
        write_exact_at(file, offset + 4, &raw)?;

        ensure_collection_entry(file, collection, offset)?;
        debug(&format!("✅ Document inserted at offset {}", offset));

        Ok(())
    })
}

/// Query documents in a collection matching a filter
pub fn query(db_path: &str, collection: &str, filter: &Value) -> Result<Vec<Value>> {
    with_shared_access(db_path, |file| {
        let mut header = [0u8; 8];
        file.read_exact(&mut header)?;
        if &header != HEADER_MAGIC {
            anyhow::bail!("Invalid JasDB header");
        }

        let toc = load_toc(file)?;
        let offset = match toc.get(collection) {
            Some(entry) => entry.offset,
            None => return Ok(vec![]),
        };

        let mut results = vec![];
        let mut pos = offset;

        while let Ok(len_buf) = read_exact_at(file, pos, 4) {
            let len = u32::from_le_bytes(len_buf.try_into().unwrap());
            let buf = read_exact_at(file, pos + 4, len as usize)?;
            let doc: Value = serde_json::from_slice(&buf)?;
            if filter_match(&doc, filter) {
                results.push(doc);
            }
            pos += 4 + len as u64;
        }

        Ok(results)
    })
}

/// Update documents in a collection matching a filter
pub fn update(db_path: &str, collection: &str, filter: &Value, update: &Value) -> Result<usize> {
    with_exclusive_access(db_path, |file| {
        let mut header = [0u8; 8];
        file.read_exact(&mut header)?;
        if &header != HEADER_MAGIC {
            anyhow::bail!("Invalid JasDB header");
        }

        let toc = load_toc(file)?;
        let offset = match toc.get(collection) {
            Some(entry) => entry.offset,
            None => return Ok(0),
        };

        let mut updated = 0;
        let mut buffer = vec![];
        let mut pos = offset;

        while let Ok(len_buf) = read_exact_at(file, pos, 4) {
            let len = u32::from_le_bytes(len_buf.clone().try_into().unwrap()) as usize;
            let buf = read_exact_at(file, pos + 4, len)?;
            let mut doc: Value = serde_json::from_slice(&buf)?;
            if filter_match(&doc, filter) {
                doc = update.clone();
                updated += 1;
            }
            let new_raw = serde_json::to_vec(&doc)?;
            let new_len = new_raw.len() as u32;
            buffer.extend(&new_len.to_le_bytes());
            buffer.extend(&new_raw);
            pos += 4 + len as u64;
        }

        file.set_len(offset)?;
        write_exact_at(file, offset, &buffer)?;

        debug(&format!("🔁 Updated {} document(s) in '{}'", updated, collection));
        Ok(updated)
    })
}

/// Delete documents in a collection matching a filter
pub fn delete(db_path: &str, collection: &str, filter: &Value) -> Result<usize> {
    with_exclusive_access(db_path, |file| {
        let mut header = [0u8; 8];
        file.read_exact(&mut header)?;
        if &header != HEADER_MAGIC {
            anyhow::bail!("Invalid JasDB header");
        }

        let toc = load_toc(file)?;
        let offset = match toc.get(collection) {
            Some(entry) => entry.offset,
            None => return Ok(0),
        };

        let mut temp_buf = vec![];
        let mut deleted = 0;
        let mut pos = offset;

        while let Ok(len_buf) = read_exact_at(file, pos, 4) {
            let len = u32::from_le_bytes(len_buf.clone().try_into().unwrap()) as usize;
            let buf = read_exact_at(file, pos + 4, len)?;

            if let Ok(doc) = serde_json::from_slice::<Value>(&buf) {
                if filter_match(&doc, filter) {
                    deleted += 1;
                } else {
                    temp_buf.extend(&len_buf);
                    temp_buf.extend(&buf);
                }
            }
            pos += 4 + len as u64;
        }

        file.set_len(offset)?;
        write_exact_at(file, offset, &temp_buf)?;

        debug(&format!("🗑️ Deleted {} document(s) from '{}'", deleted, collection));
        Ok(deleted)
    })
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
