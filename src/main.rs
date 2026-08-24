mod sys;
mod tree;
mod scanner;

use scanner::VaultScanner;
use sys::SystemIO;
use std::{path::Path, println};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vault_path = Path::new("./vault");

	let canonical_root = SystemIO::normalize_canonical_path(vault_path)?;
	let raw_entries = SystemIO::collect_raw_entries(&canonical_root)?;
	let workspace_name = canonical_root
		.file_name()
		.unwrap_or_default()
		.to_string_lossy()
		.to_string();

	let workspace = VaultScanner::build_workspace(workspace_name, canonical_root, raw_entries);

	println!("Workspace Tree Built: {}", workspace.name);
	println!("Root Dirs: {}", workspace.directories.len());

	Ok(())
}
