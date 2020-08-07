update book set credit_txn_id=debit_txn_id where credit_txn_id is null;

.headers off
.mode csv
.output mntbooks-legacy-export.csv

select id, date as booking_date, amount_cents, details, currency, receipt_url, tax_code, debit_account, credit_account, credit_txn_id as txn_id, created_at, updated_at, comment, 1 from book;

.output mntbooks-legacy-export-bookingdocs.csv
select book.rowid, id, docid from book, documents where path=receipt_url and docid is NOT NULL;

.output mntbooks-legacy-export-docimages.csv
select path,path,"application/pdf",docid,tags,1,date,date from documents where docid is NOT NULL and docid is NOT "" and state="defer";

.output mntbooks-legacy-export-documents.csv
select id,"invoice",invoice_date,amount_cents,currency,tax_code,id,order_id,payment_method,line_items,customer_account,customer_company,customer_name,customer_address_1,customer_address_2,customer_zip,customer_city,customer_state,customer_country,vat_included,replaces_id,replaced_by_id,created_at,updated_at,sales_account from invoices;

select docid,"receipt",date,sum*100,"UNK",NULL,docid,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,created_at,updated_at,NULL from documents where docid is NOT NULL and docid is NOT "" and state="defer";

.quit
