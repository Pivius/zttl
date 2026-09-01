use std::vec;
use image::DynamicImage;
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use ratatui::{style::{Color, Modifier, Style}, text::{Line, Span}};

use crate::app::App;

pub enum MarkdownBlock {
	HeaderImage { text: String, level: HeadingLevel },
	TextBlock(Vec<Line<'static>>),
}

pub trait TagRule {
	fn style(&self, base: Style) -> Style {
		base
	}
	fn prefix(&self) -> Option<String> {
		None
	}
}

pub struct HeadingRule(pub HeadingLevel);

impl TagRule for HeadingRule {
	fn style(&self, base: Style) -> Style {
		let style = match self.0 {
			HeadingLevel::H1 => Style::default()
				.fg(Color::Cyan)
				.add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
			HeadingLevel::H2 => Style::default()
				.fg(Color::Yellow)
				.add_modifier(Modifier::BOLD),
			_ => Style::default()
				.fg(Color::Green)
				.add_modifier(Modifier::BOLD),
		};
		base.patch(style)
	}
	
	fn prefix(&self) -> Option<String> {
		Some("#".repeat(self.0 as usize) + " ")
	}
}

pub struct BoldRule;
impl TagRule for BoldRule {
	fn style(&self, base: Style) -> Style {
		base.add_modifier(Modifier::BOLD)
	}
}

pub struct ItalicRule;
impl TagRule for ItalicRule {
	fn style(&self, base: Style) -> Style {
		base.add_modifier(Modifier::ITALIC)
	}
}

pub struct BulletRule;
impl TagRule for BulletRule {
	fn style(&self, base: Style) -> Style {
		base.fg(Color::DarkGray)
	}
	fn prefix(&self) -> Option<String> {
		Some("  • ".to_string())
	} 
}

pub fn resolve_rule(tag: &Tag) -> Option<Box<dyn TagRule>> {
	match tag {
		Tag::Heading { level, .. } => Some(Box::new(HeadingRule(*level))),
		Tag::Strong => Some(Box::new(BoldRule)),
		Tag::Emphasis => Some(Box::new(ItalicRule)),
		Tag::Item => Some(Box::new(BulletRule)),
		_ => None,
	}
}

fn flush_line(lines: &mut Vec<Line<'static>>, current: &mut Vec<Span<'static>>) {
	if !current.is_empty() {
		lines.push(Line::from(std::mem::take(current)));
	}
}

pub fn render_markdown(content: &str, app: &App) -> Vec<MarkdownBlock> {
	let mut blocks: Vec<MarkdownBlock> = Vec::new();
	let mut lines: Vec<Line<'static>> = Vec::new();
	let mut current_line: Vec<Span<'static>> = Vec::new();
	let mut style_stack: Vec<Style> = vec![Style::default()];
	let mut heading: Option<(HeadingLevel, String)> = None;

	let parser = Parser::new_ext(content, Options::ENABLE_TABLES);

	for event in parser {
		match event {
			Event::Start(Tag::Heading { level, .. }) => {
				heading = Some((level, String::new()));
			},
			Event::End(TagEnd::Heading(level)) => {
				flush_line(&mut lines, &mut current_line);
				
				if !lines.is_empty() {
					blocks.push(MarkdownBlock::TextBlock(std::mem::take(&mut lines)));
				}

				let (_, text) = heading.take().unwrap_or((level, String::new()));

				blocks.push(MarkdownBlock::HeaderImage { text, level });
			},
			Event::Text(t) if heading.is_some() => heading.as_mut().unwrap().1.push_str(&t),
			Event::Code(c) if heading.is_some() => heading.as_mut().unwrap().1.push_str(&c),
			Event::SoftBreak | Event::HardBreak if heading.is_some() => heading.as_mut().unwrap().1.push(' '),
			// Not heading events
			Event::Start(tag) if heading.is_none() => {
				let current = *style_stack.last().unwrap_or(&Style::default());
				if let Some(rule) = resolve_rule(&tag) {
					let new_style = rule.style(current);
					if let Some(prefix) = rule.prefix() {
						current_line.push(Span::styled(prefix, new_style));
					}
					style_stack.push(new_style);
				} else {
					style_stack.push(current);
				}
			},
			Event::End(tag) if heading.is_none() => {
				style_stack.pop();
				if matches!(tag, TagEnd::Item | TagEnd::Paragraph) && !current_line.is_empty() {
					lines.push(Line::from(std::mem::take(&mut current_line)));
				}
			},
			Event::Text(text) => {
				let active = *style_stack.last().unwrap_or(&Style::default());
				current_line.push(Span::styled(text.to_string(), active));
			},
			Event::Code(code) => {
				let code_style = Style::default().bg(app.theme.selection);
				current_line.push(Span::styled(format!(" {} ", code), code_style));
			},
			Event::SoftBreak | Event::HardBreak => flush_line(&mut lines, &mut current_line),
			_ => {}
		}
	}

	flush_line(&mut lines, &mut current_line);

	if !lines.is_empty() {
		blocks.push(MarkdownBlock::TextBlock(lines));
	}

	blocks
}