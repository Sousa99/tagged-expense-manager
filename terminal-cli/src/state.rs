use log::LevelFilter;
use tui_logger::TuiWidgetState;

pub mod tab;

pub struct AppState {
    running: bool,
    tab_index: usize,
    logger_state_index: usize,
    logger_states: Vec<TuiWidgetState>,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            running: true,
            tab_index: 0,
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

    pub fn get_logger_state(&mut self) -> &mut TuiWidgetState {
        &mut self.logger_states[self.logger_state_index]
    }
}
