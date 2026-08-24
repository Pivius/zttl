use crate::sys::FsEntry;
use crate::tree::{DirectoryNode, DocumentNode, WorkspaceNode};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};


pub struct VaultScanner;

impl VaultScanner {
	pub fn build_workspace(
		workspace_name: String,
		root_path: PathBuf,
		entries: Vec<FsEntry>
	) -> WorkspaceNode {
		let mut workspace = WorkspaceNode {
			name: workspace_name,
			root_path,
			directories: BTreeMap::new(),
			root_documents: BTreeMap::new()
		};

		let mut raw_dirs = Vec::new();
		let mut raw_files = Vec::new();

		for entry in entries {
			match entry {
				FsEntry::Directory { relative_path } => raw_dirs.push(relative_path),
				FsEntry::File { relative_path } => raw_files.push(relative_path)
			}
		}

		for dir_path in raw_dirs {
			Self::insert_directory(&mut workspace, &dir_path);
		}

		for file_path in raw_files {
			let stem = file_path
				.file_stem()
				.unwrap_or_default()
				.to_string_lossy()
				.to_string();
			let doc_node = DocumentNode {
				title: stem.clone(),
				relative_path: file_path.clone()
			};
			let parent_dir = file_path.parent().unwrap_or(Path::new(""));

			if parent_dir == Path::new("") {
				if let Some(dir_node) = workspace.directories.get_mut(&stem) {
					dir_node.container_document = Some(doc_node);
				} else {
					workspace.root_documents.insert(stem, doc_node);
				}
			} else if let Some(parent_node) = Self::get_dir_mut(&mut workspace, parent_dir) {
				if let Some(child_dir) = parent_node.subdirectories.get_mut(&stem) {
					child_dir.container_document = Some(doc_node);
				} else {
					parent_node.documents.insert(stem, doc_node);
				}
			}
		}

		workspace
	}

	fn insert_directory(workspace: &mut WorkspaceNode, rel_path: &Path) {
		let components: Vec<_> = rel_path
			.components()
			.map(|c| c.as_os_str().to_string_lossy().to_string())
			.collect();

		if components.is_empty() {
			return
		}

		let first = &components[0];
		let root_dir = workspace
			.directories
			.entry(first.clone())
			.or_insert_with(|| DirectoryNode {
				name: first.clone(),
				relative_path: PathBuf::from(first),
				container_document: None,
				subdirectories: BTreeMap::new(),
				documents: BTreeMap::new()
			});
		
		let mut current = root_dir;
		let mut acc_path = PathBuf::from(first);

		for part in &components[1..] {
			acc_path.push(part);
			current = current
				.subdirectories
				.entry(part.clone())
				.or_insert_with(|| DirectoryNode { 
					name: part.clone(), 
					relative_path: acc_path.clone(), 
					container_document: None, 
					subdirectories: BTreeMap::new(), 
					documents: BTreeMap::new()
				});
		}
	}

	fn get_dir_mut<'a>(
		workspace: &'a mut WorkspaceNode,
		rel_path: &Path
	) -> Option<&'a mut DirectoryNode> {
		let mut components = rel_path.components().map(|c| c.as_os_str().to_str());
		let first = components.next()??;
		let mut current = workspace.directories.get_mut(first)?;

		for part in components {
			let p = part?;
			current = current.subdirectories.get_mut(p)?;
		}

		Some(current)
	}
}