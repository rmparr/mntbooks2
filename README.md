# MNTBooks 2

Cleanup and rewrite of MNTBooks in Rust.

MNTBooks is a minimalist ERP (enterprise resource planning) system for smol manufacturing companies.

## Getting Started

### Build Dependencies

```bash
sudo apt install libsqlite3-dev libpq-dev default-libmysqlclient-dev
```

### Runtime Dependencies (external tools)

- `wkhtmltopdf` to generate PDF files for Documents
- `pdfunite` from `poppler-utils` if you want to use the DATEV export

```bash
sudo apt install wkhtmltopdf poppler-utils
```

### Configuration / Setup

```bash
# copy the default config and edit to your liking
cp mntconfig.toml.default mntconfig.toml

# this is only needed once to initialize the DB
cargo install diesel_cli
# initially and whenever the schema changes
export DATABASE_URL=./mntbooks.sqlite
diesel migration run

# build the code
cargo build
# run the webserver (there are also other tooling binaries)
cargo run --bin mntbooks
```

## Usage

There's currently no `/` route. Some useful endpoints:

- `/documents` Documents with metadata like invoices, quotes, refunds
- `/documentimages` Each Document can have 1 or more DocumentImages (currently PDFs)
- `/bookings` The book/ledger
- `/api-spec` OpenAPI JSON spec of the JSON API endpoints
- `/api-docs` ReDoc HTML frontend for the API docs

## Workflow

### Bank/Paypal transactions

The `scripts` folder contains two examples for importing bookings from external transactions, in this case FinTS bank accounts and PayPal. You can run tasks like these regularly to sync your asset movements into mntbooks2.

### Docstore

If you have a document scanner and/or a process to store incoming PDF receipts, you should collect them in a central folder called the Docstore. Mntbooks2 has a CLI task that finds new files added to the folder and registers them as DocumentImages. You can then create a Document in the web interface for each DocumentImage to register its metadata like date, invoice number, amount etc.

To regularly scan your incoming PDF folder for new DocumentImages, set up a cronjob like:
```crontab
* * * * *  cd /home/mntbooks/mntbooks2 && ./target/debug/update_docimgs_from_docstore
```

### Link Documents to Bookings

No booking without a receipt: each Booking has an Edit link that allows you to add/change the debit and credit accounts for this Booking as well as link any number of Documents to the booking. You can set the Booking's `done` flag `true` if you think your Booking is complete.

### Creating Outbound Documents

You can create your own Documents like quotes, invoices and refunds manually by using the built-in web form or automatically by using the `/documents.json` API. For example, MNT Research uses the Solidus shop system and a rake task that creates an invoice in mntbooks2 for each order via the API.

### DATEV Export

At the bottom of each `/bookings` page there's a link to a `/bookings-datev` page with the same query. This allows you to create "DATEV Buchungsstapel" folders containing a DATEV CSV file and the matching receipts (Documents). Tax advisors in Germany can import a folder like this into DATEV via "Beleg2Buchung".

This is more useful if you have set up a mapping of your account names to SKR ("Standardkontenrahmen") in `mntconfig.toml`.

## Technologies

- Rust
- Actix (web/actor framework)
- Tera (templating)
- Diesel (SQL abstraction)
- r2d2 (DB connection pool)
- SQLite
- Paperclip (automatic API documentation)
