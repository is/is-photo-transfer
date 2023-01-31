use std::{path::Path, collections::HashMap};

use walkdir::WalkDir;

#[derive(thiserror::Error, Debug)]
pub enum RenameError {
    #[error("io-error {0}: {1}")]
    Io(String, String),
    #[error("OK")]
    Ok,
}

impl From<walkdir::Error> for RenameError {
    fn from(i: walkdir::Error) -> Self {
        RenameError::Io(String::from("walkdir"), i.to_string())
    }
}

pub struct Request {
    pub dir: String,
    pub dry: bool,
}

impl Request {
    pub fn from(item: &crate::RenameCommand) -> Self {
        Request {
            dir: item.dir.clone(),
            dry: item.dry.clone(),
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
        println!("{} files - {}", level, entry.path().to_str().unwrap());
    }
    Ok(())
}

pub fn rename(req: &Request) -> Result<(), RenameError> {
    do_walk(req, 0, &req.dir)
}
