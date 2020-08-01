.mode csv
delete from bookings;
delete from booking_docs;
delete from document_images;
.import mntbooks-legacy-export.csv bookings
.import mntbooks-legacy-export-bookingdocs.csv booking_docs
.import mntbooks-legacy-export-docimages.csv document_images
.quit
