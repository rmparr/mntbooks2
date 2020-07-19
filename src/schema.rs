table! {
    booking_docs (id) {
        id -> Integer,
        booking_id -> Nullable<Text>,
        doc_id -> Nullable<Text>,
    }
}

table! {
    bookings (id) {
        id -> Text,
        booking_date -> Text,
        amount_cents -> Integer,
        details -> Text,
        currency -> Text,
        receipt_url -> Nullable<Text>,
        tax_code -> Nullable<Text>,
        debit_account -> Text,
        credit_account -> Text,
        txn_id -> Nullable<Text>,
        created_at -> Text,
        updated_at -> Text,
        comment -> Nullable<Text>,
    }
}

table! {
    documents (path) {
        path -> Text,
        kind -> Text,
        state -> Text,
        doc_id -> Text,
        doc_date -> Text,
        amount_cents -> Integer,
        account -> Nullable<Text>,
        tags -> Text,
        created_at -> Text,
        updated_at -> Text,
    }
}

table! {
    invoices (doc_id) {
        doc_id -> Text,
        kind -> Text,
        doc_date -> Text,
        amount_cents -> Integer,
        currency -> Text,
        tax_code -> Text,
        order_id -> Nullable<Text>,
        payment_method -> Text,
        line_items -> Text,
        account -> Text,
        customer_account -> Text,
        customer_company -> Nullable<Text>,
        customer_name -> Text,
        customer_address_1 -> Text,
        customer_address_2 -> Nullable<Text>,
        customer_zip -> Text,
        customer_city -> Text,
        customer_state -> Nullable<Text>,
        customer_country -> Text,
        vat_included -> Text,
        replaces_id -> Nullable<Text>,
        replaced_by_id -> Nullable<Text>,
        created_at -> Text,
        updated_at -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    booking_docs,
    bookings,
    documents,
    invoices,
);
