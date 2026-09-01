use std::vec;

use image::DynamicImage;
use pulldown_cmark::HeadingLevel;
use ratatui::{Frame, layout::Rect, style::{Color, Modifier, Style}, text::{Line, Span}, widgets::{Block, Borders, Paragraph, Wrap}};
use ratatui_image::{Image, Resize, picker::Picker, protocol::Protocol};

use crate::{app::{App, HeaderKey}, ui::{header_image::{self, HeaderSpec}, markdown::{MarkdownBlock, render_markdown}, theme::header_fill}};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
	let block = Block::default().title("Editor").borders(Borders::ALL);
	let inner = block.inner(area);
	
	frame.render_widget(block, area);
	
	let blocks: Vec<MarkdownBlock> = match app.focused_node() {
		Some(node) => render_markdown(&app.index.graph[node].body, app),
		None => vec![MarkdownBlock::TextBlock(vec![Line::from("No active note")])]
	};
	let mut y = inner.y;
	
	for b in blocks {
		if y >= inner.bottom() {
			break;
		}
		
		match b {
			MarkdownBlock::TextBlock(lines) => {
				let h = (lines.len() as u16).min(inner.bottom() - y);
				let rect = Rect { x: inner.x, y, width: inner.width, height: h };
				
				frame.render_widget(Paragraph::new(lines), rect);
				y += h;
			},
			MarkdownBlock::HeaderImage { text, level} => {
				let Some(picker) = &app.image_picker else {
					frame.render_widget(
						Paragraph::new(text), 
						Rect { y, height: 1, ..inner}
					);
					y += 1;
					continue;
				};
				
				match header_proto(app, picker, &text, level, inner, y) {
					Some(proto) => {
						let rows = proto.size().height;
						
						if y + rows > inner.bottom() { break; }
						
						let rect = Rect { x: inner.x, y, width: inner.width, height: rows };
						
						frame.render_widget(Image::new(&proto), rect);
						y += rows;
					},
					None => {
						let rect = Rect { x: inner.x, y, width: inner.width, height: 1 };
						
						frame.render_widget(Paragraph::new(heading_line(&text, level, app)), rect);
					}
				}
			}
		}
	}
}

fn heading_line(text: &str, level: HeadingLevel, app: &App) -> Line<'static> {
	let color: Color = match level {
		HeadingLevel::H1 => app.theme.accent,
		HeadingLevel::H2 => app.theme.success,
		_ => app.theme.info,
	};
	Line::from(Span::styled(
		text.to_string(),
		Style::default().fg(color).add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
	))
}

fn header_proto(
	app: &App,
	picker: &Picker,
	text: &str,
	level: HeadingLevel,
	inner: Rect,
	y: u16,
) -> Option<Protocol> {
	let slug = app.active
	.map(|n| app.index.graph[n].slug.clone())
	.unwrap_or_default();
	let key = HeaderKey {
		slug,
		text: text.to_string(),
		level: level as u8,
		width: inner.width
	};
	
	if let Some(p) = app.header_cache.borrow().get(&key) {
		return Some(p.clone());
	}
	
	let fs = picker.font_size();
	let spec = HeaderSpec {
		text: text.to_string(),
		level,
		font_family: app.font_family.clone(),
		cell_w: fs.width as u32,
		cell_h: fs.height as u32,
		fill: header_fill(level)
	};
	
	let img = header_image::rasterize(&spec).ok()?;
	
	if is_blank(&img) {
		return None;
	}
	
	let remaining = inner.bottom().saturating_sub(y).max(1);
	let target = ratatui::layout::Size::new(inner.width, remaining);
	let proto = picker.new_protocol(img, target, Resize::Fit(None)).ok()?;
	
	app.header_cache.borrow_mut().insert(key, proto.clone());
	Some(proto)
}

fn is_blank(img: &DynamicImage) -> bool {
	img.to_rgba8().pixels().all(|p| p.0[3] == 0)
}