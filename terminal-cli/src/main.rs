// Loading of Modules
mod app;
mod event;
mod state;
mod utils;
mod widgets;
// Import of Local Modules
use crate::app::App;
use utils::terminal as utils_term;

// Imports of External Crates
use dotenvy::dotenv;
use tagged_expense_manager::database::utils as db_utils;

use log::*;
use tui_logger::*;

fn main() -> anyhow::Result<()> {
    // Load Environment Variables
    dotenv().expect("🚫 .env file could not found");
    // Initialize Logger
    init_logger(LevelFilter::Trace)?;

    log::info!("Starting up terminal command line interface 🚀");

    log::debug!("Attempting to establish connection with database");
    let mut _database_connection = db_utils::connection::establish_connection();
    log::debug!("Connection with database established");

    set_default_level(LevelFilter::Trace);
    debug!(target:"App", "Logging initialized");

    let mut terminal = utils_term::init_terminal()?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    let app = App::default();
    app.start(&mut terminal)?;

    utils_term::restore_terminal()?;
    terminal.clear()?;

    Ok(())
}
