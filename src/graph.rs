use crate::model::{Note, NoteType};
use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::EdgeRef;
use petgraph::Direction;
use std::collections::{HashMap, HashSet};
use ulid::Ulid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeKind {
	Contains,
	Links,
}

#[derive(Debug, Clone)]
pub struct BlockRef {
	pub note: NodeIndex,
	pub text: String,
}

pub struct GraphIndex {
	pub graph: Graph<Note, EdgeKind>,
	pub by_id: HashMap<String, NodeIndex>,
	pub by_slug: HashMap<String, NodeIndex>,
	pub by_title: HashMap<String, NodeIndex>,
	pub block_registry: HashMap<String, BlockRef>,
	pub unresolved_links: Vec<(String, String)>,
	pub unresolved_parents: Vec<(String, String)>,
	pub unresolved_transclusions: Vec<(String, String)>,
}

impl GraphIndex {
	pub fn build(notes: Vec<Note>) -> GraphIndex {
		let mut graph = Graph::<Note, EdgeKind>::new();
		let mut by_id = HashMap::new();
		let mut by_slug = HashMap::new();
		let mut by_title = HashMap::new();
		
		for note in notes {
			let idx = graph.add_node(note);
			let title = graph[idx].title().to_string();

			if let Some(id) = graph[idx].frontmatter.id.clone() {
				by_id.insert(id, idx);
			}

			by_slug.insert(graph[idx].slug.clone(), idx);
			by_title.insert(title, idx);
		}
		
		let nodes: Vec<NodeIndex> = graph.node_indices().collect();
		let mut unresolved_parents = Vec::new();
		
		for &node in &nodes {
			let slug = graph[node].slug.clone();
			let parents = graph[node].frontmatter.parents.clone();
			let Some(parents) = parents else {
				continue;
			};

			for p in parents {
				match resolve_ref(&by_id, &by_slug, &by_title, &p) {
					Some(pidx) => {
						graph.add_edge(pidx, node, EdgeKind::Contains);
					}
					None => unresolved_parents.push((slug.clone(), p)),
				}
			}
		}
		
		for &s in &nodes {
			let is_struct = matches!(
				graph[s].frontmatter.r#type,
				NoteType::Area | NoteType::Project
			);

			if !is_struct {
				continue;
			}

			for link in graph[s].links.clone() {
				let Some(t) = resolve_ref(&by_id, &by_slug, &by_title, &link) else {
					continue;
				};

				if graph[t].frontmatter.parents.is_none() {
					graph.add_edge(s, t, EdgeKind::Contains);
				}
			}
		}
		
		let mut unresolved_links = Vec::new();

		for &n in &nodes {
			let slug = graph[n].slug.clone();

			for link in graph[n].links.clone() {
				let Some(t) = resolve_ref(&by_id, &by_slug, &by_title, &link) else {
					unresolved_links.push((slug.clone(), link));
					continue;
				};

				if !graph.contains_edge(n, t) {
					graph.add_edge(n, t, EdgeKind::Links);
				}
			}
		}
		
		let mut block_registry = HashMap::new();

		for &n in &nodes {
			for block in &graph[n].blocks {
				if let Some(id) = &block.id {
					block_registry.insert(
						id.clone(),
						BlockRef {
							note: n,
							text: block.text.clone(),
						},
					);
				}
			}
		}
		
		let mut unresolved_transclusions = Vec::new();

		for &n in &nodes {
			let slug = graph[n].slug.clone();

			for t in &graph[n].transclusions {
				if !block_registry.contains_key(t) {
					unresolved_transclusions.push((slug.clone(), t.clone()));
				}
			}
		}
		
		GraphIndex {
			graph,
			by_id,
			by_slug,
			by_title,
			block_registry,
			unresolved_links,
			unresolved_parents,
			unresolved_transclusions,
		}
	}
	
	pub fn resolve(&self, target: &str) -> Option<NodeIndex> {
		resolve_ref(&self.by_id, &self.by_slug, &self.by_title, target)
	}
	
	pub fn descendants(&self, node: NodeIndex) -> Vec<NodeIndex> {
		let mut out = Vec::new();
		let mut stack = vec![node];
		let mut seen = HashSet::new();

		seen.insert(node);

		while let Some(cur) = stack.pop() {
			let children: Vec<NodeIndex> = self
				.graph
				.edges_directed(cur, Direction::Outgoing)
				.filter(|e| *e.weight() == EdgeKind::Contains)
				.map(|e| e.target())
				.collect();

			for c in children {
				if seen.insert(c) {
					out.push(c);
					stack.push(c);
				}
			}
		}
		out
	}

	pub fn resolve_fuzzy(&self, target: &str) -> Option<NodeIndex> {
		if let Some(idx) = self.resolve(target) {
			return Some(idx);
		}
	
		let target_lower = target.to_lowercase();
		let target_norm = target_lower.replace(['-', '_', ' '], "");
		let mut best: Option<(i32, usize, NodeIndex)> = None;
	
		for (slug, idx) in &self.by_slug {
			let score = fuzzy_score(&slug, &target_lower, &target_norm);

			if score > 0 {
				let key = (score, slug.len());
				if best.map_or(true, |(bs, bl, _)| key.0 > bs || (key.0 == bs && key.1 < bl)) {
					best = Some((key.0, key.1, *idx))
				}
			}
		}

		for (title, idx) in &self.by_title {
			let score = fuzzy_score(&title, &target_lower, &target_norm);

			if score > 0 {
				let key = (score, title.len());

				if best.map_or(true, |(bs, bl, _)| key.0 > bs || (key.0 == bs && key.1 < bl)) {
					best = Some((key.0, key.1, *idx));
				}
			}
		}

		best.map(|(_, _, idx)| idx)
	}

	pub fn generate_ulid() -> Ulid {
		Ulid::generate()
	}
}

fn resolve_ref(
	by_id: &HashMap<String, NodeIndex>,
	by_slug: &HashMap<String, NodeIndex>,
	by_title: &HashMap<String, NodeIndex>,
	target: &str,
) -> Option<NodeIndex> {
	if let Some(i) = by_id.get(target) {
		return Some(*i);
	}
	if let Some(i) = by_title.get(target) {
		return Some(*i);
	}
	by_slug.get(target).copied()
}

fn fuzzy_score(candidate: &str, target_lower: &str, target_norm: &str) -> i32 {
	let c_lower = candidate.to_lowercase();
	let c_norm = c_lower.replace(['-', '_', ' '], "");
	if c_lower == target_lower {100}
	else if c_norm == target_norm {90}
	else if c_lower.starts_with(target_lower) || target_lower.starts_with(&c_lower) {60}
	else if c_lower.contains(target_lower) || target_lower.contains(&c_lower) {40}
	else {0}
}

#[cfg(test)]
mod tests {
	use std::assert_eq;
	use super::*;
	use crate::model::Frontmatter;
	
	fn note(
		slug: &str,
		title: &str,
		parents: Option<Vec<&str>>,
		links: Vec<&str>,
		ty: NoteType,
	) -> Note {
		Note {
			slug: slug.to_string(),
			path: Default::default(),
			frontmatter: Frontmatter {
				id: None,
				title: Some(title.to_string()),
				r#type: ty,
				status: Default::default(),
				parents: parents.map(|v| v.into_iter().map(str::to_string).collect()),
				..Default::default()
			},
			body: String::new(),
			links: links.into_iter().map(str::to_string).collect(),
			transclusions: Vec::new(),
			blocks: Vec::new(),
		}
	}

	#[test]
	fn explicit_parents_become_containment() {
		let idx = GraphIndex::build(vec![
			note("ds", "Data Structures", Some(vec![]), vec!["Algorithms"], NoteType::Area),
			note("algo", "Algorithms", Some(vec!["ds"]), vec![], NoteType::Area),
		]);
		let ds = idx.resolve("ds").unwrap();
		let algo = idx.resolve("algo").unwrap();
		assert!(idx.graph.contains_edge(ds, algo));
		let e = idx.graph.find_edge(ds, algo).unwrap();
		assert_eq!(*idx.graph.edge_weight(e).unwrap(), EdgeKind::Contains);
		assert_eq!(idx.descendants(ds), vec![algo]);
	}

	#[test]
	fn membership_derived_when_parents_absent() {
		let idx = GraphIndex::build(vec![
			note("area", "Area", Some(vec![]), vec!["child"], NoteType::Area),
			note("child", "Child", None, vec![], NoteType::Note),
		]);
		let area = idx.resolve("area").unwrap();
		let child = idx.resolve("child").unwrap();
		assert!(idx.graph.contains_edge(area, child));
		let e = idx.graph.find_edge(area, child).unwrap();
		assert_eq!(*idx.graph.edge_weight(e).unwrap(), EdgeKind::Contains);
	}

	#[test]
	fn explicit_empty_parents_blocks_membership() {
		let idx = GraphIndex::build(vec![
			note("area", "Area", Some(vec![]), vec!["child"], NoteType::Area),
			note("child", "Child", Some(vec![]), vec![], NoteType::Note),
		]);
		let area = idx.resolve("area").unwrap();
		let child = idx.resolve("child").unwrap();
		let e = idx.graph.find_edge(area, child).unwrap();
		assert_eq!(*idx.graph.edge_weight(e).unwrap(), EdgeKind::Links);
	}

	#[test]
	fn resolution_prefers_id_then_title_then_slug() {
		let mut notes = vec![note("slug-name", "Title Name", None, vec![], NoteType::Note)];
		notes[0].frontmatter.id = Some("ulid-123".to_string());
		let idx = GraphIndex::build(notes);
		assert_eq!(idx.resolve("ulid-123"), idx.resolve("Title Name"));
		assert_eq!(idx.resolve("Title Name"), idx.resolve("slug-name"));
	}

	#[test]
	fn unresolved_links_recorded() {
		let idx = GraphIndex::build(vec![note(
			"a",
			"A",
			None,
			vec!["Missing Target"],
			NoteType::Note,
		)]);
		assert_eq!(idx.unresolved_links, vec![("a".to_string(), "Missing Target".to_string())]);
	}

	#[test]
	fn fuzzy_case_insensitive() {
		let idx = GraphIndex::build(vec![
			note("algorithms", "Algorithms", None, vec![], NoteType::Note)
		]);
		assert!(idx.resolve_fuzzy("algorithms").is_some());
		assert!(idx.resolve_fuzzy("ALGORITHMS").is_some());
	}

	#[test]
	fn fuzzy_normalized() {
		let idx = GraphIndex::build(vec![
			note("distributed-systems", "Distributed Systems", None, vec![], NoteType::Note)
		]);
		assert!(idx.resolve_fuzzy("distributed systems").is_some());
		assert!(idx.resolve_fuzzy("DistributedSystems").is_some());
	}

	#[test]
	fn fuzzy_prefix() {
		let idx = GraphIndex::build(vec![
			note("algorithms", "Algorithms", None, vec![], NoteType::Note)
		]);
		assert!(idx.resolve_fuzzy("algo").is_some());
	}
	
	#[test]
	fn fuzzy_substring() {
		let idx = GraphIndex::build(vec![
			note("algorithms", "Algorithms", None, vec![], NoteType::Note)
		]);
		assert!(idx.resolve_fuzzy("rithm").is_some());
	}

	#[test]
	fn fuzzy_tiebreaker_prefers_shorter() {
		let idx = GraphIndex::build(vec![
			note("algorithms", "Algorithms", None, vec![], NoteType::Note),
			note("algo", "Algo", None, vec![], NoteType::Note)
		]);
		let r = idx.resolve_fuzzy("algo");
		assert!(r.is_some());
		assert_eq!(idx.graph[r.unwrap()].slug, "algo");
	}

	#[test]
	fn ulid_returns_26_chars() {
		let id = GraphIndex::generate_ulid();
		assert_eq!(id.to_string().len(), 26);
	}
}