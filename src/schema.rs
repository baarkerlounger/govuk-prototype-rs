// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        uuid -> Varchar,
        name -> Varchar,
        age -> Int4,
        grade -> Int4,
        active -> Bool,
    }
}
