use std::vec;

use ratatui::{Frame, layout::{Constraint, Layout, Rect}, style::{Style, Stylize}, text::Line, widgets::{Block, Borders, List, Paragraph}};

use crate::app::{App, RefKind, ViewMode};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
	match app.mode {
		ViewMode::Ego => render_ego(frame, area, app),
		ViewMode::Spatial => render_spatial(frame, area, app)
	}
}

fn render_ego (frame: &mut Frame, area: Rect, app: &App) {
	let block = Block::default()
		.title("Hierarchy")
		.borders(Borders::ALL)
		.border_style(app.theme.border);
	let inner = block.inner(area);

	let entries = app.ego_visible();

	if entries.is_empty() {
		frame.render_widget(
			Paragraph::new("no note open").fg(app.theme.muted_text), 
			inner
		);
		return;
	}

	let lines: Vec<Line> = entries
		.iter()
		.enumerate()
		.map(|(i, e)| {
			let note = &app.index.graph[e.node];
			let is_expanded = app.expanded.contains(&e.node);

			let indent = match e.depth {
				0 => String::new(),
				1 => "  ".to_string(),
				d => format!("{}├─", "  ".repeat(d - 1))
			};

			let fold_icon = if e.depth == 0 {
				"● "
			} else if is_expanded {
				"▼ "
			} else {
				"▶ "
			};

			let (badge, color) = match e.category {
				None => ("", app.theme.text),
				Some(RefKind::Conceptual) => ("-> ", app.theme.accent),
				Some(RefKind::Structural) => ("=> ", app.theme.success),
				Some(RefKind::Backlink) => ("<- ", app.theme.warning),
				Some(RefKind::StructuralBacklink) => ("<= ", app.theme.info)
			};

			let style = if i == app.ego_focus {
				Style::default().bg(app.theme.selection).fg(app.theme.text)
			} else {
				Style::default().fg(app.theme.text)
			};

			Line::styled(
				format!("{}{}{}{}", indent, fold_icon, badge, note.title()),
				style
			)
		})
		.collect();

	frame.render_widget(List::new(lines), inner);
}

fn render_spatial(frame: &mut Frame, area: Rect, app: &App) {
	let n = app.columns.len();
	let areas = Layout::horizontal(vec![Constraint::Ratio(1, n as u32); n])
		.split(area);

	for (i, col) in app.columns.iter().enumerate() {
		let is_last = i == n - 1;
		let border = if is_last { app.theme.focused_border } else { app.theme.border };
		let block = Block::default()
			.title(column_header(app, i))
			.borders(Borders::ALL)
			.border_style(border);
		let inner = block.inner(areas[i]);
		
		frame.render_widget(block, areas[i]);

		let lines: Vec<Line> = col.items
			.iter()
			.enumerate()
			.map(|(j, &node)| {
				let title = app.index.graph[node].title();
				let style = if j == col.focus {
					Style::default().bg(app.theme.selection).fg(app.theme.text)
				} else {
					Style::default().fg(app.theme.text)
				};
				Line::styled(title, style)
			})
			.collect();

		frame.render_widget(List::new(lines), inner);
	}
}

fn column_header(app: &App, i: usize) -> String {
	if i == 0 { "roots".to_string() }
	else {
		let prev = &app.columns[i - 1];

		app.index.graph[prev.items[prev.focus]].title().to_string()
	}
}