update book set credit_txn_id=debit_txn_id where credit_txn_id is null;

.headers off
.mode csv
.output mntbooks-legacy-export.csv

-- TODO: replace old hashed ids with uuids after import

select id, substr(date,1,10) as booking_date, amount_cents, details, currency, tax_code, debit_account, credit_account, credit_txn_id as txn_id, created_at, updated_at, comment, 1 from book;

.output mntbooks-legacy-export-bookingdocs.csv
create table temp (
  booking_id text not null,
  doc_id text not null
);
insert into temp select id, path from book, documents where receipt_url like "%"||path||"%" and docid is NOT NULL;
select rowid,* from temp;
drop table temp;

.output mntbooks-legacy-export-docimages.csv
select path,path,"application/pdf",path,"",1,date,date from documents where docid is NOT NULL and docid is NOT "" and state="defer";

.output mntbooks-legacy-export-documents.csv
select id,"invoice",invoice_date,amount_cents,currency,tax_code,id,NULL,order_id,payment_method,line_items,customer_account,customer_company,customer_name,customer_address_1,customer_address_2,customer_zip,customer_city,customer_state,customer_country,vat_included,replaces_id,replaced_by_id,created_at,updated_at,sales_account from invoices;

-- TODO: correct currency after the fact
select path,"receipt",date,sum*100,"EUR",NULL,NULL,docid,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,created_at,updated_at,tags from documents where docid is NOT NULL and docid is NOT "" and state="defer";

-- clean up and re-link invoices
update booking_docs set doc_id = (select documents.id from documents where booking_docs.doc_id like "invoice-%-"||documents.id||".pdf") where doc_id like "invoice-%";
update document_images set doc_id = (select documents.id from documents where document_images.doc_id like "invoice-%-"||documents.id||".pdf") where doc_id like "invoice-%";
delete from documents where id like "invoice-%" and kind="receipt";

.quit
