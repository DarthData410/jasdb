// toc.rs
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

const TOC_RESERVED_SIZE: usize = 1024;
const HEADER_MAGIC_LEN: usize = 8;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TocEntry {
    pub offset: u64,
    pub schema: Option<Value>,
}

pub type TocMap = HashMap<String, TocEntry>;

/// Loads TOC from reserved block. Returns empty if corrupted or empty.
pub fn load_toc(file: &mut File) -> std::io::Result<TocMap> {
    file.seek(SeekFrom::Start(HEADER_MAGIC_LEN as u64))?;
    println!("📥 Seeking to TOC start @ byte {}", HEADER_MAGIC_LEN);

    let mut toc_buf = vec![0u8; TOC_RESERVED_SIZE];
    file.read_exact(&mut toc_buf)?;
    println!("📥 Read {} bytes from TOC area", TOC_RESERVED_SIZE);

    let actual_len = toc_buf.iter().rposition(|&b| b != 0).map(|i| i + 1).unwrap_or(0);
    println!("📖 Non-zero TOC length: {}", actual_len);

    let toc: TocMap = match bincode::deserialize(&toc_buf[..actual_len]) {
        Ok(map) => {
            println!("✅ TOC deserialized with {} collections", map.len());
            for (k, v) in &map {
                println!("  • '{}': offset={}, schema={:?}", k, v.offset, v.schema);
            }
            map
        },
        Err(e) => {
            println!("❌ Failed to deserialize TOC: {}", e);
            HashMap::new()
        }
    };

    Ok(toc)
}

/// Saves TOC map back to reserved area, padding as needed.
pub fn save_toc(file: &mut File, toc: &TocMap) -> std::io::Result<()> {
    println!("💾 Saving TOC with {} collection(s)...", toc.len());

    for (k, v) in toc {
        println!("  💾 '{}' -> offset: {}, schema: {:?}", k, v.offset, v.schema);
    }

    file.seek(SeekFrom::Start(HEADER_MAGIC_LEN as u64))?;
    let mut serialized = bincode::serialize(toc).unwrap();
    serialized.resize(TOC_RESERVED_SIZE, 0); // Fixed 1KB for now

    file.write_all(&serialized)?;
    println!("✅ TOC saved successfully.");
    Ok(())
}

/// Sets or updates the schema for a collection in the TOC.
pub fn set_collection_schema(
    file: &mut File,
    collection: &str,
    schema: &Value,
) -> std::io::Result<()> {
    println!("📐 Setting schema for collection '{}': {}", collection, schema);

    let mut toc = load_toc(file)?;

    let offset = toc
        .get(collection)
        .map(|entry| entry.offset)
        .unwrap_or_else(|| {
            let end = file.seek(SeekFrom::End(0)).unwrap();
            println!("📍 No existing offset for '{}', using EOF: {}", collection, end);
            end
        });

    toc.insert(
        collection.to_string(),
        TocEntry {
            offset,
            schema: Some(schema.clone()),
        },
    );

    save_toc(file, &toc)
}

/// Ensures a collection has a TOC entry with offset if not already present.
pub fn ensure_collection_entry(
    file: &mut File,
    collection: &str,
    offset: u64,
) -> std::io::Result<()> {
    println!("🔎 Ensuring TOC entry for '{}'", collection);

    let mut toc = load_toc(file)?;
    if !toc.contains_key(collection) {
        println!("➕ Inserting new TOC entry for '{}'", collection);
        toc.insert(collection.to_string(), TocEntry {
            offset,
            schema: None,
        });
        save_toc(file, &toc)?;
    } else {
        println!("✅ TOC already has entry for '{}'", collection);
    }

    Ok(())
}
