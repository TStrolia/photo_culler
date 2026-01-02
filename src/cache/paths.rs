use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

pub fn get_thumb_path(original_path: &Path, temp_dir: &Path) -> PathBuf {
    let mut hasher = DefaultHasher::new();
    original_path.hash(&mut hasher);
    let hash = hasher.finish();

    let filename = format!("{}.jpg", hash);
    temp_dir.join(filename)
}