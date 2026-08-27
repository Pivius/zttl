use ratatui::{Frame, layout::Rect, widgets::{Block, Borders, Paragraph}};

use crate::app::{App, ViewMode};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
	let block = Block::default().title("Hierarchy").borders(Borders::ALL);
	let placeholder = Paragraph::new("").block(block);

	frame.render_widget(placeholder, area);
}