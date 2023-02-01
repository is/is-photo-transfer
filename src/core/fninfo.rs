#![allow(dead_code)]

use exif::{In, Tag};
use lazy_static::lazy_static;
use regex::Regex;
use sha2::Digest;
use std::path::Path;
use std::result::Result;
use std::string::String;
use thiserror::Error;

const MAX_NUMBER: u32 = 100000;

static MODEL_MAP: phf::Map<&'static str, &'static str> = phf::phf_map! {
    "iPhone 6s" => "IP6S",
    "iPhone 7" => "IP7",
    "iPhone 12 Pro Max" => "IP12PM",
    "ILCE-6400" => "A6400",
    "ILCE-7RM4A" => "A7R4A",
    "ILCE-1" => "A1",
};

lazy_static! {
    static ref FILE_NAME_PATTERN_V1: Regex = Regex::new(r"(\D)_(\d{5})__(\d{8}_\d{6})").unwrap();
    static ref FILE_NAME_PATTERN_V2: Regex =
        Regex::new(r"(\d{8}_\d{6})__(\d{2,5})_(.{1,})").unwrap();
    static ref NUMBER_IN_FILE_NAME: Regex = Regex::new(r".+?(\d{2,})").unwrap();
}

fn split_path_2(path: &str) -> Option<(&str, &str, &str)> {
    let path = Path::new(path);
    Some((
        path.parent()?.as_os_str().to_str()?,
        path.file_stem()?.to_str()?,
        path.extension()?.to_str()?,
    ))
}

fn number_from_file_name_hash(file_name: &str) -> String {
    let mut hasher = sha2::Sha256::new();
    hasher.update(file_name);

    let digest = hasher.finalize();
    let tail = &digest[digest.len() - 4..];
    format!(
        "{:05}",
        u32::from_be_bytes(TryInto::<[u8; 4]>::try_into(tail).unwrap()) % MAX_NUMBER
    )
}

fn number_from_file_name(file_name: &str) -> String {
    if file_name.len() <= 9 {
        if let Some(captures) = NUMBER_IN_FILE_NAME.captures(file_name) {
            let number = captures.get(1).unwrap().as_str();
            let number = format!("{:05}", number.parse::<u32>().unwrap() % MAX_NUMBER);
            return number;
        }
    }
    number_from_file_name_hash(file_name)
}

fn file_ext_normal(ext: &str) -> String {
    if ext == "jpeg" || ext == "JPEG" {
        "JPG".into()
    } else {
        ext.to_uppercase()
    }
}

pub struct Info {
    pub model: String,
    pub datetime: String,
    pub number: String,
    pub ext: String,
    pub ver: InfoVer,
}

#[derive(PartialEq)]
pub enum InfoVer {
    V1,
    V2,
    EXIF,
}

#[derive(Error, Debug)]
pub enum InfoErr {
    #[error("io error:{0}, path: {1}")]
    Io(&'static str, String),
    #[error("exif error:{0}, path: {1}")]
    Exif(&'static str, String),
    #[error("unknown:{0}, path: {1}")]
    Unknown(&'static str, String),
}

pub fn from(path: &str) -> Result<Info, InfoErr> {
    Info::from(path)
}

impl Info {
    pub fn from(path: &str) -> Result<Self, InfoErr> {
        if let Some(m) = Self::from_path(path) {
            Ok(m)
        } else {
            Self::from_exif(path)
        }
    }

    pub fn from_path(path: &str) -> Option<Self> {
        let (_dir, file_stem, file_ext) = split_path_2(path)?;

        if let Some(captures) = FILE_NAME_PATTERN_V1.captures(file_stem) {
            return Some(Self {
                model: captures.get(1)?.as_str().to_string(),
                datetime: captures.get(3)?.as_str().to_string(),
                number: captures.get(2)?.as_str().to_string(),
                ext: file_ext_normal(file_ext),
                ver: InfoVer::V1,
            });
        }

        if let Some(captures) = FILE_NAME_PATTERN_V2.captures(file_stem) {
            return Some(Self {
                model: captures.get(3)?.as_str().to_string(),
                datetime: captures.get(1)?.as_str().to_string(),
                number: captures.get(2)?.as_str().to_string(),
                ext: file_ext_normal(file_ext),
                ver: InfoVer::V2,
            });
        }
        None
    }

    pub fn from_exif(path: &str) -> Result<Self, InfoErr> {
        type E = InfoErr;
        let (dir, file_stem, file_ext) =
            split_path_2(path).ok_or_else(|| E::Io("split", path.into()))?;

        let number = number_from_file_name(file_stem);

        let file = std::fs::File::open(path).map_err(|e| E::Io("open", path.into()))?;

        let mut buf_reader = std::io::BufReader::new(&file);
        let exif_reader = exif::Reader::new();
        let exif = exif_reader
            .read_from_container(&mut buf_reader)
            .map_err(|e| E::Exif("parse", path.into()))?;

        let model_field = exif
            .get_field(Tag::Model, In::PRIMARY)
            .ok_or_else(|| E::Exif("model", path.into()))?;
        let datetime_field = exif
            .get_field(Tag::DateTimeOriginal, In::PRIMARY)
            .ok_or_else(|| E::Exif("datetime", path.into()))?;

        let model_value = model_field
            .display_value()
            .to_string()
            .clone()
            .replace("\"", "");
        let datetime = datetime_field
            .display_value()
            .to_string()
            .clone()
            .replace(" ", "_")
            .replace(":", "")
            .replace("-", "");

        let model = MODEL_MAP.get(&model_value).unwrap_or(&"UNSET").to_string();
        // println!("model:{}, datetime:{}", model_value, datetime_value);
        Ok(Self {
            model,
            datetime,
            number,
            ext: file_ext_normal(file_ext),
            ver: InfoVer::EXIF,
        })
    }

    pub fn from_exif_2(path: &str, number:&str) -> Result<Self, InfoErr> {
        Self::from_exif(path).map(|e| Self{number: number.to_string(), ..e})
    }

    pub fn to_name(&self) -> String {
        format!("{}__{}__{}", self.datetime, self.number, self.model)
    }

    pub fn to_file_name(&self) -> String {
        format!("{}.{}", self.to_name(), self.ext)
    }

    pub fn update_from_exif(self, path: &str) -> Self {
        if self.ver == InfoVer::V2 {
            self
        } else {
            Self::from_exif(path)
                .map(|m| Self {
                    number: self.number.clone(),
                    ..m
                })
                .unwrap_or(self)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::number_from_file_name;
    use regex::Regex;
    use sha2::Digest;
    use std::ffi::OsStr;
    use std::path::{Path, PathBuf};

    static FILE_NAME_1: &str = "A_02104__20230105_150108.arw";
    static FILE_NAME_2: &str = "20230105_150108__03212_A7R4.arw";

    #[test]
    fn test_number_in_name() {
        assert_eq!(number_from_file_name("DSC03212"), "03212");
        assert_eq!(
            number_from_file_name("BB40B3F3-2D88-4023-82BA-90CAE2C4DB45"),
            "36957"
        );
    }

    #[test]
    fn test_hash_sha2() {
        let mut hasher = sha2::Sha256::new();
        hasher.update(b"hello world");
        let digest = hasher.finalize();
        let tail = &digest[(digest.len() - 4)..];
        let sign = u32::from_be_bytes(TryInto::<[u8; 4]>::try_into(tail).unwrap());
        assert_eq!(sign, 3807366633);
        assert_eq!(format!("{:05}", sign % 10000), "06633");
        /*
        assert_eq!(digest[..], hex!("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9")[..]);
        */
    }

    #[test]
    fn test_path_buf() {
        let path_buf = PathBuf::from("/home/admin/.config");

        assert_eq!(
            "/home/admin",
            path_buf.parent().and_then(Path::to_str).unwrap()
        );
        assert_eq!(
            ".config",
            path_buf.file_name().and_then(OsStr::to_str).unwrap()
        );

        let path_buf = PathBuf::from("HELLO.ME.WORLD");
        assert!(path_buf.parent().and_then(Path::to_str).unwrap().is_empty());
        assert_eq!(
            "WORLD",
            path_buf.as_path().extension().unwrap().to_str().unwrap()
        );
        assert_eq!(
            "HELLO.ME",
            path_buf.as_path().file_stem().unwrap().to_str().unwrap()
        );
    }

    #[test]
    fn test_split_path() {
        let (p0, p1, p2) = super::split_path_2("A_02104__20230105_150108.ARW").unwrap();
        assert_eq!("", p0);
        assert_eq!("A_02104__20230105_150108", p1);
        assert_eq!("ARW", p2);

        let (p0, p1, p2) = super::split_path_2("ABC/A_02104__20230105_150108.ARW").unwrap();
        assert_eq!("ABC", p0);
        assert_eq!("A_02104__20230105_150108", p1);
        assert_eq!("ARW", p2);
    }

    #[test]
    fn test_regex_0() {
        let name = "A_02104__20230105_150108";
        let r0 = Regex::new(r"(\D)_(\d{5})__(\d{8}_\d{6})").unwrap();
        let captures = r0.captures(name).unwrap();
        assert_eq!(
            "A_02104__20230105_150108",
            captures.get(0).unwrap().as_str()
        );
        assert_eq!("A", captures.get(1).unwrap().as_str());
        assert_eq!("02104", captures.get(2).unwrap().as_str());
        assert_eq!("20230105_150108", captures.get(3).unwrap().as_str());
    }
}
