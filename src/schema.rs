table! {
    booking_docs (id) {
        id -> Integer,
        booking_id -> Text,
        doc_id -> Text,
    }
}

table! {
    bookings (id) {
        id -> Text,
        booking_date -> Text,
        amount_cents -> Integer,
        details -> Text,
        currency -> Text,
        tax_code -> Text,
        debit_account -> Text,
        credit_account -> Text,
        txn_id -> Text,
        created_at -> Text,
        updated_at -> Text,
        comment -> Text,
        done -> Bool,
    }
}

table! {
    document_images (path) {
        path -> Text,
        pdf_path -> Text,
        mime_type -> Text,
        doc_id -> Nullable<Text>,
        extracted_text -> Text,
        done -> Bool,
        created_at -> Text,
        updated_at -> Text,
    }
}

table! {
    documents (id) {
        id -> Text,
        kind -> Text,
        doc_date -> Text,
        amount_cents -> Nullable<Integer>,
        currency -> Nullable<Text>,
        tax_code -> Nullable<Text>,
        serial_id -> Nullable<Text>,
        foreign_serial_id -> Nullable<Text>,
        order_id -> Nullable<Text>,
        payment_method -> Nullable<Text>,
        line_items -> Nullable<Text>,
        customer_account -> Nullable<Text>,
        customer_company -> Nullable<Text>,
        customer_name -> Nullable<Text>,
        customer_address_1 -> Nullable<Text>,
        customer_address_2 -> Nullable<Text>,
        customer_zip -> Nullable<Text>,
        customer_city -> Nullable<Text>,
        customer_state -> Nullable<Text>,
        customer_country -> Nullable<Text>,
        vat_included -> Nullable<Text>,
        replaces_id -> Nullable<Text>,
        replaced_by_id -> Nullable<Text>,
        created_at -> Text,
        updated_at -> Text,
        account -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    booking_docs,
    bookings,
    document_images,
    documents,
);
