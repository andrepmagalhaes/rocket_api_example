// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Int4,
        user_id -> Int4,
        title -> Varchar,
        description -> Text,
        completed -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(todos -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    todos,
    users,
);
