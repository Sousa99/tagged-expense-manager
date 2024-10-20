mod importer_definition;
pub mod santander_pt;

use clap_derive::ValueEnum;
use importer_definition::Importer as ImporterTrait;
use santander_pt::ImporterSantanderPT;

#[derive(Clone, ValueEnum)]
pub enum Importer {
    SantanderPT,
}

impl Importer {
    pub fn get_importer(self) -> Box<dyn ImporterTrait> {
        match self {
            Importer::SantanderPT => Box::new(ImporterSantanderPT::default()),
        }
    }
}
