pub enum Tab {
    ShowExpenses,
    ShowCategories,
}

pub fn get_identifier(tab: Tab) -> &'static str {
    match tab {
        Tab::ShowExpenses => "Show Expenses",
        Tab::ShowCategories => "Show Categories",
    }
}
