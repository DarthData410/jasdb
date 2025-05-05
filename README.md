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

## 🔧 Tech Stack

- **Node.js** (LTS)
- **Binary storage engine** (custom, MessagePack-based)
- **SQLite3 (optional)** for metadata/indexing layer
- **Express** (for API endpoints)
- **BullMQ + Redis (optional)** for queued jobs
- **PM2** integration for multi-instance orchestration

---

## 🧱 Architecture Modes

1. **Embedded Mode**  
   Drop JasDB into your Node.js app — think of it like a secure, smarter `nedb`.

2. **Service Mode (`jasd`)**  
   Run as a daemon exposing API endpoints for microservices, with secure tokens + REST.

---

## 🗂️ Sample Use Cases

- Secure storage for task metadata and event logs
- Local config and state store for distributed services
- Replace Firebase/MongoDB for embedded/local JSON apps
- Expose internal data as controlled read-only REST views

---

## 🚀 Quick Start

```bash
npm install jasdb
