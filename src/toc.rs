// toc.rs
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

const TOC_RESERVED_SIZE: usize = 1024;
const HEADER_MAGIC_LEN: usize = 8; // Length of "JASDB01\n"

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TocEntry {
    pub offset: u64,
    pub schema: Option<Value>,
}

pub type TocMap = HashMap<String, TocEntry>;

pub fn load_toc(file: &mut File) -> std::io::Result<TocMap> {
    file.seek(SeekFrom::Start(HEADER_MAGIC_LEN as u64))?;
    let mut toc_buf = vec![0u8; TOC_RESERVED_SIZE];
    file.read_exact(&mut toc_buf)?;
    let toc: TocMap = bincode::deserialize(&toc_buf).unwrap_or_default();
    Ok(toc)
}

pub fn save_toc(file: &mut File, toc: &TocMap) -> std::io::Result<()> {
    file.seek(SeekFrom::Start(HEADER_MAGIC_LEN as u64))?;
    let mut serialized = bincode::serialize(toc).unwrap();
    serialized.resize(TOC_RESERVED_SIZE, 0);
    file.write_all(&serialized)?;
    Ok(())
}
