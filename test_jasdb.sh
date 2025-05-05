#!/bin/bash

set -e
DBFILE="swc.jasdb"

echo "🧪 Starting JasDB Tests..."

# Clean up
rm -f $DBFILE

# Create new database
jasdb create -p $DBFILE

# Set schema (testing new feature)
jasdb schema -c apples -s '{"type": "string", "price": "number"}' -p $DBFILE

# Insert valid docs
jasdb insert -c apples -d '{"type":"Gala","price":1.99}' -p $DBFILE
jasdb insert -c apples -d '{"type":"Fuji","price":2.50}' -p $DBFILE

# Find all
jasdb find -c apples -f '{}' -p $DBFILE

# Update one
jasdb update -c apples -f '{"type":"Gala"}' -u '{"type":"Gala","price":2.25}' -p $DBFILE
jasdb find -c apples -f '{}' -p $DBFILE

# Delete one
jasdb delete -c apples -f '{"type":"Fuji"}' -p $DBFILE
jasdb find -c apples -f '{}' -p $DBFILE

# Optional: try inserting invalid schema
echo "⛔ Trying to insert invalid doc (missing field)..."
if ! jasdb insert -c apples -d '{"type":"RedDelicious"}' -p $DBFILE; then
  echo "✅ Properly rejected invalid document"
else
  echo "❌ Invalid document was inserted — schema check failed"
  exit 1
fi

echo "🧬 Hexdump of database:"
hexdump -C $DBFILE | head -n 40

echo "✅ All tests completed!"
