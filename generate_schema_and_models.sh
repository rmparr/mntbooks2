#!/bin/bash

# this drops all tables!
# and generates src/schema.rs
diesel redo

# this generates src/models.rs
diesel_ext -d 'serde::Serialize, Queryable, Insertable, Identifiable' -I 'crate::schema::*' > src/models.rs

