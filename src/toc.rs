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
