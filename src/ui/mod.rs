use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, layout::{Constraint, Direction, Layout}, style::Stylize, widgets::Block};

use crate::app::App;

pub mod colors;
pub mod theme;
mod editor;
mod nav;
mod toolbar;

pub fn draw(frame: &mut Frame, app: &App) {
	let main_chunks = Layout::default()
		.direction(Direction::Vertical)
		.constraints([Constraint::Min(0), Constraint::Length(1)])
		.split(frame.area());

	let nav_chunks = Layout::default()
		.direction(Direction::Horizontal)
		.constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
		.split(main_chunks[0]);
	
	frame.render_widget(Block::new().bg(app.theme.background), frame.area());

	nav::render(frame, nav_chunks[0], app);
	editor::render(frame, nav_chunks[1], app);
	toolbar::render(frame, main_chunks[1], app);
}

pub fn handle_key(app: &mut App, key: KeyEvent) {
	match key.code {
		KeyCode::Char('q') | KeyCode::Esc => app.quit(),
		_ => {}
	}
}