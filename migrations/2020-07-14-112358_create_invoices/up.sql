-- Your SQL goes here
CREATE TABLE invoices (
  id TEXT PRIMARY KEY NOT NULL,
  invoice_date TEXT NOT NULL,
  amount INTEGER NOT NULL,
  line_items TEXT NOT NULL
)

