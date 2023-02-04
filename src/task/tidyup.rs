use std::path::Path;

use crate::core::scandir::{scan as scan_dir, DirEntry};

use clap::Parser;

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
    #[arg(short, long, default_value_t = false)]
    usemove: bool,
    #[arg(short, long, default_value_t = true)]
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

    fn file(&self, _dir: &Path, _dest: &Path, _entry: &DirEntry, 
        _level: i32, _order: i32) {
        
    }
}
