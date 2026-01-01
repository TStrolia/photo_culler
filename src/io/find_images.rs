use std::{fs, path::{Path, PathBuf}};

pub fn find_images(dir: &Path) -> Vec<PathBuf> {
    let mut images = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path =  entry.path();

            if let Some(extension) = path.extension() {
                if let Some(ext_str) = extension.to_str() {
                    let ext = ext_str.to_lowercase();
                    if ext == "jpg" || ext == "jpeg" || ext == "png" {
                        images.push(path);
                    }
                }
            }
        }
    }

    images.sort();

    images
}