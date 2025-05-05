# JasDB 🛡️

**JasDB** (JSON Access Secure Database) is a secure, embedded, JSON-native microservice database built for lightweight, high-performance, and secure data handling in modern server environments.

Designed specifically for **Node.js microservices**, JasDB combines:
- 🔒 **Security-first architecture**
- 🧩 **Native JSON document storage**
- ⚡ **Efficient binary encoding**
- 🧵 **Built-in REST endpoint support**
- 🛠️ **PM2 and BullMQ-friendly integration**

---

## ✨ Key Features

- **📦 JSON-native**: Insert, query, and update structured JSON documents directly.
- **🔐 End-to-end encryption**: Data at rest is stored in binary-encoded, AES-encrypted format.
- **🧠 Smart indexing**: Index any nested path with fast, flexible lookup support.
- **🌐 RESTful endpoints**: Collections and views can be exposed directly as secure API routes.
- **🧰 Local-first**: Embedded or daemon modes, ideal for VPS, edge, and container setups.
- **🧪 Built-in testing harness**: Schema validation and query simulations built-in.

---

## 🗂️ Sample Use Cases

- Secure storage for task metadata and event logs
- Local config and state store for distributed services
- Replace Firebase/MongoDB for embedded/local JSON apps
- Expose internal data as controlled read-only REST views

---

## 🚀 .jasdb File structure

[Header]
  - Version
  - Global settings
  - Encryption info

[TOC - Table of Contents]
  - Collection names
  - Offsets to data blocks
  - Index info
  - Permissions

[Data Blocks]
  - Collection: apples → [binary doc1][doc2]...
  - Collection: bananas → [binary doc1][doc2]...

[Index Section]
  - Map of field → offset (B-tree index)

[Permissions / Views]
  - Role → Collection → Allowed fields/filters

[Footer]
  - Hash/checksum
  - File signature

---

## Important JasDB semantics

| Concept  | Equivalent In SQL | JasDB Term                |
| -------- | ----------------- | ------------------------- |
| Database | SQLite file       | `.jasdb` binary file      |
| Table    | Table             | **Collection**            |
| Row      | Row               | **Entry** or **Document** |
| Column   | Field             | JSON key-path             |

---

## CLI command examples

# Create New DB:
jasdb create -p json.jasdb

# Expected Output:
✅ Created new JasDB file: json.jasdb

# Insert Document:
jasdb insert -c apples -d '{"type":"Gala","price":1.99}' -p json.jasdb
jasdb insert -c apples -d '{"type":"Fuji","price":2.50}' -p json.jasdb

# Query Documents:
jasdb find -c apples -f '{}' -p json.jasdb

# Expected Output:
[
  {
    "type": "Gala",
    "price": 1.99
  },
  {
    "type": "Fuji",
    "price": 2.5
  }
]

# Update Document:
jasdb update -c apples -f '{"type":"Gala"}' -u '{"type":"Gala","price":2.25}' -p json.jasdb

# Expected Output:
🔄 Updated 1 document(s) in 'apples'

# Delete Document:
jasdb delete -c apples -f '{"type":"Fuji"}' -p json.jasdb

# Expected Output:
🗑️ Deleted 1 document(s) from 'apples'

