use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, layout::{Constraint, Direction, Layout}, style::Stylize, widgets::Block};

use crate::app::{App, ViewMode};

pub mod colors;
pub mod theme;
mod editor;
mod nav;
mod toolbar;

fn nav_editor_split(app: &App) -> (Constraint, Constraint) {
	match app.mode {
		ViewMode::Ego => (Constraint::Percentage(30), Constraint::Percentage(70)),
		ViewMode::Spatial => match app.columns.len() {
			1 => (Constraint::Percentage(30), Constraint::Percentage(70)),
			2 => (Constraint::Percentage(60), Constraint::Percentage(40)),
			_ => (Constraint::Min(0), Constraint::Length(0))
		}
	}
}

pub fn draw(frame: &mut Frame, app: &App) {
	let main_chunks = Layout::default()
		.direction(Direction::Vertical)
		.constraints([Constraint::Min(0), Constraint::Length(1)])
		.split(frame.area());
	let (nav_chunk, editor_chunk) = nav_editor_split(app);
	let body = Layout::default()
		.direction(Direction::Horizontal)
		.constraints([nav_chunk, editor_chunk])
		.split(main_chunks[0]);
	
	frame.render_widget(Block::new().bg(app.theme.background), frame.area());

	nav::render(frame, body[0], app);
	editor::render(frame, body[1], app);
	toolbar::render(frame, main_chunks[1], app);
}

pub fn handle_key(app: &mut App, key: KeyEvent) {
	match key.code {
		KeyCode::Char('q') | KeyCode::Esc => app.quit(),
		KeyCode::Tab => app.toggle_mode(),
		KeyCode::Char('j') => app.move_focus(1),
		KeyCode::Char('k') => app.move_focus(-1),
		KeyCode::Char('l') => app.descend(),
		KeyCode::Char('h') => app.ascend(),
		KeyCode::Enter => app.open_focused(),
		_ => {}
	}
}