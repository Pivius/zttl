use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub enum FsEntry {
	Directory { relative_path: PathBuf },
	File { relative_path: PathBuf }
}

pub struct SystemIO;

impl SystemIO {
	pub fn normalize_canonical_path<P: AsRef<Path>>(path: P) -> io::Result<PathBuf> {
		let canonical = path.as_ref().canonicalize()?;
		#[cfg(windows)]
		{
			let path_str = canonical.to_string_lossy();
			if path_str.starts_with(r"\\?\") {
				return Ok(PathBuf::from(&path_str[4..]));
			}
		}
		Ok(canonical)
	}

	pub fn collect_raw_entries(root: &Path) -> io::Result<Vec<FsEntry>> {
		let mut entries = Vec::new();
		Self::walk_recursive(root, root, &mut entries)?;
		Ok(entries)
	}

	fn walk_recursive(base: &Path, current: &Path, acc: &mut Vec<FsEntry>) -> io::Result<()> {
		for entry in fs::read_dir(current)? {
			let entry = entry?;
			let path = entry.path();
			let rel_path = match path.strip_prefix(base) {
				Ok(p) => p.to_path_buf(),
				Err(_) => continue
			};

			// Ignore hidden files
			if rel_path
				.components()
				.any(|c| c.as_os_str().to_string_lossy().starts_with('.'))
			{
				continue;
			}

			if path.is_dir() {
				acc.push(FsEntry::Directory {
					relative_path: rel_path.clone()
				});
				Self::walk_recursive(base, &path, acc)?;
			} else if path.extension().and_then(|s| s.to_str()) == Some("md") {
				acc.push(FsEntry::File { 
					relative_path: rel_path 
				});
			}
		}
		Ok(())
	}
} 