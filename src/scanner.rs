use crate::graph::GraphIndex;
use crate::model::Note;
use crate::parser;
use crate::sys::SystemIO;
use std::path::Path;

pub struct VaultScanner;

impl VaultScanner {
	pub fn scan(vault_path: &Path) -> Result<GraphIndex, Box<dyn std::error::Error>> {
		let root = SystemIO::normalize_canonical_path(vault_path)?;
		let files = SystemIO::collect_markdown_files(&root)?;
		let mut notes = Vec::with_capacity(files.len());

		for rel in files {
			notes.push(Self::parse_file(&root, &rel)?);
		}

		Ok(GraphIndex::build(notes))
	}
	
	fn parse_file(root: &Path, rel: &Path) -> Result<Note, Box<dyn std::error::Error>> {
		let content = std::fs::read_to_string(root.join(rel))?;
		let normalized = parser::normalize(&content);
		let (fm, body) = parser::split_frontmatter(&normalized);
		let frontmatter = parser::parse_frontmatter(fm)?;
		let parsed = parser::parse_body(body);
		let slug = rel
			.file_stem()
			.map(|s| s.to_string_lossy().to_string())
			.unwrap_or_default();

		Ok(Note {
			slug,
			path: rel.to_path_buf(),
			frontmatter,
			body: body.to_string(),
			links: parsed.links,
			transclusions: parsed.transclusions,
			blocks: parsed.blocks,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn scans_fixture_vault() {
		let idx = VaultScanner::scan(Path::new("./vault")).unwrap();
		assert_eq!(idx.graph.node_count(), 7);
		
		let ds = idx.resolve("Data Structures").unwrap();
		let algo = idx.resolve("algorithms").unwrap();
		assert!(idx.graph.contains_edge(ds, algo));
		
		assert_eq!(idx.descendants(ds).len(), 4);
		assert!(idx.block_registry.contains_key("raft-elem-01"));
		assert!(idx.block_registry.contains_key("algo-bound-01"));
		
		assert!(idx
			.unresolved_links
			.iter()
			.any(|(s, t)| s == "unsorted-inbox" && t == "Graph Theory"));
			assert!(idx
				.unresolved_transclusions
				.iter()
				.any(|(s, t)| s == "raft" && t == "cs-root-01"));
			}
		}
		