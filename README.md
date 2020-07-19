# MNTBooks 2

Cleanup and rewrite of MNTBooks in Rust.

## Getting Started

```
cp mntconfig.toml.default mntconfig.toml

cargo install diesel_cli
export DATABASE_URL=./mntbooks.sqlite
diesel migration up

cargo build
./target/debug/mntbooks
```

## Technologies

- Actix (web/actor framework)
- Tera (templates)
- Diesel (SQL abstraction)
- r2d2 (DB connection pool)
- SQLite (but can also be Postgres or anything supported by Diesel)
