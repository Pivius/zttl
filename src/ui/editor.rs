use std::vec;

use ratatui::{Frame, layout::Rect, text::Line, widgets::{Block, Borders, Paragraph, Wrap}};

use crate::{app::App, ui::markdown::render_markdown};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
	let content = if let Some(focused) = app.focused_node() {
		render_markdown(&app.index.graph[focused].body, app)
	} else {
		vec![Line::default().spans(vec!["No active note selected"])]
	};

	let block = Block::default().title("Editor").borders(Borders::ALL);
	let paragraph = Paragraph::new(content)
		.block(block)
		.wrap(Wrap { trim: false });

	frame.render_widget(paragraph, area);
}