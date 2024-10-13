use std::fmt::Debug;

use ratatui::crossterm::event::Event;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum AppEvent {
    UiEvent(Event),
}
