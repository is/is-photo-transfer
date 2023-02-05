use std::{path::Path, time::Instant};

use crate::core::{
    scandir::{scan as scan_dir, DirEntry},
    utils,
};

use clap::{ArgAction, Parser};

use crate::cmd::{Cmd, CmdResult};

// ==== COMMAND ====
#[derive(Parser, Debug)]
pub struct TidyupCommand {
    #[arg(default_value = ".")]
    source: String,
    #[arg(default_value = None)]
    dest: Option<String>,
    #[arg(short, long, default_value_t = false)]
    exif: bool,
    #[arg(short, long, default_value_t = false)]
    dry: bool,
    #[arg(short, long, default_value_t = false)]
    compact: bool,
    #[arg(short, long, default_value_t = false)]
    touch: bool,
    #[arg(short = 'D', long, default_value_t = false)]
    docopy: bool,
    #[arg(short, long = "no-year", default_value_t = true)]
    #[arg(action=ArgAction::SetFalse)]
    year: bool,
}

impl Cmd for TidyupCommand {
    fn run(self) -> CmdResult {
        let mut task = Task { cmd: self };

        task.run();
        Ok(())
    }
}

// ==== TASK ====
struct Task {
    cmd: TidyupCommand,
}

impl Task {
    fn run(&mut self) {
        let src = self.cmd.source.clone();
        // let dest = self.cmd.dest.clone()
        //     .unwrap_or_else(|| src.clone());
        let dest = self.cmd.dest.as_ref().unwrap_or(&src).clone();

        self.dir(Path::new(&src), Path::new(&dest), 0);
    }

    fn dir(&self, dir: &Path, dest: &Path, level: i32) {
        let (files, dirs) = scan_dir(dir);

        for e in &dirs {
            Self::dir(self, e.path(), dest, level + 1);
        }

        let mut file_num = files.len() as i32;
        for f in &files {
            self.file(dir, dest, f, level, file_num);
            file_num = file_num - 1;
        }
    }

    fn file(&self, _dir: &Path, dest: &Path, entry: &DirEntry, _level: i32, order: i32) {
        let path = entry.path();
        let path_str = path.to_str().unwrap();
        let msg_head = format!("F,{},{}", order, path_str);

        let file_ext = if let Some(ext) = path.extension() {
            ext.to_ascii_uppercase()
        } else {
            return;
        };

        let file_ext = file_ext.to_str().unwrap().to_string();
        if !utils::is_img_ext(file_ext.to_ascii_lowercase()) {
            return;
        }

        let start = Instant::now();

        let meta_res = crate::core::fninfo::from(path_str);
        if meta_res.is_err() {
            println!("{msg_head},ERR");
            return;
        }

        let meta = meta_res.unwrap();
        let meta = if self.cmd.exif {
            meta.update_from_exif(path_str)
        } else {
            meta
        };

        let cmd = &self.cmd;
        let date_str = meta.to_date();

        let new_name = if cmd.compact {
            meta.to_compact_name()
        } else {
            meta.to_name()
        };

        let dest_str = if cmd.year {
            let year_str = date_str[0..4].to_string();
            format!("{year_str}/{date_str}/{new_name}.{}", meta.ext)
        } else {
            format!("{date_str}/{new_name}.{}", meta.ext)
        };

        let dest_path = dest.join(&dest_str);
        let full_dest = dest_path.to_str().unwrap();

        if cmd.dry {
            println!("{msg_head},OK,{full_dest},{}", start.elapsed().as_millis());
            return;
        }

        if dest_path.is_file() {
            println!(
                "{msg_head},SKIP,{full_dest},{}",
                start.elapsed().as_millis()
            );
            return;
        }

        if let Some(parent) = dest_path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent).unwrap();
                if cmd.touch {
                    crate::core::touch::touch_form_0(parent.to_str().unwrap(), &date_str).unwrap();
                }
            }
        }

        let err = if cmd.docopy {
            let s = std::fs::copy(path, &dest_path);
            if let Err(e) = s {
                println!("{:?}", e);
                true
            } else {
                false
            }
        } else {
            std::fs::rename(path, &dest_str).is_err()
        };

        if err {
            println!("{msg_head},ERROR,{full_dest}");
            return;
        };

        if cmd.touch {
            crate::core::touch::touch(&dest_str, meta.to_systemtime()).unwrap();
        }
        println!("{msg_head},OK,{full_dest},{}", start.elapsed().as_millis());
    }
}