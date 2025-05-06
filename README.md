# JasDB 🛡️

``` bash
     _~^~^~_
 \) /  o o  \ (/ 
  ' _   u   _ '
   \ '-----' /
      JasDB
 Powered by Rust!

 https://github.com/DarthData410/jasdb
 v0.1.2
```

**JasDB** (JSON Access Secure Database) is a secure, embedded, JSON-native microservice database built for lightweight, high-performance data handling in modern server environments.

Designed specifically for **Node.js microservices**, JasDB combines:

- 🔒 **Security-first architecture**
- 🧩 **Native JSON document storage**
- ⚡ **Efficient binary encoding**
- 🧵 **Built-in REST endpoint support**
- 🛠️ **PM2 and BullMQ-friendly integration**

---

## ✨ Key Features

- 📦 **JSON-native** — Insert, query, and update structured JSON documents directly.
- 🔐 **End-to-end encryption** — AES-encrypted, binary-encoded data at rest.
- 🧠 **Smart indexing** — Fast, flexible lookups on any nested JSON path.
- 🌐 **RESTful endpoints** — Securely expose collections and views as API routes.
- 🧰 **Local-first** — Embedded or daemon modes ideal for VPS, edge, and container deployments.
- 🧪 **Built-in testing harness** — Includes schema validation and query simulation tools.

---

## 🗂️ Sample Use Cases

- Secure storage for task metadata and event logs
- Local config and state store for distributed services
- Firebase/MongoDB alternative for embedded/local JSON apps
- Controlled, read-only REST views over internal data

---

## 🚀 `.jasdb` File Structure

```
[Header]
  - Version
  - Global settings
  - Encryption info

[TOC - Table of Contents]
  - Collection names
  - Data block offsets
  - Index info
  - Permissions

[Data Blocks]
  - Collection: apples → [binary doc1][doc2]...
  - Collection: bananas → [binary doc1][doc2]...

[Index Section]
  - Field → Offset (B-tree map)

[Permissions / Views]
  - Role → Collection → Allowed fields/filters

[Footer]
  - Hash/checksum
  - File signature
```

---

## 📘 JasDB Concepts vs SQL

| SQL Concept | JasDB Equivalent         | Description                      |
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

> Built for speed. Secured by design. Powered by simplicity.  
> **JasDB** — Your JSON-native microservice database.