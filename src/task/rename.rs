use std::{path::Path, collections::HashMap};

use walkdir::{WalkDir, DirEntry};

#[derive(thiserror::Error, Debug)]
pub enum RenameError {
    #[error("io-error {0}: {1}")]
    Io(String, String),
}

impl From<walkdir::Error> for RenameError {
    fn from(i: walkdir::Error) -> Self {
        Self::Io(String::from("walkdir"), i.to_string())
    }
}

impl From<crate::core::fninfo::InfoErr> for RenameError {
    fn from(e: crate::core::fninfo::InfoErr) -> Self {
        Self::Io(String::from("metainfo"),
            e.to_string())
    }
}

pub struct Request {
    pub dir: String,
    pub exif: bool,
    pub dry: bool,
}

impl Request {
    pub fn from(item: &crate::RenameCommand) -> Self {
        Request {
            dir: item.dir.clone(),
            exif: item.exif,
            dry: item.dry,
        }
    }
}

type _Error = RenameError;

fn do_walk<T: AsRef<Path>>(req: &Request, level: i32, dir: T) -> Result<(), RenameError> {
    let dir = dir.as_ref();
    let mut files:Vec<DirEntry> = Vec::new();
    let mut dirs:Vec<DirEntry> = Vec::new();
    let walker = WalkDir::new(dir).max_depth(1).min_depth(1).sort_by_file_name();

    for entry in walker {
        if let Ok(e) = entry {
            // println!("{level} - {}", e.path().to_str().unwrap());
            if e.file_type().is_dir() {
                // do_walk(req, level + 1, e.path())?
                if e.path().file_name().unwrap() != "preview" {
                    dirs.push(e)
                }
            } else {
                files.push(e)
            }
        }
    }

    // scan subdirectory
    for entry in dirs {
        println!("{} dirs - {}", level, entry.path().to_str().unwrap());
        do_walk(req, level + 1, entry.path())?;
    }

    let mut name_map : HashMap<String, String> = HashMap::new();
    for entry in &files {
        let path = entry.path();
        let full_path = path.to_str().unwrap().to_string();
        let _file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        let file_stem = path.file_stem().unwrap().to_str().unwrap().to_string();
        let file_ext = path.extension();

        if file_ext.is_none() {
            continue;
        }
        let file_ext = file_ext.unwrap().to_str().unwrap().to_string();
        let file_ext_lower = file_ext.to_ascii_lowercase();

        let is_img = crate::utils::is_img_ext(file_ext_lower);

        if !is_img {
            println!("{level} - {full_path:?} - NO.IMG");
            continue
        }
        
        let meta = crate::core::fninfo::from(&full_path);
        if meta.is_err() {
            println!("{level} - {full_path:?} - MISS.META");
            continue;
        }

        let meta = meta.unwrap();
        let meta_name = meta.to_name();
        println!("{level} - {full_path:?} -> {}.{}",
            meta_name, file_ext);

        name_map.insert(file_stem, meta_name);
    }
    do_rename_files(req, level, dir, &files, &name_map);

    let preview = dir.join("preview");
    if preview.is_dir() {
        let preview_dir = preview.as_path();
    
        let walker = WalkDir::new(preview_dir).max_depth(1).min_depth(1);
        let walker = walker.sort_by_file_name();
    
        let pfiles: Vec<DirEntry> = walker.into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .collect();
        
        do_rename_files(req, level, preview_dir, &pfiles, &name_map);
    }
    Ok(())
}

fn do_rename_files(_req: &Request, _level: i32, dir: &Path, 
    files:& Vec<DirEntry>,
    map: &HashMap<String, String>) {
    let base_dir = dir.to_str().unwrap();

    for entry in files {
        let path = entry.path();
        let file_path = path.to_str().unwrap();
        let file_stem = path.file_stem().unwrap().to_str().unwrap().to_string();
        let file_ext = path.extension();
        if file_ext.is_none() {
            break
        }
        let file_ext = file_ext.unwrap().to_str().unwrap();

        match map.get(&file_stem) {
            Some(r) => {
                let new_fn = format!("{base_dir}/{r}.{file_ext}");
                println!("RENAME {file_path} -> {new_fn}")
            },
            None => (),
        }
    }
}

pub fn rename(req: &Request) -> Result<(), RenameError> {
    do_walk(req, 0, &req.dir)
}
