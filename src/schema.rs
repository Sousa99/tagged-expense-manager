// @generated automatically by Diesel CLI.

diesel::table! {
    expenses (id) {
        id -> Nullable<Integer>,
        title -> Text,
        description -> Nullable<Text>,
    }
}
