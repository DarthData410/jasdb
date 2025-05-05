use anyhow::Result;
use serde_json::Value;
use std::fs::{OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

/// Create a new JasDB file with header.
/// Writes magic bytes to identify file format and version.
pub fn create(db_path: &str) -> Result<()> {
    // Only create if it doesn't exist
    if Path::new(db_path).exists() {
        println!("⚠️ JasDB file already exists: {}", db_path);
        return Ok(());
    }

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(db_path)?;

    // Write a simple header (magic bytes + version)
    file.write_all(b"JASDB01\n")?;
    Ok(())
}

/// Inserts a JSON document into the specified collection
/// in the given JasDB file. Appends in plain text (JSON lines format for now).
pub fn insert(db_path: &str, collection: &str, doc: &Value) -> Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(db_path)?;

    let line = format!("{{\"__collection\":\"{}\",\"doc\":{}}}\n", collection, serde_json::to_string(doc)?);
    file.write_all(line.as_bytes())?;
    Ok(())
}

/// Queries all documents from a given collection
/// in the specified JasDB file. Filter is not yet implemented.
pub fn query(db_path: &str, collection: &str, _filter: &Value) -> Result<Vec<Value>> {
    if !Path::new(db_path).exists() {
        return Ok(vec![]);
    }

    let mut file = OpenOptions::new()
        .read(true)
        .open(db_path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut results = Vec::new();

    for line in contents.lines() {
        if let Ok(root) = serde_json::from_str::<Value>(line) {
            if root.get("__collection") == Some(&Value::String(collection.to_string())) {
                if let Some(doc) = root.get("doc") {
                    results.push(doc.clone());
                }
            }
        }
    }

    Ok(results)
}
