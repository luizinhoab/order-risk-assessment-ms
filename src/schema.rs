table! {
    assessment (id) {
        id -> Uuid,
        order_number -> Int8,
        customer_id -> Nullable<Uuid>,
        customer_name -> Text,
        customer_cpf -> Varchar,
        card_number -> Varchar,
        card_holder_name -> Varchar,
        creation_date_order -> Timestamp,
        value -> Float8,
        status -> Varchar,
        motivation -> Nullable<Text>,
        create_at -> Timestamp,
        update_at -> Nullable<Timestamp>,
    }
}
