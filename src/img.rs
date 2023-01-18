use std::ffi::OsStr;
use std::path::Path;
use std::string::String;
use lazy_static::lazy_static;
use regex::Regex;

fn split_path(path: &str) -> Option<(&OsStr, &OsStr, &OsStr)> {
    let path = Path::new(path);
    Some((path.parent()?.as_os_str(), path.file_stem()?, path.extension()?))
}

#[derive(Debug, PartialEq)]
pub enum PatternVersion {
    UNKNOWN,
    V1,
    V2,
}


pub struct ImageMeta {
    pub model: String,
    pub datetime: String,
    pub number: String,
    pub ext: String,
    pub version: PatternVersion,
}


lazy_static! {
    static ref FILE_NAME_PATTERN_V1: Regex = Regex::new(r"(\D)_(\d{5})__(\d{8}_\d{6})").unwrap();
    static ref FILE_NAME_PATTERN_V2: Regex = Regex::new(r"(\d{8}_\d{6})__(\d{2,5})_(.{1,})").unwrap();
}

fn get_image_meta_from_file_name(file_stem: &str, file_ext: &str) -> Option<ImageMeta>{
    if let Some(captures) =  FILE_NAME_PATTERN_V1.captures(file_stem) {
        Some(ImageMeta {
            model: captures.get(1)?.as_str().to_string(),
            datetime: captures.get(3)?.as_str().to_string(),
            number: captures.get(2)?.as_str().to_string(),
            ext: file_ext.to_uppercase(),
            version: PatternVersion::V1,
        })
    } else if let Some(captures) = FILE_NAME_PATTERN_V2.captures(file_stem) {
        Some(ImageMeta {
            model: captures.get(3)?.as_str().to_string(),
            datetime: captures.get(1)?.as_str().to_string(),
            number: captures.get(2)?.as_str().to_string(),
            ext: file_ext.to_uppercase(),
            version: PatternVersion::V2,
        })
    } else {
        None
    }
}


fn get_image_meta_from_exif(
    full_path: &str, file_stem: &str, file_ext: &str) -> Option<ImageMeta>{
    None
}


impl ImageMeta {
    fn from_file(full_path:&str) -> Option<Self> {
        let (_dir, file_stem, file_ext) = split_path(full_path)?;

        let file_stem = file_stem.to_str()?;
        let file_ext= file_ext.to_str()?;

        get_image_meta_from_file_name(file_stem, file_ext)
            .or(get_image_meta_from_exif(full_path, file_stem, file_ext))
    }
}


#[cfg(test)]
mod tests {
    use std::ffi::OsStr;
    use std::path::{Path, PathBuf};
    use regex::Regex;
    use crate::img::{ImageMeta, PatternVersion};


    #[test]
    fn test_image_meta_from_file() {
        let path = "A_02104__20230105_150108.arw";
        let meta = ImageMeta::from_file(path);
        assert!(!meta.is_none());
        assert_eq!(meta.as_ref().unwrap().version, PatternVersion::V1);

        match &meta {
            Some(m) => {
                assert_eq!(m.ext, "ARW");
                assert_eq!(m.version, PatternVersion::V1);
                assert_eq!(m.datetime, "20230105_150108");
                assert_eq!(m.model, "A");
            }
            None => assert!(false)
        };

        let path = "20230105_150108__03212_A7R4.arw";
        let meta = ImageMeta::from_file(path).unwrap();

        match meta.version {
            PatternVersion::V2  => {
                assert_eq!(meta.datetime, "20230105_150108");
                assert_eq!(meta.ext, "ARW");
                assert_eq!(meta.model, "A7R4");
            }
            _ => assert!(false)
        }
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
            super::split_path("A_02104__20230105_150108.ARW").unwrap();
        assert_eq!("", p0);
        assert_eq!("A_02104__20230105_150108", p1);
        assert_eq!("ARW", p2);


        let (p0, p1, p2) =
            super::split_path("ABC/A_02104__20230105_150108.ARW").unwrap();
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