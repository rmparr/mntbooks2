table! {
    book (id) {
        id -> Text,
        date -> Text,
        amount_cents -> Integer,
        details -> Text,
        comment -> Nullable<Text>,
        currency -> Text,
        receipt_url -> Nullable<Text>,
        tax_code -> Nullable<Text>,
        debit_account -> Text,
        credit_account -> Text,
        txn_id -> Nullable<Text>,
        created_at -> Text,
        updated_at -> Text,
    }
}

table! {
    documents (path) {
        path -> Text,
        state -> Text,
        docid -> Text,
        date -> Text,
        sum -> Integer,
        tags -> Text,
        created_at -> Text,
        updated_at -> Text,
    }
}

table! {
    invoices (invoice_id) {
        invoice_id -> Text,
        date -> Text,
        amount_cents -> Integer,
        currency -> Text,
        tax_code -> Text,
        order_id -> Nullable<Text>,
        payment_method -> Text,
        line_items -> Text,
        sales_account -> Text,
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
        replaces_id -> Text,
        replaced_by_id -> Nullable<Text>,
        created_at -> Nullable<Text>,
        updated_at -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    book,
    documents,
    invoices,
);
