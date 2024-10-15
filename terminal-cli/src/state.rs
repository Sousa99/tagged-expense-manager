use log::LevelFilter;
use tui_logger::TuiWidgetState;

use tagged_expense_manager::database::utils as db_utils;

pub mod tab;

pub struct AppState {
    running: bool,
    tab_index: usize,
    database_connection: db_utils::connection::SqliteConnection,
    logger_state_index: usize,
    logger_states: Vec<TuiWidgetState>,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            running: true,
            tab_index: 0,
            database_connection: establish_database_connection(),
            logger_state_index: 0,
            logger_states: vec![
                TuiWidgetState::new().set_default_display_level(LevelFilter::Info),
                TuiWidgetState::new().set_default_display_level(LevelFilter::Info),
                TuiWidgetState::new().set_default_display_level(LevelFilter::Info),
                TuiWidgetState::new().set_default_display_level(LevelFilter::Info),
            ],
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn stop_running(&mut self) {
        self.running = false
    }

    pub fn get_tab_index(&self) -> usize {
        self.tab_index
    }

    pub fn get_database_connection(&mut self) -> &mut db_utils::connection::SqliteConnection {
        &mut self.database_connection
    }

    pub fn get_logger_state(&mut self) -> &mut TuiWidgetState {
        &mut self.logger_states[self.logger_state_index]
    }
}

fn establish_database_connection() -> db_utils::connection::SqliteConnection {
    log::debug!("Attempting to establish connection with database");
    let database_connection = db_utils::connection::establish_connection();
    log::debug!("Connection with database established");

    database_connection
}
