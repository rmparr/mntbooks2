#!/bin/bash

# diesel_cli should already be installed according to README instructions,
# diesel_cli_ext on the other hand …
cargo install diesel_cli_ext

# this drops all tables!
# and generates src/schema.rs
diesel migration redo

# this generates src/models.rs
diesel_ext -d 'serde::Serialize, serde::Deserialize, Clone, Queryable, Insertable, Identifiable, Debug' -I 'crate::schema::*' > src/models.rs

