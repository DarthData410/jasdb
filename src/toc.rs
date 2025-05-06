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

    let mut toc_buf = vec![0u8; TOC_RESERVED_SIZE];
    file.read_exact(&mut toc_buf)?;

    let toc: TocMap = match bincode::deserialize(&toc_buf) {
        Ok(map) => map,
        Err(_) => HashMap::new(),
    };

    Ok(toc)
}

/// Saves TOC map back to reserved area, padding as needed.
pub fn save_toc(file: &mut File, toc: &TocMap) -> std::io::Result<()> {
    file.seek(SeekFrom::Start(HEADER_MAGIC_LEN as u64))?;

    let mut serialized = bincode::serialize(toc).unwrap();
    serialized.resize(TOC_RESERVED_SIZE, 0); // Fixed 1KB for now

    file.write_all(&serialized)?;
    Ok(())
}

/// Sets or updates the schema for a collection in the TOC.
pub fn set_collection_schema(
    file: &mut File,
    collection: &str,
    schema: &Value,
) -> std::io::Result<()> {
    let mut toc = load_toc(file)?;  // Do not manually seek here

    let offset = toc
        .get(collection)
        .map(|entry| entry.offset)
        .unwrap_or_else(|| file.seek(SeekFrom::End(0)).unwrap());

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
    let mut toc = load_toc(file)?;
    if !toc.contains_key(collection) {
        toc.insert(collection.to_string(), TocEntry {
            offset,
            schema: None,
        });
        save_toc(file, &toc)?;
    }
    Ok(())
}
