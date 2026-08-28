use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
	#[default]
	Active,
	Archived,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Frontmatter {
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub title: Option<String>,
	#[serde(default)]
	pub status: Status,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub parents: Option<Vec<String>>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub created: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub updated: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub deadline: Option<String>,
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub tags: Vec<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub share_id: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub visibility: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Block {
	pub id: Option<String>,
	pub text: String,
}

#[derive(Debug, Clone)]
pub struct Note {
	pub slug: String,
	pub path: PathBuf,
	pub frontmatter: Frontmatter,
	pub body: String,
	pub links: Vec<String>,
	pub transclusions: Vec<String>,
	pub blocks: Vec<Block>,
}

impl Note {
	pub fn title(&self) -> &str {
		self.frontmatter.title.as_deref().unwrap_or(&self.slug)
	}
}
