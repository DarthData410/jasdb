use anyhow::Result;
use serde_json::Value;
use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use std::path::Path;

pub fn insert(collection: &str, doc: &Value) -> Result<()> {
    let path = format!("./data/{}.jsonl", collection);
    create_dir_all("./data")?;

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)?;

    let line = serde_json::to_string(doc)? + "\n";
    file.write_all(line.as_bytes())?;
    Ok(())
}

pub fn query(collection: &str, _filter: &Value) -> Result<Vec<Value>> {
    let path = format!("./data/{}.jsonl", collection);
    if !Path::new(&path).exists() {
        return Ok(vec![]);
    }

    let data = std::fs::read_to_string(&path)?;
    let mut results = vec![];

    for line in data.lines() {
        if let Ok(val) = serde_json::from_str::<Value>(line) {
            results.push(val);
        }
    }

    Ok(results)
}
