use ratatui::style::Color;

use crate::ui::colors::{ColorSupport, try_color};

#[derive(Debug, Clone, Copy)]
pub enum Gradient {
	Gradient0,
	Gradient1,
	Gradient2,
	Gradient3,
	Gradient4,
	Gradient5,
	Gradient6,
	Gradient7,
	Gradient8,
	Gradient9,
	Gradient10,
	Gradient11,
	Gradient12,
	Gradient13,
	Gradient14,
}

impl Gradient {
	pub fn rgb(self) -> (u8, u8, u8) {
		match self {
			Self::Gradient0 => (46, 41, 48),   // #2E2930
			Self::Gradient1 => (60, 50, 69),   // #3C3245
			Self::Gradient2 => (72, 60, 93),   // #483C5D
			Self::Gradient3 => (82, 71, 118),  // #524776
			Self::Gradient4 => (88, 83, 146),  // #585392
			Self::Gradient5 => (90, 96, 174),  // #5A60AE
			Self::Gradient6 => (87, 111, 202), // #576FCA
			Self::Gradient7 => (77, 128, 230), // #4D80E6
			Self::Gradient8 => (88, 117, 210), // #5875D2
			Self::Gradient9 => (94, 107, 190), // #5E6BBE
			Self::Gradient10 => (96, 98, 169), // #6062A9
			Self::Gradient11 => (95, 90, 149), // #5F5A95
			Self::Gradient12 => (92, 82, 129), // #5C5281
			Self::Gradient13 => (86, 75, 110), // #564B6E
			Self::Gradient14 => (79, 69, 92),  // #4F455C
		}
	}
}

pub struct Theme {
	pub background: Color,
	pub foreground: Color,
	pub text: Color,
	pub muted_text: Color,
	pub border: Color,
	pub focused_border: Color,
	pub selection: Color,
	pub cursor: Color,
	pub accent: Color,
	pub error: Color,
	pub warning: Color,
	pub success: Color,
	pub info: Color,
}

impl Theme {
	pub fn new(support: ColorSupport) -> Self {
		Self {
			background: try_color(Gradient::Gradient0.rgb(), &support),
			foreground: try_color(Gradient::Gradient14.rgb(), &support),
			text: Color::White,
			muted_text: Color::Gray,
			border: try_color(Gradient::Gradient2.rgb(), &support),
			focused_border: try_color(Gradient::Gradient7.rgb(), &support),
			selection: try_color(Gradient::Gradient3.rgb(), &support),
			cursor: Color::White,
			accent: try_color(Gradient::Gradient7.rgb(), &support),
			error: Color::Red,
			warning: Color::Yellow,
			success: try_color(Gradient::Gradient9.rgb(), &support),
			info: try_color(Gradient::Gradient6.rgb(), &support),
		}
	}
}