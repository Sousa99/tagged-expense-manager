use crate::database::entities::expenses::NewExpense;

pub trait Importer {
    /// Used to get an identifier for the specific importer
    fn get_name(&self) -> String;

    /// Used to configure importer
    fn configure(&mut self) -> Result<(), ()>;

    /// Used to import a set of expenses
    fn import_expenses(&self) -> Vec<NewExpense>;
}
