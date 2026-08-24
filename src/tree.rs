use std::collections::BTreeMap;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceNode {
	pub name: String,
	pub root_path: PathBuf,
	pub directories: BTreeMap<String, DirectoryNode>,
	pub root_documents: BTreeMap<String, DocumentNode>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectoryNode {
	pub name: String,
	pub relative_path: PathBuf,
	pub container_document: Option<DocumentNode>,
	pub subdirectories: BTreeMap<String, DirectoryNode>,
	pub documents: BTreeMap<String, DocumentNode>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentNode {
	pub title: String,
	pub relative_path: PathBuf,
}