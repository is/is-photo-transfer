use std::path::Path;

use walkdir::WalkDir;

pub use walkdir::DirEntry;

pub fn scan(dir: &Path) -> (Vec<DirEntry>, Vec<DirEntry>) {
    let mut files: Vec<DirEntry> = Vec::new();
    let mut dirs: Vec<DirEntry> = Vec::new();
    let walker = WalkDir::new(dir)
        .max_depth(1)
        .min_depth(1)
        .sort_by_file_name();

    for entry in walker {
        if let Ok(e) = entry {
            if e.file_type().is_dir() {
                if e.path().file_name().unwrap() != "preview" {
                    dirs.push(e)
                }
            } else {
                files.push(e)
            }
        }
    }
    (files, dirs)
}
