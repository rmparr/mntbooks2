-- Your SQL goes here
create table invoices (
  invoice_id varchar(64) not null primary key,
  date text not null,
  amount_cents int not null,
  currency text not null,
  tax_code text not null,
  order_id text,
  payment_method text not null,
  line_items text not null,
  sales_account text not null,
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
  replaces_id text not null,
  replaced_by_id text,
  created_at text,
  updated_at text
);

create table documents (
  path text not null primary key,
  state text not null,
  docid text not null,
  date text not null,
  sum integer not null,
  tags text not null,
  created_at text not null,
  updated_at text not null
);

create table book (
  id text not null primary key,
  date text not null,
  amount_cents integer not null,
  details text not null,
  comment text,
  currency text not null,
  receipt_url text,
  tax_code text,
  debit_account text not null,
  credit_account text not null,
  txn_id text,
  created_at text not null,
  updated_at text not null
);
