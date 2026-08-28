use ratatui::{Frame, layout::{Constraint, Direction, Layout, Rect}, style::{Color, Modifier, Style, Stylize}, text::{Line, Span}, widgets::{Block, Paragraph}};
use std::{format, vec};
use crate::{app::{App, ViewMode}, ui::theme::Palette};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
	let chunks = Layout::default()
		.direction(Direction::Horizontal)
		.constraints([
			Constraint::Min(30), 
			Constraint::Percentage(60),
			Constraint::Length(12)
		])
		.split(area);

	let mode_label = match app.mode {
		ViewMode::Ego => "EGO",
		ViewMode::Spatial => "SPATIAL"
	};

	let active_title = app
		.focused_node()
		.map(|idx| app.index.graph[idx].title())
		.unwrap_or("None");

	let status_line = Line::from(vec![
		Span::styled(
			mode_label,
			Style::default().bg(app.theme.foreground).fg(app.theme.text).add_modifier(Modifier::BOLD)
		),
		Span::raw(format!(" {}", active_title))
	]);

	let hints_str = match app.mode {
		ViewMode::Ego => "j/k: nav | h/l: fold | Enter: root | /:jump | Tab: mode",
		ViewMode::Spatial => "j/k: nav | h/l: cols | Enter: open | /: jump | Tab: mode"
	};

	let hints_line = Line::from(Span::styled(hints_str, Style::default().fg(app.theme.muted_text)));

	let global_line = Line::from(vec![
		Span::styled("q", Style::default().fg(app.theme.warning).add_modifier(Modifier::BOLD)),
		Span::raw(": quit"),
	]);

	frame.render_widget(Paragraph::new(status_line), chunks[0]);
	frame.render_widget(Paragraph::new(hints_line), chunks[1]);
	frame.render_widget(Paragraph::new(global_line), chunks[2]);
}