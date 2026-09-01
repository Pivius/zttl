use std::{error::Error, format, sync::Arc};

use image::{DynamicImage, RgbaImage};
use palette::num::Round;
use pulldown_cmark::HeadingLevel;
use unicode_width::UnicodeWidthStr;
use usvg::fontdb::{self, Database, Family, Query};

use crate::app::{H1_SCALE, H2_SCALE, H3_SCALE};

pub struct HeaderSpec {
	pub text: String,
	pub level: HeadingLevel,
	pub font_family: String,
	pub cell_w: u32,
	pub cell_h: u32,
	pub fill: (u8, u8, u8)
}

fn normalize(s: &str) -> String {
	s.to_lowercase()
		.chars()
		.filter(|c| c.is_alphanumeric())
		.collect()
}

fn canonicalize_family(db: &Database, requested: &str) -> Option<String> {
	let query = Query {
		families: &[Family::Name(requested)],
		..Default::default()
	};

	if let Some(id) = db.query(&query) {
		if let Some((name, _)) = db.face(id).and_then(|f| f.families.first()) {
			return Some(name.clone());
		}
	}

	let needle = normalize(requested);

	if needle.is_empty() {
		return None;
	}

	// (monospaced, score, len, name)
	let mut best: Option<(bool, i32, usize, String)> = None;

	for face in db.faces() {
		for (name, _) in &face.families {
			let candidate = normalize(name);
			let score = if candidate == needle {
				3
			} else if candidate.starts_with(&needle) {
				2
			} else if candidate.contains(&needle) || needle.contains(&candidate) {
				1
			} else {
				0
			};

			if score == 0 {
				continue;
			}

			let this = (face.monospaced, score, name.len(), name.to_string());
			let better = best.as_ref().map_or(true, |b| {
				(this.0, this.1) > (b.0, b.1)
					|| ((this.0, this.1) == (b.0, b.1) && this.2 < b.2) 
			});

			if better {
				best = Some(this);
			}
		}
	}

	best.map(|(_, _, _, name)| name)
}

fn scale_for(level: HeadingLevel) -> f32 {
	match level {
		HeadingLevel::H1 => H1_SCALE,
		HeadingLevel::H2 => H2_SCALE,
		HeadingLevel::H3 => H3_SCALE,
		_ => 1.0
	}
}

fn build_svg(spec: &HeaderSpec, family: &str, font_px: u32, w: u32, h: u32) -> String {
	let baseline = font_px;
	let (r, g, b) = spec.fill;
	format!(
		"<svg width=\"{w}\" height=\"{h}\" xmlns=\"http://www.w3.org/2000/svg\">
			<text x=\"0\" y=\"{baseline}\" font-family=\"{fam}\" font-size=\"{font_px}\"
				fill=\"#{r:02x}{g:02x}{b:02x}\">{text}</text>
		</svg>",
		fam = family,
		text = xml_escape(&spec.text),
	)
}

fn xml_escape(s: &str) -> String {
	s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

pub fn rasterize(spec: &HeaderSpec) -> Result<DynamicImage, Box<dyn Error>> {
	let scale = scale_for(spec.level);
	let font_px = (spec.cell_h as f32 * scale).round() as u32;
	let cells = UnicodeWidthStr::width(spec.text.as_str()) as u32;
	let w = (cells as f32 * spec.cell_w as f32 * scale).ceil() as u32 + font_px * 2;
	let h = font_px * 2;

	let mut db = Database::new();
	db.load_system_fonts();

	let family = canonicalize_family(&db, &spec.font_family)
		.unwrap_or_else(|| "monospace".to_string());
	
	let mut opts = usvg::Options::default();
	opts.fontdb = Arc::new(db);
	
	let tree = resvg::usvg::Tree::from_str(&build_svg(spec, &family, font_px, w, h), &opts)?;
	let mut pixmap = tiny_skia::Pixmap::new(w, h).ok_or("zero-size pixmap")?;
	
	resvg::render(&tree, resvg::tiny_skia::Transform::default(), &mut pixmap.as_mut());
	
	let rgba = RgbaImage::from_raw(w, h, pixmap.data().to_vec()).ok_or("bad buffer")?;
	
	Ok(crop_to_content(DynamicImage::ImageRgba8(rgba)))
}

fn crop_to_content(img: DynamicImage) -> DynamicImage {
	let rgba = img.to_rgba8();
	let (w, h) = rgba.dimensions();
	let mut min_x = w;
	let mut min_y = h;
	let mut max_x = 0u32;
	let mut max_y = 0u32;
	let mut any = false;
	
	for (x, y, p) in rgba.enumerate_pixels() {
		if p.0[3] != 0 {
			any = true;
			min_x = min_x.min(x);
			max_x = max_x.max(x);
			min_y = min_y.min(y);
			max_y = max_y.max(y);
		}
	}
	
	if !any {
		return DynamicImage::ImageRgba8(rgba);
	}
	
	DynamicImage::ImageRgba8(rgba).crop_imm(min_x, min_y, max_x - min_x + 1, max_y - min_y + 1)
}