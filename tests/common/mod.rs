pub mod current_dir;

use std::fs;
use std::path::{Path, PathBuf};

pub fn collect_files(root: &Path, files: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(root).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            collect_files(&path, files);
        } else {
            files.push(path);
        }
    }
}
