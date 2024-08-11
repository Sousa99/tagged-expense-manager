// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Integer,
        created_at -> Nullable<Timestamp>,
        name -> Text,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    categories_hierarchy (rowid) {
        rowid -> Integer,
        category_id -> Nullable<Integer>,
        rgt -> Nullable<Integer>,
    }
}

diesel::table! {
    expenses (id) {
        id -> Integer,
        created_at -> Nullable<Timestamp>,
        title -> Text,
        description -> Nullable<Text>,
        timestamp -> Nullable<Timestamp>,
    }
}

diesel::table! {
    expenses_categories (expense_id, category_id) {
        expense_id -> Nullable<Integer>,
        category_id -> Nullable<Integer>,
    }
}

diesel::joinable!(expenses_categories -> categories (category_id));
diesel::joinable!(expenses_categories -> expenses (expense_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    categories_hierarchy,
    expenses,
    expenses_categories,
);
