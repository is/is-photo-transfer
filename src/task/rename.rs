use std::{path::Path, collections::HashMap};

use walkdir::WalkDir;

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
    let mut files:Vec<walkdir::DirEntry> = Vec::new();
    let mut dirs:Vec<walkdir::DirEntry> = Vec::new();

    for entry in WalkDir::new(dir.as_ref()).max_depth(1).min_depth(1) {
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

    for entry in dirs {
        println!("{} dirs - {}", level, entry.path().to_str().unwrap());
        do_walk(req, level + 1, entry.path())?;
    }

    let mut name_map : HashMap<String, String> = HashMap::new();
    
    for entry in files {
        let path = entry.path();
        let full_path = path.to_str().unwrap().to_string();
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
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
        println!("{level} - {full_path:?} -> {}.{}",
            meta.to_name(), file_ext)
        
    }
    Ok(())
}

pub fn rename(req: &Request) -> Result<(), RenameError> {
    do_walk(req, 0, &req.dir)
}
