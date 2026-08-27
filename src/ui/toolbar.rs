use ratatui::{Frame, layout::Rect, widgets::Paragraph};

use crate::app::{App, ViewMode};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
	let mode_label = match app.mode {
		ViewMode::Ego => "MODE: EGO",
		ViewMode::Spatial => "MODE: SPATIAL"
	};

	let toolbar_text = format!(" {} | q quit", mode_label);
	let paragraph = Paragraph::new(toolbar_text);

	frame.render_widget(paragraph, area);
}