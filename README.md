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

## Technologies

- Rust
- Actix (web/actor framework)
- Tera (templating)
- Diesel (SQL abstraction)
- r2d2 (DB connection pool)
- SQLite
- Paperclip (automatic API documentation)
