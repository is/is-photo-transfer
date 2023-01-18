use std::path::Path;
use std::string::String;
use lazy_static::lazy_static;
use regex::Regex;

// fn split_path(path: &str) -> Option<(&OsStr, &OsStr, &OsStr)> {
//     let path = Path::new(path);
//     Some((path.parent()?.as_os_str(), path.file_stem()?, path.extension()?))
// }

fn split_path_2(path: &str) -> Option<(&str, &str, &str)> {
    let path = Path::new(path);
    Some((path.parent()?.as_os_str().to_str()?,
          path.file_stem()?.to_str()?,
          path.extension()?.to_str()?))
}


pub struct MetaCore {
    pub model: String,
    pub datetime: String,
    pub number: String,
    pub ext: String,
}


pub enum FileMeta {
    V1(MetaCore),
    V2(MetaCore),
}

lazy_static! {
    static ref FILE_NAME_PATTERN_V1: Regex = Regex::new(r"(\D)_(\d{5})__(\d{8}_\d{6})").unwrap();
    static ref FILE_NAME_PATTERN_V2: Regex = Regex::new(r"(\d{8}_\d{6})__(\d{2,5})_(.{1,})").unwrap();
}

impl FileMeta {
    pub fn from_path(path:&str) -> Option<FileMeta> {
        let (_dir, file_stem, file_ext) = split_path_2(path)?;

        if let Some(captures) = FILE_NAME_PATTERN_V1.captures(file_stem) {
            return Some(FileMeta::V1(MetaCore{
                model: captures.get(1)?.as_str().to_string(),
                datetime: captures.get(3)?.as_str().to_string(),
                number: captures.get(2)?.as_str().to_string(),
                ext: file_ext.to_uppercase(),
            }));
        }

        if let Some(captures) = FILE_NAME_PATTERN_V2.captures(file_stem) {
            return Some(FileMeta::V2(MetaCore {
                model: captures.get(3)?.as_str().to_string(),
                datetime: captures.get(1)?.as_str().to_string(),
                number: captures.get(2)?.as_str().to_string(),
                ext: file_ext.to_uppercase(),
            }));
        }
        None
    }

    pub fn from_exif(path:&str) -> Option<FileMeta> {
        None
    }
}


#[cfg(test)]
mod tests {
    use std::ffi::OsStr;
    use std::path::{Path, PathBuf};
    use hex_literal::hex;
    use regex::Regex;
    use sha2::{Sha256, Sha512, Digest};

    static FILE_NAME_1:&str = "A_02104__20230105_150108.arw";
    static FILE_NAME_2:&str = "20230105_150108__03212_A7R4.arw";


    #[test]
    fn test_hash_sha2() {
        let mut hasher = sha2::Sha256::new();
        hasher.update(b"hello world");
        let digest = hasher.finalize();
        assert_eq!(digest[..], hex!("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9")[..]);
    }

    #[test]
    fn test_path_buf() {
        let path_buf = PathBuf::from("/home/admin/.config");

        assert_eq!("/home/admin", path_buf.parent().and_then(Path::to_str).unwrap());
        assert_eq!(".config", path_buf.file_name().and_then(OsStr::to_str).unwrap());

        let path_buf = PathBuf::from("HELLO.ME.WORLD");
        assert!(path_buf.parent().and_then(Path::to_str).unwrap().is_empty());
        assert_eq!("WORLD", path_buf.as_path().extension().unwrap().to_str().unwrap());
        assert_eq!("HELLO.ME", path_buf.as_path().file_stem().unwrap().to_str().unwrap());
    }

    #[test]
    fn test_split_path() {
        let (p0, p1, p2) =
            super::split_path_2("A_02104__20230105_150108.ARW").unwrap();
        assert_eq!("", p0);
        assert_eq!("A_02104__20230105_150108", p1);
        assert_eq!("ARW", p2);


        let (p0, p1, p2) =
            super::split_path_2("ABC/A_02104__20230105_150108.ARW").unwrap();
        assert_eq!("ABC", p0);
        assert_eq!("A_02104__20230105_150108", p1);
        assert_eq!("ARW", p2);
    }


    #[test]
    fn test_regex_0() {
        let name = "A_02104__20230105_150108";
        let r0 = Regex::new(r"(\D)_(\d{5})__(\d{8}_\d{6})").unwrap();
        let captures = r0.captures(name).unwrap();
        assert_eq!("A_02104__20230105_150108", captures.get(0).unwrap().as_str());
        assert_eq!("A", captures.get(1).unwrap().as_str());
        assert_eq!("02104", captures.get(2).unwrap().as_str());
        assert_eq!("20230105_150108", captures.get(3).unwrap().as_str());
    }
}