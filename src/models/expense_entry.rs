use super::expense_price::ExpensePrice;

pub struct ExpenseEntry {
    // Label associated with expense
    label: String,
    // Price associated with expense
    value: ExpensePrice,
}