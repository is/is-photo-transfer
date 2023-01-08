use std::error::Error;

mod img;

fn main() -> Result<(), Box<dyn Error>> {
    for path in &["tests/IMG_0256.HEIC", "tests/IMG_0257.DNG"] {
        println!("-- {}", path);

        let file = std::fs::File::open(path)?;
        let mut buf_reader = std::io::BufReader::new(&file);
        let exifreader = exif::Reader::new();
        let exif = exifreader.read_from_container(&mut buf_reader)?;
        for f in exif.fields() {
            println!("{} {} {}", f.tag, f.ifd_num, f.display_value().with_unit(&exif));
        }
    }
    Ok(())
}
