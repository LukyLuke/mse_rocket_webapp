// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Nullable<Text>,
        surname -> Nullable<Text>,
        email -> Text,
        password -> Text,
    }
}
