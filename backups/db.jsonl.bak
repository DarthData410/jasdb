use anyhow::Result;
use serde_json::Value;
use std::fs::{OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

/// Create a new JasDB file with header.
/// Writes magic bytes to identify file format and version.
pub fn create(db_path: &str) -> Result<()> {
    if Path::new(db_path).exists() {
        println!("⚠️ JasDB file already exists: {}", db_path);
        return Ok(());
    }

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(db_path)?;

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

    let line = format!(
        "{{\"__collection\":\"{}\",\"doc\":{}}}\n",
        collection,
        serde_json::to_string(doc)?
    );

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

/// Updates matching documents in a collection by replacing them.
/// Returns the count of updated documents.
pub fn update(db_path: &str, collection: &str, filter: &Value, update: &Value) -> Result<usize> {
    if !Path::new(db_path).exists() {
        return Ok(0);
    }

    let mut file = OpenOptions::new()
        .read(true)
        .open(db_path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut new_lines = Vec::new();
    let mut updated = 0;

    for line in contents.lines() {
        if let Ok(mut root) = serde_json::from_str::<Value>(line) {
            if root.get("__collection") == Some(&Value::String(collection.to_string())) {
                if let Some(doc) = root.get_mut("doc") {
                    if filter_match(doc, filter) {
                        *doc = update.clone();
                        updated += 1;
                    }
                }
            }
            new_lines.push(serde_json::to_string(&root)?);
        }
    }

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(db_path)?;
    file.write_all(new_lines.join("\n").as_bytes())?;
    file.write_all(b"\n")?;

    Ok(updated)
}

/// Deletes matching documents from a collection.
/// Returns the count of deleted documents.
pub fn delete(db_path: &str, collection: &str, filter: &Value) -> Result<usize> {
    if !Path::new(db_path).exists() {
        return Ok(0);
    }

    let mut file = OpenOptions::new()
        .read(true)
        .open(db_path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut new_lines = Vec::new();
    let mut deleted = 0;

    for line in contents.lines() {
        if let Ok(root) = serde_json::from_str::<Value>(line) {
            if root.get("__collection") == Some(&Value::String(collection.to_string())) {
                if let Some(doc) = root.get("doc") {
                    if filter_match(doc, filter) {
                        deleted += 1;
                        continue; // Skip this line (i.e., delete it)
                    }
                }
            }
            new_lines.push(serde_json::to_string(&root)?);
        }
    }

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(db_path)?;
    file.write_all(new_lines.join("\n").as_bytes())?;
    file.write_all(b"\n")?;

    Ok(deleted)
}

/// Basic filter matcher: checks if all keys in filter equal doc[key]
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
