use std::{collections::HashSet};
use petgraph::graph::NodeIndex;
use crate::graph::{EdgeKind, GraphIndex};

pub enum ViewMode { Ego, Spatial }
pub enum RefKind {
	Conceptual,
	Structural,
	Backlink
}

pub struct Column {
	pub items: Vec<NodeIndex>,
	pub focus: usize
}

pub struct EgoEntry {
	pub node: NodeIndex,
	pub depth: usize,
	pub category: Option<RefKind>
}

pub struct App {
	pub index: GraphIndex,
	pub mode: ViewMode,
	pub active: Option<NodeIndex>,
	pub columns: Vec<Column>,
	pub expanded: HashSet<NodeIndex>,
	pub ego_focus: usize,
	pub running: bool
}

impl App {
	//TODO - Config maintaining persistence
	pub fn new(index: GraphIndex) -> App {
		let roots = index.roots();
		let initial_column = Column {
			items: roots,
			focus: 0
		};

		App {
			index,
			mode: ViewMode::Ego,
			active: None,
			columns: vec![initial_column],
			expanded: HashSet::new(),
			ego_focus: 0,
			running: true
		}
	}

	pub fn focused_node(&self) -> Option<NodeIndex> {
		match self.mode {
			ViewMode::Ego => {
				let visible = self.ego_visible();
				
				if visible.is_empty() {
					None
				} else {
					let focus = self.ego_focus.min(visible.len() - 1);
					Some(visible[focus].node)
				}
			},
			ViewMode::Spatial => {
				let last_column = self.columns.last()?;
				if last_column.items.is_empty() {
					None
				} else {
					let focus = last_column.focus.min(last_column.items.len() - 1);
					Some(last_column.items[focus])
				}
			},
		}
	}

	fn collect_ego_children(
		&self,
		parent: NodeIndex,
		depth: usize,
		seen: &mut HashSet<NodeIndex>,
		out: &mut Vec<EgoEntry>
	) {
		let mut children = Vec::new();

		//for child in self.index.children_of(parent) {
		//	children.push((child, RefKind::Structural));
		//}

		for (kind, target) in self.index.forward_refs(parent) {
			let category = match kind {
				EdgeKind::Transcludes => RefKind::Structural,
				EdgeKind::Links | _ => RefKind::Conceptual
			};

			children.push((target, category));
		}

		for source in self.index.backlinks_of(parent) {
			children.push((source, RefKind::Backlink));
		}

		for (node, category) in children {
			if seen.insert(node) {
				out.push(EgoEntry {
					node,
					depth,
					category: Some(category)
				});

				if self.expanded.contains(&node) {
					self.collect_ego_children(node, depth + 1, seen, out);
				}
			}
		}
	}

	pub fn ego_visible(&self) -> Vec<EgoEntry> {
		let mut visible = Vec::new();

		let Some(root) = self.active else {
			return visible;
		};

		let mut seen = HashSet::new();
		seen.insert(root);

		visible.push(EgoEntry {
			node: root,
			depth: 0,
			category: None
		});

		if self.expanded.contains(&root) {
			self.collect_ego_children(root, 1, &mut seen, &mut visible);
		}

		visible
	}
}

impl App {
	pub fn toggle_mode(&mut self) {
		match self.mode {
			ViewMode::Ego => self.mode = ViewMode::Spatial,
			ViewMode::Spatial => self.mode = ViewMode::Ego,
		}
	}

	pub fn move_focus(&mut self, delta: isize) {
		match self.mode {
			ViewMode::Ego => {
				let visible_len = self.ego_visible().len();
				if visible_len == 0 {
					self.ego_focus = 0;
					return;
				}

				let new_focus = self.ego_focus as isize + delta;
				self.ego_focus = new_focus.clamp(0, (visible_len - 1) as isize) as usize;
			},
			ViewMode::Spatial => {
				if let Some(col) = self.columns.last_mut() {
					if col.items.is_empty() {
						col.focus = 0;
						return;
					}

					let new_focus = col.focus as isize + delta;
					col.focus = new_focus.clamp(0, (col.items.len() - 1) as isize) as usize;
				}
			}
		}
	}

	pub fn descend(&mut self) {
		let Some(focused) = self.focused_node() else { return };

		match self.mode {
			ViewMode::Ego => {
				if !self.expanded.contains(&focused) {
					self.expanded.insert(focused);
				}
			},
			ViewMode::Spatial => {
				let children: Vec<NodeIndex> = self.index.children_of(focused);
				//	.into_iter()
				//	.chain(self.index.forward_refs(focused).into_iter().map(|(_, target)| target))
				//	.collect();

				if !children.is_empty() {
					self.columns.push(Column {
						items: children,
						focus: 0
					});
				}
			}
		}
	}

	pub fn ascend(&mut self) {
		match self.mode {
			ViewMode::Ego => {
				let visible = self.ego_visible();
				if visible.is_empty() { return };

				let focus_clamp = self.ego_focus.min(visible.len() - 1);
				let current_entry = &visible[focus_clamp];
				let current_node = current_entry.node;

				if self.expanded.contains(&current_node) {
					self.expanded.remove(&current_node);
					self.ego_focus = focus_clamp;
				} else if current_entry.depth > 0 {
					let target_depth = current_entry.depth - 1;
					if let Some((idx, parent_entry)) = visible[..focus_clamp]
						.iter()
						.enumerate()
						.rfind(|(_, e)| e.depth == target_depth)
					{
						self.expanded.remove(&parent_entry.node);
						self.ego_focus = idx;
					}
				}
			},
			ViewMode::Spatial => {
				if self.columns.len() > 1 {
					self.columns.pop();
				}
			}
		}
	}

	pub fn open(&mut self, n: NodeIndex) {
		self.expanded.clear();
		self.active = Some(n);
		self.ego_focus = 0;
	}

	pub fn open_focused(&mut self) {
		if let Some(n) = self.focused_node() {
			self.open(n);
		}
	}

	pub fn jump(&mut self, query: &str) {
		if let Some(target_idx) = self.index.resolve_fuzzy(query) {
			self.open(target_idx);
		}
	}

	pub fn quit(&mut self) {
		self.running = false;
	}
}
