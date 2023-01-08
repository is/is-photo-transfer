use std::ffi::OsStr;
use std::path::Path;
use std::string::String;

pub struct ImageMeta {
    pub model: String,
    pub datetime: String,
    pub number: String,
    pub ext: String,
}

pub enum FileNamePattern {
    V0(ImageMeta),
    V1(ImageMeta),
    NumberOnly(ImageMeta),
    Unknown,
}


fn split_path(path: &str) -> Option<(&OsStr, &OsStr, &OsStr)> {
    let path = Path::new(path);
    Some((path.parent()?.as_os_str(), path.file_stem()?, path.extension()?))
}

pub fn image_meta_from_file_name_only(file_name:&str) -> FileNamePattern {
    FileNamePattern::Unknown
}


#[cfg(test)]
mod tests {
    use std::env;
    use std::ffi::OsStr;
    use std::path::{Path, PathBuf};
    use lazy_regex::regex;
    use regex::Regex;

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