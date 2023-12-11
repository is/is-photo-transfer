use std::{collections::HashMap, path::Path};

use clap::Parser;

use crate::cmd::{Cmd, CmdResult};
use crate::core::fninfo::Info;
use crate::core::scandir::{scan as scan_dir, DirEntry};

// ==== COMMAND ====
#[derive(Parser, Debug)]
pub struct Rename2Command {
    #[arg(default_value = ".")]
    dir: String,
    #[arg(short, long, default_value_t = false)]
    #[arg(help = "update information from exif")]
    exif: bool,
    #[arg(short, long, default_value_t = false)]
    #[arg(help = "show what would have been renamed")]
    dry: bool,
    #[arg(short, long, default_value_t = false)]
    #[arg(help = "rename in compact one mode")]
    compact: bool,
    #[arg(help = "disable touch file timestamp.")]
    #[arg(short, long = "no-touch", default_value_t = true)]
    #[arg(action=clap::ArgAction::SetFalse)]
    touch: bool,
}

impl Cmd for Rename2Command {
    fn run(self) -> CmdResult {
        do_rename(&Request::from(&self))?;
        Ok(())
    }
}

// ==== TASK ====

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
    pub compact: bool,
    pub touch: bool,
}

impl Request {
    pub fn from(cmd: &Rename2Command) -> Self {
        Request {
            dir: cmd.dir.clone(),
            exif: cmd.exif,
            dry: cmd.dry,
            compact: cmd.compact,
            touch: cmd.touch,
        }
    }
}

struct RenameEntry {
    name: String,
    meta: Info,
}

type _Error = RenameError;

fn walk<T: AsRef<Path>>(req: &Request, level: i32, dir: T) -> Result<(), RenameError> {
    let dir = dir.as_ref();
    let (files, dirs) = scan_dir(dir);

    // scan subdirectory
    for entry in dirs {
        println!("{} dirs - {}", level, entry.path().to_str().unwrap());
        walk(req, level + 1, entry.path())?;
    }

    let name_map: HashMap<String, RenameEntry> = build_rename_map(req, level, dir, &files);
    do_rename_files(req, level, dir, &files, &name_map, false);

    let preview = dir.join("preview");
    if preview.is_dir() {
        let preview_dir = preview.as_path();
        let (pfiles, _) = scan_dir(preview_dir);
        do_rename_files(req, level, preview_dir, &pfiles, &name_map, true);
    }
    Ok(())
}

fn build_rename_map(
    req: &Request,
    level: i32,
    _dir: &Path,
    files: &Vec<DirEntry>,
) -> HashMap<String, RenameEntry> {
    let mut name_map: HashMap<String, RenameEntry> = HashMap::new();
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

        let is_img = crate::core::utils::is_img_ext(file_ext_lower);

        if !is_img {
            println!("{level} - {full_path:?} - NO.IMG");
            continue;
        }

        let is_enhanced: bool = _file_name.contains("Enhanced-NR");
        if is_enhanced {
            println!("{level} - {full_path:?} - ENHANCED-NR");
            continue;
        }


        let meta: Result<Info, crate::core::fninfo::InfoErr> = crate::core::fninfo::from(&full_path);
        if meta.is_err() {
            println!("{level} - {full_path:?} - MISS.META");
            continue;
        }

        let meta = meta.unwrap();
        let meta = if req.exif {
            meta.update_from_exif(&full_path)
        } else {
            meta
        };

        let meta_name = meta.to_name();

        let meta_name = if req.compact {
            meta_name[9..].to_string()
        } else {
            meta_name
        };

        if !file_stem.eq(&meta_name) {
            println!("{level} - {full_path:?} -> {}.{}", meta_name, file_ext);
            name_map.insert(
                file_stem,
                RenameEntry {
                    name: meta_name,
                    meta,
                },
            );
        } else {
            println!("{level} - {full_path:?} -> HOLD")
        }
    }
    name_map
}

fn do_rename_files(
    req: &Request,
    _level: i32,
    dir: &Path,
    files: &Vec<DirEntry>,
    map: &HashMap<String, RenameEntry>,
    preview: bool,
) {
    if !preview && (map.len() == 0 || files.len() == 0) {
        return;
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
        // println!("file_stem: {file_stem}");
        let enhanced = file_stem.contains("Enhanced-NR");
        
        let file_stem2 = if enhanced && preview {
            file_stem.to_string().replace("-Enhanced-NR", "")
        } else {
            file_stem.to_string()
        };
    
        match map.get(&file_stem2) {
            Some(r) => {
                let name = &r.name;
                let new_fn = format!("{base_dir}/{name}.{file_ext}");
                println!("RENAME {file_path} -> {new_fn}");
                if !req.dry {
                    rename(req, file_path, &new_fn, &r.meta).expect("do_rename_faile");
                }
            }
            None => {
                if enhanced && preview {
                    let new_fn = format!("{base_dir}/{file_stem2}.{file_ext}");
                    println!("RENAME {file_path} -> {new_fn}");
                    let meta = crate::core::fninfo::from(file_path).unwrap();
                    if !req.dry {
                        rename(req, file_path, &new_fn, &meta).expect("do_rename_enhanced_failed");
                    }
                }
            }
        }
    }
}

fn rename(req: &Request, src: &str, dest: &str, meta: &Info) -> Result<(), std::io::Error> {
    std::fs::rename(src, dest)?;
    if !req.compact && req.touch {
        crate::core::touch::touch(dest, meta.to_systemtime())?
    };
    Ok(())
}

pub fn do_rename(req: &Request) -> Result<(), RenameError> {
    walk(req, 0, &req.dir)
}
