use std::fs::{self};
use std::path::{Path, PathBuf};
use std::time::Instant;

use clap::Parser;

use crate::cmd::{Cmd, CmdResult};
use crate::core::{fninfo, utils};
use crate::core::progress::Progress;

use kdam::{term, term::Colorizer, tqdm, BarExt, Column, RichProgress, Spinner};
use std::io::{stderr, IsTerminal};

// ==== COMMAND ====
#[derive(Parser)]
pub struct ImportCommand {
    source: Option<String>,
    dest: Option<String>,
    #[arg(long, default_value_t=String::from("mac"))]
    host: String,
    #[arg(long, short, default_value_t = false)]
    compact: bool,
    #[arg(help = "disable touch file timestamp.")]
    #[arg(short, long = "no-touch", default_value_t = true)]
    #[arg(action=clap::ArgAction::SetFalse)]
    touch: bool,
    #[arg(long, short, default_value_t = false)]
    rename: bool,
}


fn cmd_import_source_dir(cmd: &ImportCommand) -> String {
    match cmd.source.as_ref() {
        Some(s) => s.clone(),
        #[rustfmt::skip]
        _ => match cmd.host.as_str() {
            "mac" => "/Volumns/Untitled",
            _ => "/Volumns/Untitled",
        }.to_string(),
    }
}


fn cmd_import_dest_dir(cmd: &ImportCommand) -> String {
    if let Some(s) = cmd.dest.as_ref() {
        s.clone()
    } else {
        let home = utils::env_var("HOME").unwrap();
        match cmd.host.as_str() {
            // "mac" => format!("{home}/PI"),
            // "hi" => format!("{home}/PI"),
            // "mi2" => format!("{home}/PI"),
            _ => format!("{home}/PI"),
        }
    }
}


impl Cmd for ImportCommand {
    fn run(self) -> CmdResult {
        let cmd = &self;
        let source = PathBuf::from(cmd_import_source_dir(cmd));
        let dest = PathBuf::from(cmd_import_dest_dir(cmd));
        // let compact = cmd.compact;
        // let touch = cmd.touch;

        println!("name:{:?}, source:{:?}, dest:{:?}", cmd.host, source, dest);
        let mut req = Request {
            source,
            dest,
            compact: cmd.compact,
            touch: cmd.touch,
            rename: cmd.rename,
        };
        do_import(&mut req)?;
        Ok(())
    }
}

// ==== TASK ====
pub struct Request {
    pub source: PathBuf,
    pub dest: PathBuf,
    pub compact: bool,
    pub touch: bool,
    pub rename: bool,
}

pub struct Response {}

#[derive(thiserror::Error, Debug)]
pub enum ImportError {
    #[error("io-error {0}: {1}")]
    Io(String, String),
}

type E = ImportError;
type R<T> = Result<T, E>;

fn io_error(why: String, who: String) -> E {
    E::Io(why, who)
}

pub struct Task<'a> {
    request: &'a mut Request,
}

fn glob_ex(base_dir:&str) -> Vec<PathBuf> {
    let patterns = vec!(
        base_dir.to_owned() + "/*.ARW",
        base_dir.to_owned() + "/*.NEF");
    
    return patterns.iter()
        .map(|x| glob::glob(x))
        .map(|x| x.unwrap()).flatten()
        .map(|x| x.unwrap())
        .collect();
}

impl<'a> Task<'a> {
    pub fn copy<S: AsRef<Path>>(&mut self, src: S, prog:&mut Progress) -> R<u64> {
        let src = src.as_ref();
        let _start = Instant::now();
        let src_str = src.to_str().unwrap();
        let dest_root_str = self.request.dest.to_str().unwrap().to_string();

        let info = fninfo::from(src_str).unwrap();
        let date_str = info.datetime[0..8].to_string();

        let (dest_dir_str, dest_str) = if self.request.compact {
            info.to_compact_dir_and_full(&dest_root_str)
        } else {
            info.to_dir_and_full(&dest_root_str)
        };

        let dest_dir = Path::new(&dest_dir_str);
        if !dest_dir.is_dir() {
            fs::create_dir_all(dest_dir)
                .map_err(|_| io_error("create-dir".to_string(), dest_dir_str.clone()))?;
            if self.request.touch {
                crate::core::touch::touch_form_0(&dest_dir_str, &date_str).unwrap();
            }
        }
        let dest = Path::new(&dest_str);
        if !dest.is_file() {
            let r = if !self.request.rename {
                fs::copy(src, &dest)
            } else {
                fs::rename(src, &dest).map(|_| 0)
            };

            // println!("SRC->{src_str}");
            if self.request.touch && !self.request.rename {
                let metadata = fs::metadata(&src_str).unwrap();
                crate::core::touch::touch(&dest_str, metadata.created().unwrap()).unwrap();
            }
            
            prog.inc();
            // println!(
            //     "{}/{} {src_str} -> {dest_str}  _  {:.2}s",
            //     prog.cur, prog.total,
            //     start.elapsed().as_secs_f32(),
            // );
            r.map_err(|_| io_error("copy".to_string(), src_str.to_string()))
        } else {
            prog.inc();
            // println!("{src_str} -> {dest_str}  _  skip");
            Ok(999999999)
        }
    }

    pub fn run(&mut self) -> Result<Response, ImportError> {
        let src = &self.request.source;
        let src_dir = src.to_str().unwrap();

        let src_pattern = if src_dir.ends_with("/DCIM") {
            format!("{}/**", src_dir)
        } else if src.join("DCIM").is_dir() {
            format!("{}/DCIM/**", src_dir)
        } else {
            format!("{}", src_dir)
        };
    
        println!("pattern: {}", src_pattern);
        let src_dir = src_dir.to_string();

        let files = glob_ex(src_pattern.as_str());


        println!(
            "[IMPORT] {} to {}, {} photos",
            src_dir,
            self.request.dest.to_str().unwrap(),
            files.len()
        );

        term::init(stderr().is_terminal());
        term::hide_cursor().unwrap();

        let mut prog = Progress::new(files.len());
        let mut pb = RichProgress::new(
            tqdm!(
                total = prog.total,
                unit_scale = true,
                unit_divisor = 1
                // unit = "F"
            ),
            vec![
                Column::Spinner(Spinner::new(
                    &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
                    200.0,
                    1.0,
                )),
                Column::Text("[bold blue] & ".to_owned()),
                Column::Animation,
                Column::Percentage(1),
                Column::Text("•".to_owned()),
                Column::CountTotal,
                Column::Text("•".to_owned()),
                // Column::Rate,
                // Column::Text("•".to_owned()),
                Column::RemainingTime,
            ],
        );

        pb.replace(1, Column::Text("[bold blue]Import".to_owned()));
        
        for file in &files {
            self.copy(file, &mut prog)?;
            pb.update_to(prog.cur).unwrap();
        }
        term::show_cursor().unwrap();
        Ok(Response {})
    }
}

pub fn do_import(request: &mut Request) -> Result<Response, ImportError> {
    // println!("THIS IS import ACTION");
    let mut task = Task { request };
    task.run()
}
