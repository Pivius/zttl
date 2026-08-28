use crate::model::{Block, Frontmatter};
use regex::Regex;
use std::sync::OnceLock;

pub struct ParsedBody {
	pub links: Vec<String>,
	pub transclusions: Vec<String>,
	pub blocks: Vec<Block>,
}

fn link_re() -> &'static Regex {
	static RE: OnceLock<Regex> = OnceLock::new();
	RE.get_or_init(|| Regex::new(r"\[\[([^\]]+)\]\]").unwrap())
}

fn transclusion_re() -> &'static Regex {
	static RE: OnceLock<Regex> = OnceLock::new();
	RE.get_or_init(|| Regex::new(r"\(\(([^)]+)\)\)").unwrap())
}

fn block_id_re() -> &'static Regex {
	static RE: OnceLock<Regex> = OnceLock::new();
	RE.get_or_init(|| Regex::new(r"\s*\^([A-Za-z0-9][A-Za-z0-9_-]*)\s*$").unwrap())
}

pub fn normalize(content: &str) -> String {
	content.replace("\r\n", "\n")
}

pub fn split_frontmatter(content: &str) -> (&str, &str) {
	let content = content.strip_prefix('\u{feff}').unwrap_or(content);
	let Some(rest) = content.strip_prefix("---\n") else {
		return ("", content);
	};
	let Some(idx) = rest.find("\n---") else {
		return ("", content);
	};
	let frontmatter = &rest[..idx];
	let mut body = &rest[idx + 4..];

	body = body.strip_prefix('\n').unwrap_or(body);

	(frontmatter, body)
}

pub fn parse_frontmatter(fm: &str) -> Result<Frontmatter, serde_yaml::Error> {
	if fm.trim().is_empty() {
		Ok(Frontmatter::default())
	} else {
		serde_yaml::from_str(fm)
	}
}

pub fn parse_body(body: &str) -> ParsedBody {
	let links = link_re()
		.captures_iter(body)
		.filter_map(|c| c.get(1))
		.map(|m| m.as_str().trim().to_string())
		.collect();
	let transclusions = transclusion_re()
		.captures_iter(body)
		.filter_map(|c| c.get(1))
		.map(|m| m.as_str().trim().to_string())
		.collect();
	let blocks = parse_blocks(body);

	ParsedBody {
		links,
		transclusions,
		blocks,
	}
}

fn parse_blocks(body: &str) -> Vec<Block> {
	let mut blocks = Vec::new();

	for line in body.lines() {
		let Some(rest) = split_bullet(line) else {
			continue;
		};
		let (id, text) = split_block_id(rest);

		blocks.push(Block {
			id,
			text: text.trim().to_string(),
		});
	}
	blocks
}

fn split_bullet(line: &str) -> Option<&str> {
	let rest = line.trim_start();
	let bytes = rest.as_bytes();

	if bytes.len() >= 2 && matches!(bytes[0], b'-' | b'*' | b'+') && bytes[1] == b' ' {
		Some(&rest[2..])
	} else {
		None
	}
}

fn split_block_id(text: &str) -> (Option<String>, &str) {
	let Some(caps) = block_id_re().captures(text) else {
		return (None, text);
	};
	let id = caps.get(1).map(|m| m.as_str().to_string());
	let end = caps.get(0).map(|m| m.start()).unwrap_or(text.len());

	(id, &text[..end])
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn splits_frontmatter() {
		let content = "---\ntitle: \"X\"\n---\nbody";
		let (fm, body) = split_frontmatter(content);
		assert_eq!(fm, "title: \"X\"");
		assert_eq!(body, "body");
	}
	
	#[test]
	fn no_frontmatter_returns_whole_content_as_body() {
		let content = "just body";
		let (fm, body) = split_frontmatter(content);
		assert_eq!(fm, "");
		assert_eq!(body, "just body");
	}
	
	#[test]
	fn extracts_links_transclusions_and_blocks() {
		let body = "- hello ^block-01\n- [[Alpha]] and ((beta))\n  - nested\n";
		let p = parse_body(body);
		assert_eq!(p.links, vec!["Alpha"]);
		assert_eq!(p.transclusions, vec!["beta"]);
		assert_eq!(p.blocks.len(), 3);
		assert_eq!(p.blocks[0].id.as_deref(), Some("block-01"));
		assert_eq!(p.blocks[0].text, "hello");
		assert_eq!(p.blocks[2].text, "nested");
	}
	
	#[test]
	fn parses_frontmatter_defaults() {
		let fm = parse_frontmatter("id: \"abc\"\n").unwrap();
		assert_eq!(fm.id.as_deref(), Some("abc"));
		//assert_eq!(fm.r#type, crate::model::NoteType::Note);
		assert!(fm.parents.is_none());
	}
}
