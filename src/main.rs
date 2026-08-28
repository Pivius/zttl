mod graph;
mod model;
mod parser;
mod scanner;
mod sys;
mod app;
mod ui;

use std::{error::Error, io::stdout, panic, path::PathBuf};

use app::App;
use crossterm::{event::{self, Event, KeyEventKind}, execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode}};
use ratatui::{Terminal, backend::CrosstermBackend};
use scanner::VaultScanner;

fn restore_terminal() {
	let _ = disable_raw_mode();
	let _ = execute!(stdout(), LeaveAlternateScreen);
}

fn main() -> Result<(), Box<dyn Error>> {
	let original_hook = panic::take_hook();

	panic::set_hook(Box::new(move |panic_info| {
		restore_terminal();
		original_hook(panic_info);
	}));

	let vault_path = PathBuf::from("./vault");
	let index = VaultScanner::scan(&vault_path)?;
	let mut app = App::new(index);

	enable_raw_mode()?;

	let mut stdout_handle = stdout();

	execute!(stdout_handle, EnterAlternateScreen)?;

	let backend = CrosstermBackend::new(stdout_handle);
	let mut terminal = Terminal::new(backend)?;

	while app.running {
		terminal.draw(|f| ui::draw(f, &app))?;

		if let Event::Key(key) = event::read()? {
			if key.kind == KeyEventKind::Press {
				ui::handle_key(&mut app, key);
			}
		}
	}

	restore_terminal();
	Ok(())
}