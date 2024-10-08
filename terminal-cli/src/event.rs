use ratatui::crossterm::event::Event;

pub enum AppEvent {
    UiEvent(Event),
}
