use ratatui::layout::Rect;
use ratatui::style::Modifier;
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::Block;
use ratatui::widgets::Borders;
use ratatui::widgets::Tabs;
use ratatui::widgets::Widget;

use crate::state::AppState;

pub fn render_widget<'a, T>(
    area: Rect,
    buffer: &mut ratatui::prelude::Buffer,
    state: &mut AppState,
    tabs: T,
) where
    T: IntoIterator,
    T::Item: Into<Line<'a>>,
{
    Tabs::new(tabs)
        .block(Block::default().title("States").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .select(state.get_tab_index())
        .render(area, buffer);
}
