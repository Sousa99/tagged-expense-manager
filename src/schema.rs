// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Integer,
        created_at -> Timestamp,
        name -> Text,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    categories_hierarchy (rowid) {
        rowid -> Integer,
        category_id -> Integer,
        lft -> Integer,
        rgt -> Integer,
    }
}

diesel::table! {
    expenses (id) {
        id -> Integer,
        created_at -> Timestamp,
        title -> Text,
        description -> Nullable<Text>,
        timestamp -> Timestamp,
        value_decimal -> Integer,
    }
}

diesel::table! {
    expenses_categories (expense_id, category_id) {
        expense_id -> Integer,
        category_id -> Integer,
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
