use std::path::PathBuf;

pub struct Request {
    pub source:PathBuf,
    pub dest:PathBuf,
}

pub struct Response {
}

#[derive(thiserror::Error, Debug)]
pub enum ImportError {
    #[error("io-error {0}: {1}")]
    Io(String, String),
}

pub fn import(req:&mut Request) -> Result<Response, ImportError> {
    println!("THIS IS import ACTION");

    let src_glob = format!("{}/**/*.ARW", req.source.to_str().unwrap());
    println!("{}", src_glob);


    
    for entry in glob::glob(&src_glob).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let m = super::super::fninfo::from(path.to_str().unwrap()).unwrap();
                println!("{} -> {}", path.to_str().unwrap(), m.to_name());
            },
            Err(_) => (),
        }
    }
    Err(ImportError::Io("import".into(), "final".into()))
}