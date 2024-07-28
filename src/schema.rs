// @generated automatically by Diesel CLI.

diesel::table! {
    expenses (id) {
        id -> Integer,
        title -> Text,
        description -> Nullable<Text>,
    }
}
