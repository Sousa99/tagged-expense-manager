use ratatui::layout::Rect;
use ratatui::prelude::Buffer;
use ratatui::style::Modifier;
use ratatui::style::Style;
use ratatui::widgets::Block;
use ratatui::widgets::Borders;
use ratatui::widgets::List;
use ratatui::widgets::ListState;
use ratatui::widgets::StatefulWidget;

use tagged_expense_manager::services::expenses as tem_expenses;

use crate::state::AppState;

pub fn render_widget(area: Rect, buffer: &mut Buffer, state: &mut AppState) {
    let database_connection = state.get_database_connection();
    let expenses = tem_expenses::get_expenses(Some(database_connection));

    let expenses_identifiers: Vec<String> = expenses
        .unwrap_or_default()
        .iter()
        .map(|expense| format!("{}: {}", expense.id, expense.title))
        .collect();

    let mut list_state = ListState::default();
    List::new(expenses_identifiers)
        .block(Block::default().title("Expenses").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .render(area, buffer, &mut list_state);
}
