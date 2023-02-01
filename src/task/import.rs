use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

use crate::core::fninfo;

pub struct Request {
    pub source: PathBuf,
    pub dest: PathBuf,
    pub compact: bool,
}

pub struct Response {}

#[derive(thiserror::Error, Debug)]
pub enum ImportError {
    #[error("io-error {0}: {1}")]
    Io(String, String),
    #[error("OK")]
    Ok,
}

type E = ImportError;
type R<T> = Result<T, E>;

fn io_error(why: String, who: String) -> E {
    E::Io(why, who)
}

pub struct Task<'a> {
    request: &'a mut Request,
}

impl<'a> Task<'a> {
    pub fn copy<S: AsRef<Path>>(&mut self, src: S) -> R<u64> {
        let src = src.as_ref();
        let start = Instant::now();
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
            crate::core::touch::touch_form_0(&dest_dir_str, &date_str).unwrap();
        }
        let dest = Path::new(&dest_str);
        if !dest.is_file() {
            let r = fs::copy(src, &dest);
            let metadata = fs::metadata(&src_str).unwrap();
            crate::core::touch::touch_form_1(&dest_str, metadata.created().unwrap()).unwrap();
            println!(
                "{src_str} -> {dest_str}  _  {:.2}s",
                start.elapsed().as_secs_f32()
            );
            r.map_err(|_| io_error("copy".to_string(), src_str.to_string()))
        } else {
            println!("{src_str} -> {dest_str}  _  skip");
            Ok(999999999)
        }
    }

    pub fn run(&mut self) -> Result<Response, ImportError> {
        let src = &self.request.source;
        let src_dir = src.to_str().unwrap();

        let src_pattern = if src_dir.ends_with("/DCIM") {
            format!("{}/**/*.ARW", src_dir)
        } else if src.join("DCIM").is_dir() {
            format!("{}/DCIM/**/*.ARW", src_dir)
        } else {
            format!("{}/*.ARW", src_dir)
        };

        let src_dir = src_dir.to_string();

        for entry in glob::glob(&src_pattern).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    self.copy(&path)?;
                }
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }

        println!("{}: {}", src_dir, src_pattern);

        Err(ImportError::Ok)
    }
}

pub fn import(request: &mut Request) -> Result<Response, ImportError> {
    println!("THIS IS import ACTION");
    let mut task = Task { request };
    task.run()
}
