use std::path::PathBuf;

pub struct Request {
    pub source: PathBuf,
    pub dest: PathBuf,
}

pub struct Response {}

#[derive(thiserror::Error, Debug)]
pub enum ImportError {
    // #[error("io-error {0}: {1}")]
    // Io(String, String),
    #[error("OK")]
    Ok,
}

pub struct Task<'a> {
    request: &'a mut Request,
}

impl<'a> Task<'a> {
    pub fn run(&mut self) -> Result<Response, ImportError> {
        let req: &Request = self.request;
        let src_dir = req.source.to_str().unwrap();

        let src_pattern = if src_dir.ends_with("/DCIM") {
            format!("{}/**/*.ARW", src_dir)
        } else if req.source.join("DCIM").is_dir() {
            format!("{}/DCIM/**/*.ARW", src_dir)
        } else {
            format!("{}/*.ARW", src_dir)
        };

        for entry in glob::glob(&src_pattern).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => self.copy(&path),
                Err(e) => println!("{:?}", e),
            }
        }

        println!("{}: {}", src_dir, src_pattern);
        Err(ImportError::Ok)
    }

    pub fn copy(&self, src: &PathBuf) {
        let src = src.to_str().unwrap();
        let dst = self.request.dest.to_str().unwrap().to_string();

        let info = crate::fninfo::from(src).unwrap();
        let date_str = info.datetime[0..8].to_string();
        println!("{} -> {}/{}/{}", src, dst, date_str, info.to_file_name());
    }
}

pub fn import(request: &mut Request) -> Result<Response, ImportError> {
    println!("THIS IS import ACTION");
    let mut task = Task { request };
    task.run()
}
