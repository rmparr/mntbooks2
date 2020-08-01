#!/bin/bash

# warning! deletes all data in ../mntbooks.sqlite

sqlite3 ../../mntbooks/receipts.db <mntbooks-legacy-export.sql
sqlite3 ../mntbooks.sqlite <mntbooks-legacy-import.sql

