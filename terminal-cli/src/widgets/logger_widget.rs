use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::style::Style;
use ratatui::widgets::Widget;
use tui_logger::TuiLoggerLevelOutput;
use tui_logger::TuiLoggerSmartWidget;

use crate::state::AppState;

pub fn render_widget(area: Rect, buffer: &mut ratatui::prelude::Buffer, state: &mut AppState) {
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
        .state(state.get_logger_state())
        .render(area, buffer);
}
