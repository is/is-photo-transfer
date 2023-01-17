use std::ffi::OsStr;
use std::path::Path;
use std::string::String;
use lazy_regex::{Lazy, regex};
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


impl Default for ImageMeta {
    fn default() -> Self {
        ImageMeta {
            model: "".to_string(),
            datetime: String::from(""),
            number: String::from(""),
            ext: String::from(""),
            version: PatternVersion::UNKNOWN,
        }
    }
}


static FILENAME_PATTERN_V1: &Lazy<Regex> = regex!(r"(\D)_(\d{5})__(\d{8}_\d{6})");
static FILENAME_PATTERN_V2: &Lazy<Regex> = regex!(r"(\d{8}_\d{6})__(\d{2,5})_(\D{1,2})");

fn get_image_meta_from_filename(file_stem: &str, file_ext: &str) -> Option<ImageMeta>{
    if let Some(captures) =  FILENAME_PATTERN_V1.captures(file_stem) {
        Some(ImageMeta {
            model: captures.get(3)?.as_str().to_string(),
            datetime: captures.get(1)?.as_str().to_string(),
            number: captures.get(2)?.as_str().to_string(),
            ext: file_ext.to_uppercase(),
            version: PatternVersion::V1,
        })
    } else if let Some(captures) = FILENAME_PATTERN_V2.captures(file_stem) {
        Some(ImageMeta {
            model: captures.get(2)?.as_str().to_string(),
            datetime: captures.get(1)?.as_str().to_string(),
            number: captures.get(3)?.as_str().to_string(),
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

        get_image_meta_from_filename(file_stem, file_ext)
            .or(get_image_meta_from_exif(full_path, file_stem, file_ext))
    }
}


#[cfg(test)]
mod tests {
    use std::ffi::OsStr;
    use std::path::{Path, PathBuf};
    use lazy_regex::regex;
    use regex::Regex;
    use crate::img::ImageMeta;

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


    #[test]
    fn test_lazy_regex_0() {
        let name = "A_02104__20230105_150108";
        let r0 = regex!(r"(\D)_(\d{5})__(\d{8}_\d{6})");
        let captures = r0.captures(name).unwrap();
        assert_eq!("A_02104__20230105_150108", captures.get(0).unwrap().as_str());
        assert_eq!("A", captures.get(1).unwrap().as_str());
        assert_eq!("02104", captures.get(2).unwrap().as_str());
        assert_eq!("20230105_150108", captures.get(3).unwrap().as_str());
    }

    #[test]
    fn test_image_meta_from_file() {
        let path = "A_02104__20230105_150108.arw";
        let meta = ImageMeta::from_file(path);
        assert!(!meta.is_none());
        assert_eq!(meta.unwrap().version, crate::img::PatternVersion::V1)
    }
}