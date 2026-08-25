use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub struct SystemIO;

impl SystemIO {
	pub fn normalize_canonical_path<P: AsRef<Path>>(path: P) -> io::Result<PathBuf> {
		let canonical = path.as_ref().canonicalize()?;
		#[cfg(windows)]
		{
			let path_str = canonical.to_string_lossy();
			if let Some(stripped) = path_str.strip_prefix(r"\\?\") {
				return Ok(PathBuf::from(stripped));
			}
		}
		Ok(canonical)
	}
	
	pub fn collect_markdown_files(root: &Path) -> io::Result<Vec<PathBuf>> {
		let mut files = Vec::new();
		Self::walk(root, root, &mut files)?;
		Ok(files)
	}
	
	fn walk(base: &Path, current: &Path, acc: &mut Vec<PathBuf>) -> io::Result<()> {
		for entry in fs::read_dir(current)? {
			let entry = entry?;
			let path = entry.path();
			let rel = match path.strip_prefix(base) {
				Ok(p) => p.to_path_buf(),
				Err(_) => continue,
			};
			if rel
				.components()
				.any(|c| c.as_os_str().to_string_lossy().starts_with('.'))
			{
				continue;
			}
			if path.is_dir() {
				Self::walk(base, &path, acc)?;
			} else if path.extension().and_then(|s| s.to_str()) == Some("md") {
				acc.push(rel);
			}
		}
		Ok(())
	}
}
