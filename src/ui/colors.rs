use crossterm::style::available_color_count;
use crossterm::style::Color;
use ratatui::backend::FromCrossterm;

#[derive(Debug, Clone)]
pub enum ColorSupport {
	TrueColor,
	Ansi256,
	Ansi16
}

impl ColorSupport {
	pub fn detect() -> Self {
		match available_color_count() {
			u16::MAX => Self::TrueColor,
			256 => Self::Ansi256,
			_ => Self::Ansi16,
		}
	}
}

pub fn try_color(rgb: (u8, u8, u8), support: &ColorSupport) -> ratatui::style::Color {
	match support {
		ColorSupport::TrueColor => ratatui::style::Color::Rgb (rgb.0, rgb.1, rgb.2),
		ColorSupport::Ansi256 => FromCrossterm::from_crossterm(rgb_to_ansi256(rgb.0, rgb.1, rgb.2)),
		ColorSupport::Ansi16 => FromCrossterm::from_crossterm(rgb_to_ansi16(rgb.0, rgb.1, rgb.2)),
	}
}

fn rgb_to_ansi256(r: u8, g: u8, b: u8) -> Color {
	if r == g && g == b {
		if r < 8 { return Color::AnsiValue(16); }
		if r > 248 { return Color::AnsiValue(231); }

		let gray_index = 232 + ((r as f32 - 8.0) / 10.0).round() as u8;

		return Color::AnsiValue(gray_index);
	}

	let r_cube = ((r as f32 / 255.0) * 5.0).round() as u8;
	let g_cube = ((g as f32 / 255.0) * 5.0).round() as u8;
	let b_cube = ((b as f32 / 255.0) * 5.0).round() as u8;
	
	let ansi_index = 16 + (36 * r_cube) + (6 * g_cube) + b_cube;
	
	Color::AnsiValue(ansi_index)
}

pub fn rgb_to_ansi16(r: u8, g: u8, b: u8) -> Color {
	let ansi16_palette = [
		(Color::Black, 0, 0, 0),        (Color::DarkRed, 128, 0, 0),
		(Color::DarkGreen, 0, 128, 0),  (Color::DarkYellow, 128, 128, 0),
		(Color::DarkBlue, 0, 0, 128),   (Color::DarkMagenta, 128, 0, 128),
		(Color::DarkCyan, 0, 128, 128), (Color::Grey, 192, 192, 192),
		(Color::DarkGrey, 128, 128, 128),(Color::Red, 255, 0, 0),
		(Color::Green, 0, 255, 0),      (Color::Yellow, 255, 255, 0),
		(Color::Blue, 0, 0, 255),       (Color::Magenta, 255, 0, 255),
		(Color::Cyan, 0, 255, 255),     (Color::White, 255, 255, 255),
	];

	ansi16_palette
		.iter()
		.min_by_key(|(_, pr, pg, pb)| {
			let dr = (r as i32) - (*pr as i32);
			let dg = (g as i32) - (*pg as i32);
			let db = (b as i32) - (*pb as i32);
			dr * dr + dg * dg + db * db
		})
		.map(|(color, _, _, _)| *color)
		.unwrap_or(Color::Reset)
}