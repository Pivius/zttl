use ratatui::style::Color;
use crate::ui::colors::{ColorSupport, AppColor};

#[derive(Debug, Clone, Copy)]
pub enum Palette {
	MidnightBlack,
	AegeanBlue,
	AverageBlue,
	
	// Complementary
	DeepCharcoal,
	SubtleBorder,
	
	IceWhite,
	MutedSlate,
	
	CrimsonRed,
	AmberGold,
	EmeraldGreen,
	CyanInfo
}

impl Palette {
	pub fn color(self) -> AppColor {
		match self {
			Self::MidnightBlack => AppColor::new(0, 0, 11),   // #01000b
			Self::AegeanBlue => AppColor::new(68, 97, 123),   // #44617B
			Self::AverageBlue => AppColor::new(77, 128, 230), // #4D80E6
			Self::DeepCharcoal => AppColor::new(18, 24, 38),  // #121826
			Self::SubtleBorder => AppColor::new(36, 48, 68),  // #243044
			Self::IceWhite => AppColor::new(230, 240, 255),   // #E7F0FF
			Self::MutedSlate => AppColor::new(112, 138, 165), // #708AA5
			Self::CrimsonRed => AppColor::new(224, 72, 90),   // #E0485A
			Self::AmberGold => AppColor::new(228, 160, 62),   // #E4A03E
			Self::EmeraldGreen => AppColor::new(60, 185, 140),// #3CB98C
			Self::CyanInfo => AppColor::new(56, 178, 206),    // #38B2CE
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
	pub support: ColorSupport
}

impl Theme {
	pub fn new(support: ColorSupport) -> Self {
		Self {
			background: Palette::MidnightBlack.color().to_tui(&support),
			foreground: Palette::DeepCharcoal.color().to_tui(&support),
			text: Palette::IceWhite.color().to_tui(&support),
			muted_text: Palette::MutedSlate.color().to_tui(&support),
			border: Palette::SubtleBorder.color().to_tui(&support),
			focused_border: Palette::AverageBlue.color().to_tui(&support),
			selection: Palette::AegeanBlue.color().darken(0.2).to_tui(&support),
			cursor: Palette::IceWhite.color().to_tui(&support),
			accent: Palette::AverageBlue.color().to_tui(&support),
			error: Palette::CrimsonRed.color().to_tui(&support),
			warning: Palette::AmberGold.color().to_tui(&support),
			success: Palette::EmeraldGreen.color().to_tui(&support),
			info: Palette::CyanInfo.color().to_tui(&support),
			support
		}
	}
}