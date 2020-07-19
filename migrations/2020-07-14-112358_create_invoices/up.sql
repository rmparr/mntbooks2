create table invoices (
  doc_id text not null primary key,
  kind text not null, -- invoice, quote, refund, reminder
  doc_date text not null,
  amount_cents integer not null,
  currency text not null,
  tax_code text not null,
  order_id text,
  payment_method text not null,
  line_items text not null,
  account text not null, -- was sales_account
  customer_account text not null,
  customer_company text,
  customer_name text not null,
  customer_address_1 text not null,
  customer_address_2 text,
  customer_zip text not null,
  customer_city  text not null,
  customer_state text,
  customer_country text not null,
  vat_included text not null,
  replaces_id text,
  replaced_by_id text,
  created_at text not null,
  updated_at text not null
);

create table documents (
  path text not null primary key,
  kind text not null, -- invoice, quote, refund, reminder, letter
  state text not null, -- todo, defer, ...?
  doc_id text not null,
  doc_date text not null,
  amount_cents integer not null,
  account text, -- was sales_account
  tags text not null,
  created_at text not null,
  updated_at text not null
);

create table bookings (
  id text not null primary key,
  booking_date text not null,
  amount_cents integer not null,
  details text not null,
  comment text,
  currency text not null,
  receipt_url text,
  tax_code text,
  debit_account text not null,
  credit_account text not null,
  txn_id text, -- transaction id in third-party system (bank, paypal)
  created_at text not null,
  updated_at text not null
);

create table booking_docs (
  id integer not null primary key,
  booking_id text,
  doc_id text
);
