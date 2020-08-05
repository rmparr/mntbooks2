create table documents (
  id text not null primary key, -- UUID
  kind text not null, -- invoice, quote, refund, reminder
  doc_date text not null, -- date on document
  amount_cents integer,
  currency text,
  tax_code text,
  serial_id text, -- set by system
  order_id text,
  payment_method text,
  line_items text,
  customer_account text,
  customer_company text,
  customer_name text,
  customer_address_1 text,
  customer_address_2 text,
  customer_zip text,
  customer_city  text,
  customer_state text,
  customer_country text,
  vat_included text,
  replaces_id text,
  replaced_by_id text,
  created_at text not null,
  updated_at text not null,
  account text -- was sales_account
);

create table document_images (
  path text not null primary key, -- dirty incoming filename, unique
  pdf_path text not null, -- as pdf, converted if necessary; may be identical to .path
  mime_type text not null, -- incoming file type: pdf, png, html …
  doc_id text, -- UUID, unique
  extracted_text text not null, -- for full text search
  done boolean not null default false, -- true if processed; archive: false + no doc_id
  created_at text not null,
  updated_at text not null
);

create table bookings (
  id text not null primary key,
  booking_date text not null,
  amount_cents integer not null,
  details text not null,
  currency text not null,
  receipt_url text,
  tax_code text,
  debit_account text not null,
  credit_account text not null,
  txn_id text, -- transaction id in third-party system (bank, paypal)
  created_at text not null,
  updated_at text not null,
  comment text,
  done boolean
);

create table booking_docs (
  id integer not null primary key,
  booking_id text, -- FIXME should be not null
  doc_id text -- FIXME should be not null
);
