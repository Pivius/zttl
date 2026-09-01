use crossterm::style::available_color_count;
use crossterm::style::Color as CrossColor;
use palette::{Darken, Lighten, Srgb};
use ratatui::style::Color as TuiColor;
use ratatui::backend::FromCrossterm;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AppColor(pub Srgb<u8>);

impl AppColor {
	pub const fn new(r: u8, g: u8, b: u8) -> Self {
		Self(Srgb::new(r, g, b))
	}

	pub fn lighten(self, amount: f32) -> Self {
		let float_color: Srgb<f32> = self.0.into_format();
		let lightened = float_color.lighten(amount);
		Self(lightened.into_format())
	}

	pub fn darken(self, amount: f32) -> Self {
		let float_color: Srgb<f32> = self.0.into_format();
		let darkened = float_color.darken(amount);
		Self(darkened.into_format())
	}

	pub fn rgb(self) -> (u8, u8, u8) {
		(self.0.red, self.0.green, self.0.blue)
	}

	pub fn to_tui(self, support: &ColorSupport) -> TuiColor {
		let (r, g, b) = (self.0.red, self.0.green, self.0.blue);

		match support {
			ColorSupport::TrueColor => TuiColor::Rgb (r, g, b),
			ColorSupport::Ansi256 => FromCrossterm::from_crossterm(rgb_to_ansi256(r, g, b)),
			ColorSupport::Ansi16 => FromCrossterm::from_crossterm(rgb_to_ansi16(r, g, b)),
		}
	}
}

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

fn rgb_to_ansi256(r: u8, g: u8, b: u8) -> CrossColor {
	if r == g && g == b {
		if r < 8 { return CrossColor::AnsiValue(16); }
		if r > 248 { return CrossColor::AnsiValue(231); }

		let gray_index = 232 + ((r as f32 - 8.0) / 10.0).round() as u8;

		return CrossColor::AnsiValue(gray_index);
	}

	let r_cube = ((r as f32 / 255.0) * 5.0).round() as u8;
	let g_cube = ((g as f32 / 255.0) * 5.0).round() as u8;
	let b_cube = ((b as f32 / 255.0) * 5.0).round() as u8;
	
	let ansi_index = 16 + (36 * r_cube) + (6 * g_cube) + b_cube;
	
	CrossColor::AnsiValue(ansi_index)
}

pub fn rgb_to_ansi16(r: u8, g: u8, b: u8) -> CrossColor {
	let ansi16_palette = [
		(CrossColor::Black, 0, 0, 0),         (CrossColor::DarkRed, 128, 0, 0),
		(CrossColor::DarkGreen, 0, 128, 0),   (CrossColor::DarkYellow, 128, 128, 0),
		(CrossColor::DarkBlue, 0, 0, 128),    (CrossColor::DarkMagenta, 128, 0, 128),
		(CrossColor::DarkCyan, 0, 128, 128),  (CrossColor::Grey, 192, 192, 192),
		(CrossColor::DarkGrey, 128, 128, 128),(CrossColor::Red, 255, 0, 0),
		(CrossColor::Green, 0, 255, 0),       (CrossColor::Yellow, 255, 255, 0),
		(CrossColor::Blue, 0, 0, 255),        (CrossColor::Magenta, 255, 0, 255),
		(CrossColor::Cyan, 0, 255, 255),      (CrossColor::White, 255, 255, 255),
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
		.unwrap_or(CrossColor::Reset)
}