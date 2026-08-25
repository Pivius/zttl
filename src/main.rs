mod graph;
mod model;
mod parser;
mod scanner;
mod sys;

use graph::EdgeKind;
use petgraph::visit::EdgeRef;
use petgraph::Direction;
use scanner::VaultScanner;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let index = VaultScanner::scan(Path::new("./vault"))?;
	
	println!("Notes ({})", index.graph.node_count());
	let mut nodes: Vec<_> = index.graph.node_indices().collect();

	nodes.sort_by_key(|&n| index.graph[n].slug.clone());

	for &n in &nodes {
		let note = &index.graph[n];
		let id = note.frontmatter.id.as_deref().unwrap_or("-");
		println!(
			"  {:>8}  {:<24} slug={:<20} id={:<26} {}",
			format!("{:?}", note.frontmatter.r#type),
			note.title(),
			note.slug,
			id,
			note.path.display()
		);
	}
	
	println!("\nContainment edges");
	for &n in &nodes {
		for e in index.graph.edges_directed(n, Direction::Outgoing) {
			if *e.weight() == EdgeKind::Contains {
				println!(
					"  {} -> {}",
					index.graph[n].title(),
					index.graph[e.target()].title()
				);
			}
		}
	}
	
	println!("\nAssociative edges");
	for &n in &nodes {
		for e in index.graph.edges_directed(n, Direction::Outgoing) {
			if *e.weight() == EdgeKind::Links {
				println!(
					"  {} -> {}",
					index.graph[n].title(),
					index.graph[e.target()].title()
				);
			}
		}
	}
	
	println!("\nBlock registry");
	let mut blocks: Vec<_> = index.block_registry.iter().collect();

	blocks.sort_by_key(|(id, _)| id.to_string());
	
	for (id, bref) in blocks {
		println!(
			"  {} -> {} : \"{}\"",
			id,
			index.graph[bref.note].title(),
			bref.text
		);
	}
	
	println!("\nUnresolved links");
	for (slug, target) in &index.unresolved_links {
		println!("  {} -> {}", slug, target);
	}
	
	println!("\nUnresolved parents");
	for (slug, target) in &index.unresolved_parents {
		println!("  {} -> {}", slug, target);
	}
	
	println!("\nUnresolved transclusions");
	for (slug, target) in &index.unresolved_transclusions {
		println!("  {} -> {}", slug, target);
	}
	
	println!("\nDescendants of Data Structures");
	if let Some(root) = index.resolve("Data Structures") {
		for d in index.descendants(root) {
			println!("  {}", index.graph[d].title());
		}
	}
	
	Ok(())
}
