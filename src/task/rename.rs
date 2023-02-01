use std::{collections::HashMap, path::Path};

use walkdir::{DirEntry, WalkDir};

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
        Self::Io(String::from("metainfo"), e.to_string())
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
    let (files, dirs) = scan_dir(dir);

    // scan subdirectory
    for entry in dirs {
        println!("{} dirs - {}", level, entry.path().to_str().unwrap());
        do_walk(req, level + 1, entry.path())?;
    }

    let name_map: HashMap<String, String> = build_rename_map(req, level, dir, &files);
    do_rename_files(req, level, dir, &files, &name_map);

    let preview = dir.join("preview");
    if preview.is_dir() {
        let preview_dir = preview.as_path();
        let (pfiles, _) = scan_dir(preview_dir);
        do_rename_files(req, level, preview_dir, &pfiles, &name_map);
    }
    Ok(())
}

fn scan_dir(dir: &Path) -> (Vec<DirEntry>, Vec<DirEntry>) {
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

// fn scan_dir_only_files(dir: &Path) -> Vec<DirEntry> {
//     let walker = WalkDir::new(dir).max_depth(1).min_depth(1);
//     let walker = walker.sort_by_file_name();
//     walker
//         .into_iter()
//         .filter_map(|e| e.ok())
//         .filter(|e| e.file_type().is_file())
//         .collect()
// }

fn build_rename_map(
    _req: &Request,
    level: i32,
    _dir: &Path,
    files: &Vec<DirEntry>,
) -> HashMap<String, String> {
    let mut name_map: HashMap<String, String> = HashMap::new();
    for entry in files {
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
            continue;
        }

        let meta = crate::core::fninfo::from(&full_path);
        if meta.is_err() {
            println!("{level} - {full_path:?} - MISS.META");
            continue;
        }

        let meta = meta.unwrap();
        let meta_name = meta.to_name();
        
        if !file_stem.eq(&meta_name) {
            println!("{level} - {full_path:?} -> {}.{}", meta_name, file_ext);
            name_map.insert(file_stem, meta_name);
        } else {
            println!("{level} - {full_path:?} -> HOLD")
        }
    }
    name_map
}

fn do_rename_files(
    _req: &Request,
    _level: i32,
    dir: &Path,
    files: &Vec<DirEntry>,
    map: &HashMap<String, String>)
{
    if map.len() == 0 || files.len() == 0 {
        return
    }

    let base_dir = dir.to_str().unwrap();

    for entry in files {
        let path = entry.path();
        let file_path = path.to_str().unwrap();
        let file_stem = path.file_stem().unwrap().to_str().unwrap().to_string();
        let file_ext = path.extension();
        if file_ext.is_none() {
            break;
        }
        let file_ext = file_ext.unwrap().to_str().unwrap();

        match map.get(&file_stem) {
            Some(r) => {
                let new_fn = format!("{base_dir}/{r}.{file_ext}");
                println!("RENAME {file_path} -> {new_fn}")
            }
            None => (),
        }
    }
}

pub fn rename(req: &Request) -> Result<(), RenameError> {
    do_walk(req, 0, &req.dir)
}
