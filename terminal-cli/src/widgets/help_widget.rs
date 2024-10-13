use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::text::Text;
use ratatui::widgets::Widget;

pub fn render_widget(area: Rect, buffer: &mut ratatui::prelude::Buffer) {
    Text::from(vec![
        "Q: Quit | Tab: Switch state | ↑/↓: Select target | f: Focus target".into(),
        "←/→: Display level | +/-: Filter level | Space: Toggle hidden targets".into(),
        "h: Hide target selector | PageUp/Down: Scroll | Esc: Cancel scroll".into(),
    ])
    .style(Color::Gray)
    .centered()
    .render(area, buffer);
}
