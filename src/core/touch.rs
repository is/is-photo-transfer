use std::path::Path;

pub fn touch_form_0(path: &str, date_str: &str) -> Result<(), std::io::Error> {
    let date = chrono::NaiveDate::parse_from_str(date_str, "%Y%m%d").unwrap();
    let seconds = date
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_local_timezone(chrono::offset::Local)
        .unwrap()
        .timestamp();
    let ftime = filetime::FileTime::from_unix_time(seconds, 0);
    filetime::set_file_times(path, ftime, ftime)
}

pub fn touch_form_1(path: &str, ftime: std::time::SystemTime) -> Result<(), std::io::Error> {
    let ftime = filetime::FileTime::from_system_time(ftime);
    filetime::set_file_times(path, ftime, ftime)
}

pub fn touch_with_filename(path: &str) -> Result<(), std::io::Error> {
    let p = Path::new(path);
    let file_stem = p.file_stem().unwrap();

    if file_stem.len() < 15 {
        return Ok(());
    }

    let date_str = &file_stem.to_str().unwrap().to_string()[0..15];
    let date = chrono::NaiveDateTime::parse_from_str(date_str, "%Y%m%d_%H%M%S")
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let seconds = date
        .and_local_timezone(chrono::offset::Local)
        .unwrap()
        .timestamp();
    let ftime = filetime::FileTime::from_unix_time(seconds, 0);
    filetime::set_file_times(path, ftime, ftime)
}
