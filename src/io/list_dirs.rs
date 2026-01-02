use std::{fs, path::{Path, PathBuf}};

pub fn list_dirs(path: &Path) -> Vec<PathBuf> {
    let mut dirs = Vec::new();

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    let path = entry.path();

                    let name = path.file_name()
                        .map(|n| n.to_string_lossy())
                        .unwrap_or_default();

                    if !name.starts_with(".") {
                        dirs.push(path);
                    }
                }
            }
        }
    }

    dirs.sort();
    dirs
}