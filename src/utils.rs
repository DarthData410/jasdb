/// Prints the Ferris crab ASCII art with JasDB branding
pub fn print_ferris() {
    let version = env!("CARGO_PKG_VERSION");
    println!(
r#"
     _~^~^~_
 \) /  o o  \ (/
  ' _   u   _ '
   \ '-----' /
      JasDB
 Powered by Rust!

 https://github.com/DarthData410/jasdb
 v{}
"#,
        version
    );
}

/// Validates a document against a basic schema by checking required fields.
pub fn validate_against_schema(doc: &serde_json::Value, schema: &serde_json::Value) -> bool {
    if let (Some(doc_obj), Some(schema_obj)) = (doc.as_object(), schema.as_object()) {
        for key in schema_obj.keys() {
            if !doc_obj.contains_key(key) {
                return false;
            }
        }
        true
    } else {
        false
    }
}
