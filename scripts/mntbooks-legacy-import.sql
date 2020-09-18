.mode csv
delete from bookings;
delete from booking_docs;
delete from document_images;
delete from documents;
.import mntbooks-legacy-export.csv bookings
.import mntbooks-legacy-export-bookingdocs.csv booking_docs
.import mntbooks-legacy-export-docimages.csv document_images
.import mntbooks-legacy-export-documents.csv documents

-- clean up and re-link invoices
update booking_docs set doc_id = (select documents.id from documents where booking_docs.doc_id like "invoice-%-"||documents.id||".pdf") where doc_id like "invoice-%";
update document_images set doc_id = (select documents.id from documents where document_images.doc_id like "invoice-%-"||documents.id||".pdf") where doc_id like "invoice-%";
delete from documents where id like "invoice-%" and kind="receipt";

.quit
