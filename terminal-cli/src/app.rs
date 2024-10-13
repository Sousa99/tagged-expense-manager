use log::debug;
use ratatui::crossterm::event::Event;
use ratatui::crossterm::event::KeyCode;
use ratatui::crossterm::event::KeyEventKind;
use ratatui::layout::Constraint;
use ratatui::layout::Layout;
use ratatui::prelude::Backend;
use ratatui::style::*;
use ratatui::text::*;
use ratatui::widgets::*;
use ratatui::Terminal;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use tui_logger::TuiLoggerLevelOutput;
use tui_logger::TuiLoggerSmartWidget;
use tui_logger::TuiWidgetEvent;

use crate::event::AppEvent;
use crate::state::tab::get_identifier;
use crate::state::tab::Tab;
use crate::state::AppState;
use crate::utils::stream as utils_stream;
use crate::utils::terminal as utils_term;

pub struct App {
    state: AppState,
    tabs: Vec<&'static str>,
}

impl App {
    pub fn start(mut self, terminal: &mut Terminal<impl Backend>) -> anyhow::Result<()> {
        // Use an mpsc::channel to combine stdin events with app events
        let (tx, rx) = mpsc::channel();
        let event_tx = tx.clone();

        thread::spawn(move || utils_term::input_thread(event_tx));

        self.run(terminal, rx)
    }

    fn run(
        &mut self,
        terminal: &mut Terminal<impl Backend>,
        rx: mpsc::Receiver<AppEvent>,
    ) -> anyhow::Result<()> {
        // Debounce events
        let debounce_duration = Duration::from_millis(250);
        let debounced_rx = utils_stream::debouce_stream(rx, debounce_duration);

        for event in debounced_rx {
            // Handle application events
            match event {
                AppEvent::UiEvent(event) => self.handle_ui_event(event),
            }

            // Escape application if supposed to quit
            if !self.state.is_running() {
                break;
            }

            // Re-render application
            self.draw(terminal)?;
        }
        Ok(())
    }

    fn draw(&mut self, terminal: &mut Terminal<impl Backend>) -> anyhow::Result<()> {
        terminal.draw(|frame| {
            frame.render_widget(self, frame.area());
        })?;
        Ok(())
    }

    fn handle_ui_event(&mut self, event: Event) {
        debug!(target: "App", "Handling UI event: {:?}",event);
        let state = self.state.get_logger_state();

        if let Event::Key(key) = event {
            let code = key.code;
            let kind = key.kind;

            if matches!(kind, KeyEventKind::Release) {
                return;
            }

            match code {
                KeyCode::Char('q') => self.state.stop_running(),
                KeyCode::Char(' ') => state.transition(TuiWidgetEvent::SpaceKey),
                KeyCode::Esc => state.transition(TuiWidgetEvent::EscapeKey),
                KeyCode::PageUp => state.transition(TuiWidgetEvent::PrevPageKey),
                KeyCode::PageDown => state.transition(TuiWidgetEvent::NextPageKey),
                KeyCode::Up => state.transition(TuiWidgetEvent::UpKey),
                KeyCode::Down => state.transition(TuiWidgetEvent::DownKey),
                KeyCode::Left => state.transition(TuiWidgetEvent::LeftKey),
                KeyCode::Right => state.transition(TuiWidgetEvent::RightKey),
                KeyCode::Char('+') => state.transition(TuiWidgetEvent::PlusKey),
                KeyCode::Char('-') => state.transition(TuiWidgetEvent::MinusKey),
                KeyCode::Char('h') => state.transition(TuiWidgetEvent::HideKey),
                KeyCode::Char('f') => state.transition(TuiWidgetEvent::FocusKey),
                _ => (),
            }
        }
    }
}

impl Default for App {
    fn default() -> Self {
        // Create app state
        let state = AppState::new();

        App {
            state,
            tabs: vec![
                get_identifier(Tab::ShowExpenses),
                get_identifier(Tab::ShowCategories),
            ],
        }
    }
}

// Implement Ratatui's Widget for App
impl Widget for &mut App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let [tabs_area, main_area, log_area, help_area] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Fill(50),
            Constraint::Fill(30),
            Constraint::Length(3),
        ])
        .areas(area);

        // show two TuiWidgetState side-by-side
        let [_left, _right] = Layout::horizontal([Constraint::Fill(1); 2]).areas(main_area);

        Tabs::new(self.tabs.iter().cloned())
            .block(Block::default().title("States").borders(Borders::ALL))
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
            .select(self.state.get_tab_index())
            .render(tabs_area, buf);

        TuiLoggerSmartWidget::default()
            .style_error(Style::default().fg(Color::Red))
            .style_debug(Style::default().fg(Color::Green))
            .style_warn(Style::default().fg(Color::Yellow))
            .style_trace(Style::default().fg(Color::Magenta))
            .style_info(Style::default().fg(Color::Cyan))
            .output_separator(':')
            .output_timestamp(Some("%H:%M:%S".to_string()))
            .output_level(Some(TuiLoggerLevelOutput::Abbreviated))
            .output_target(true)
            .output_file(true)
            .output_line(true)
            .state(self.state.get_logger_state())
            .render(log_area, buf);

        if area.width > 40 {
            Text::from(vec![
                "Q: Quit | Tab: Switch state | ↑/↓: Select target | f: Focus target".into(),
                "←/→: Display level | +/-: Filter level | Space: Toggle hidden targets".into(),
                "h: Hide target selector | PageUp/Down: Scroll | Esc: Cancel scroll".into(),
            ])
            .style(Color::Gray)
            .centered()
            .render(help_area, buf);
        }
    }
}
