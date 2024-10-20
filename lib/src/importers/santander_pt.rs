use crate::database::entities::expenses::NewExpense;
use crate::importers::importer_definition::Importer;

#[derive(Default)]
pub struct ImporterSantanderPT {}

impl Importer for ImporterSantanderPT {
    fn get_name(&self) -> String {
        String::from("IMPORTER_SANTANDER_PT")
    }

    fn configure(&mut self) -> Result<(), ()> {
        todo!("Still to develop main functionality!");
    }

    fn import_expenses(&self) -> Vec<NewExpense> {
        todo!("Still to develop main functionality!");
    }
}
