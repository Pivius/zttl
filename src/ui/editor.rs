use ratatui::{Frame, layout::Rect, widgets::{Block, Borders, Paragraph}};

use crate::app::App;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
	let content = if let Some(focused) = app.focused_node() {
		&app.index.graph[focused].body
	} else {
		"No note open"
	};

	let block = Block::default().title("Editor").borders(Borders::ALL);
	let paragraph = Paragraph::new(content).block(block);

	frame.render_widget(paragraph, area);
}