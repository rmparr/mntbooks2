#!/bin/bash

# diesel_cli should already be installed according to README instructions,
# diesel_cli_ext on the other hand …
cargo install diesel_cli_ext

# this drops all tables!
# and generates src/schema.rs
diesel migration redo

# this generates src/models.rs
diesel_ext -d 'serde::Serialize, serde::Deserialize, Apiv2Schema, Clone, Queryable, Insertable, Identifiable, Debug, Default' -I 'crate::schema::*' > src/models.rs

