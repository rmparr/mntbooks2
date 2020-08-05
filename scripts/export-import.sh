#!/bin/bash

# warning! deletes all data in ../mntbooks.sqlite

# compile uuid extension for sqlite3
cd sqlite-uuid
./build.sh
cd ..

sqlite3 ../../mntbooks/receipts.db <mntbooks-legacy-export.sql
sqlite3 ../mntbooks.sqlite <mntbooks-legacy-import.sql

