use lazy_static::lazy_static;
use regex::Regex;

pub fn env_var(key: &str) -> Option<String> {
    std::env::var_os(key).map(|e| e.into_string().unwrap())
}

lazy_static! {
    static ref IMG_PEXT: Regex = Regex::new(r"jpeg|jpg|heif|heic|arw").unwrap();
}

pub fn is_img_ext<T: AsRef<str>>(ext: T) -> bool {
    IMG_PEXT.is_match(ext.as_ref())
}
