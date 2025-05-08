# JasDB

``` bash
     _~^~^~_
 \) /  o o  \ (/ 
  ' _   u   _ '
   \ '-----' /
      JasDB
 Powered by Rust!

 https://github.com/DarthData410/jasdb
 v0.2.0
```

**JasDB** (JSON Access Secure Database) is a secure, embedded, JSON-native microservice database built for lightweight, high-performance data handling in modern server environments.

Designed specifically for **Node.js microservices**, JasDB combines:

- 🧩 **Native JSON document storage**
- ⚡ **Efficient binary encoding**
- 🛠️ **NodeJS integration**

---

## ✨ Key Features

- 📦 **JSON-native** — Insert, query, and update structured JSON documents directly.
- 🧠 **Document/Collection indexing** — Fast, flexible lookups on any nested JSON path.
- 🧰 **Local-first** — Embedded or daemon modes ideal for VPS, edge, and container deployments.
- 🧪 **Built-in testing harness** — Includes schema validation and query simulation tools.
- 🔐 **End-to-end encryption** — AES-encrypted, binary-encoded data at rest options.

---

## 🗂️ Sample Use Cases

- NodeJ microservices/apps JSON datastore
- Secure storage for task metadata and event logs
- Local config and state store for distributed services
- Firebase/MongoDB alternative for embedded/local NodeJS/JSON apps
- A true JSON native db alternative to SQLite3 for development

---

## 📘 JasDB Concepts vs SQL

| SQL Concept | JasDB Equivalent          | Description                      |
|-------------|---------------------------|----------------------------------|
| Database    | `.jasdb` binary file      | One file per database            |
| Table       | **Collection**            | Stores grouped documents         |
| Row         | **Entry** / **Document**  | Each JSON object                 |
| Column      | JSON key-path             | Supports deep nested fields      |

---

## 🧪 CLI Examples

```bash
# Create New DB:
jasdb create -p json.jasdb

# Output:
✅ Created new JasDB file: json.jasdb
```

```bash
# Insert Documents:
jasdb insert -c apples -d '{"type":"Gala","price":1.99}' -p json.jasdb
jasdb insert -c apples -d '{"type":"Fuji","price":2.50}' -p json.jasdb
```

```bash
# Query Documents:
jasdb find -c apples -f '{}' -p json.jasdb

# Output:
[
  { "type": "Gala", "price": 1.99 },
  { "type": "Fuji", "price": 2.5 }
]
```

```bash
# Update Document:
jasdb update -c apples -f '{"type":"Gala"}' -u '{"type":"Gala","price":2.25}' -p json.jasdb

# Output:
🔄 Updated 1 document(s) in 'apples'
```

```bash
# Delete Document:
jasdb delete -c apples -f '{"type":"Fuji"}' -p json.jasdb

# Output:
🗑️ Deleted 1 document(s) from 'apples'
```

---

## Development Plan
```bash
Phase 1 – Core Infrastructure

 Define new file header (Magic + TOC offsets)

 Implement centralized TOC management (load/save/track dynamic sections)

 Abstract sections (Schema, Collection, Index) into self-managed types

 Add tombstone support (soft delete)

Phase 2 – Data I/O Implementation

 Encode/decode collections as binary JSON

 Schema validation per collection

 Basic CLI: create DB, add schema, insert doc, list docs

Phase 3 – Indexing Layer

 B-Tree serialization format

 Build/query on-demand indexes

 Allow multi-field composite indexes

Phase 4 – Maintenance Tools

 compact command

Lock

Clean tombstones

Reorganize binary layout

Rewrite file + update header TOC

 encrypt command (Future)

 compress command (Future)
```
---

## Code Abstraction Plan

```bash
db.rs / index.rs / schema.rs
   ↑
filemanager.rs        ← Orchestrates high-level flow, marshals sections
   ↑
 ┌───────────────┐
 │ header.rs     │ ← Reads/writes DB magic & TOC start/end offsets
 │ footer.rs     │ ← Handles EOF footer markers, hashes, versioning
 │ tombstone.rs  │ ← Marks & detects deleted entries (for compaction)
 └───────────────┘
   ↑
 ┌────────────┐
 │ io.rs      │ ← Byte-level reads/writes, offset control
 │ lock.rs    │ ← OS-level read/write/process locks
 └────────────┘
```
---

> Built for speed. Secured by design. Powered by simplicity.  
> **JasDB** — Your JSON-native microservice database.