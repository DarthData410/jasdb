// toc.rs
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;

use crate::utils::debug;
use crate::io::{read_at, write_at, get_eof, TOC_RESERVED_SIZE, HEADER_MAGIC_LEN};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TocEntry {
    pub offset: u64,
    pub schema: Option<Vec<u8>>, // Store schema as raw bytes
}

pub type TocMap = HashMap<String, TocEntry>;

/// Loads TOC from reserved block. Returns empty if corrupted or empty.
pub fn load_toc(file: &mut File) -> std::io::Result<TocMap> {
    debug(&format!("📥 Seeking to TOC start @ byte {}", HEADER_MAGIC_LEN));

    let toc_buf = read_at(file, HEADER_MAGIC_LEN as u64, TOC_RESERVED_SIZE)?;
    debug(&format!("📥 Read {} bytes from TOC area", TOC_RESERVED_SIZE));

    let actual_len = toc_buf.iter().rposition(|&b| b != 0).map(|i| i + 1).unwrap_or(0);
    debug(&format!("📖 Non-zero TOC length: {}", actual_len));

    let toc: TocMap = match bincode::deserialize::<TocMap>(&toc_buf[..actual_len]) {
        Ok(map) => {
            debug(&format!("✅ TOC deserialized with {} collections", map.len()));
            for (k, v) in &map {
                debug(&format!("  • '{}': offset={}, schema_len={:?}", k, v.offset, v.schema.as_ref().map(|s| s.len())));
            }
            map
        },
        Err(e) => {
            debug(&format!("❌ Failed to deserialize TOC: {}", e));
            HashMap::new()
        }
    };

    Ok(toc)
}

/// Saves TOC map back to reserved area, padding as needed.
pub fn save_toc(file: &mut File, toc: &TocMap) -> std::io::Result<()> {
    debug(&format!("💾 Saving TOC with {} collection(s)...", toc.len()));

    for (k, v) in toc {
        debug(&format!("  💾 '{}' -> offset: {}, schema_len: {:?}", k, v.offset, v.schema.as_ref().map(|s| s.len())));
    }

    let mut serialized = bincode::serialize(toc).unwrap();
    serialized.resize(TOC_RESERVED_SIZE, 0); // Fixed 1KB for now

    write_at(file, HEADER_MAGIC_LEN as u64, &serialized)?;
    debug("✅ TOC saved successfully.");
    Ok(())
}

/// Sets or updates the schema for a collection in the TOC.
pub fn set_collection_schema(
    file: &mut File,
    collection: &str,
    schema: &Value,
) -> std::io::Result<()> {
    debug(&format!("📐 Setting schema for collection '{}': {}", collection, schema));

    let mut toc = load_toc(file)?;

    let offset = toc
        .get(collection)
        .map(|entry| entry.offset)
        .unwrap_or_else(|| {
            let end = get_eof(file).unwrap();
            debug(&format!("📍 No existing offset for '{}', using EOF: {}", collection, end));
            end
        });

    let schema_bytes = serde_json::to_vec(schema).unwrap();

    toc.insert(
        collection.to_string(),
        TocEntry {
            offset,
            schema: Some(schema_bytes),
        },
    );

    save_toc(file, &toc)
}

/// Validates a document against the stored schema.
pub fn validate_collection_schema(
    file: &mut File,
    collection: &str,
    doc: &Value
) -> std::io::Result<()> {
    let toc = load_toc(file)?;
    if let Some(entry) = toc.get(collection) {
        if let Some(schema_bytes) = &entry.schema {
            let schema: Value = match serde_json::from_slice(schema_bytes) {
                Ok(v) => v,
                Err(e) => {
                    debug(&format!("❌ Failed to parse stored schema for '{}': {}", collection, e));
                    return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid schema"));
                }
            };
            debug(&format!("🔎 Validating doc against schema for '{}': {}", collection, schema));
            if !crate::utils::validate_against_schema(doc, &schema) {
                debug(&format!("❌ Schema validation failed for collection '{}'", collection));
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Document does not match schema"));
            }
        }
    }
    Ok(())
}

/// Ensures a collection has a TOC entry with offset if not already present.
pub fn ensure_collection_entry(
    file: &mut File,
    collection: &str,
    offset: u64,
) -> std::io::Result<()> {
    debug(&format!("🔎 Ensuring TOC entry for '{}'", collection));

    let mut toc = load_toc(file)?;
    if !toc.contains_key(collection) {
        debug(&format!("➕ Inserting new TOC entry for '{}'", collection));
        toc.insert(collection.to_string(), TocEntry {
            offset,
            schema: None,
        });
        save_toc(file, &toc)?;
    } else {
        debug(&format!("✅ TOC already has entry for '{}'", collection));
    }

    Ok(())
}
