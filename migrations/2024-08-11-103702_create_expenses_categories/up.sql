CREATE TABLE expenses_categories (
    expense_id INTEGER,
    category_id INTEGER,
    PRIMARY KEY (expense_id, category_id),
    FOREIGN KEY (expense_id) REFERENCES expenses(id),
    FOREIGN KEY (category_id) REFERENCES categories(id)
)