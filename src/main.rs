use std::error::Error;
use std::path::Path;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod cmd;
mod fninfo;

#[allow(dead_code)]
fn p(p: &str) -> Result<(), Box<dyn Error>> {
    let m = fninfo::from(p)?;
    println!("{} -> {}", p, m.to_file_name());
    Ok(())
}

#[derive(Parser)]
#[command(name = "is-armory-photo")]
#[command(author = "Yu Xin <scaner@gmail.com>")]
#[command(version = "0.1.0", about = "I.S. Photo Armory")]
#[command(about="Photograph toolbox", long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Import photographs from Camera")]
    Import(ImportCommand),
}

#[derive(Parser)]
struct ImportCommand {
    source: Option<String>,
    dest: Option<String>,
    #[arg(long, default_value_t=String::from("mac"))]
    host: String,
}

type R0 = Result<(), Box<dyn Error>>;

fn cmd_import_source_dir(cmd: &ImportCommand) -> String {
    match cmd.source.as_ref() {
        Some(s) => s.clone(),
        _ => match cmd.host.as_str() {
            "mac" => "/Volumns/Untitled",
            _ => "/Volumns/Untitled",
        }
        .to_string(),
    }
}

fn cmd_import_dest_dir(cmd: &ImportCommand) -> String {
    if let Some(s) = cmd.dest.as_ref() {
        s.clone()
    } else {
        let home = env_var("HOME").unwrap();
        match cmd.host.as_str() {
            // "mac" => format!("{home}/PI"),
            // "hi" => format!("{home}/PI"),
            // "mi2" => format!("{home}/PI"),
            _ => format!("{home}/PI"),
        }
    }
}

fn env_var(key: &str) -> Option<String> {
    std::env::var_os(key).map(|e| e.into_string().unwrap())
}

fn cmd_import(cmd: &ImportCommand) -> R0 {
    let source = PathBuf::from(cmd_import_source_dir(cmd));
    let dest = PathBuf::from(cmd_import_dest_dir(cmd));

    println!("name:{:?}, source:{:?}, dest:{:?}", cmd.host, source, dest);
    let mut req = cmd::import::Request { source, dest };
    cmd::import::import(&mut req)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Import(cmd)) => cmd_import(&cmd),
        _ => Ok(()),
    }
}

fn _main() -> Result<(), Box<dyn Error>> {
    // let m = meta::Meta::from_exif("tests/BCDF1203-FD49-4805-B2AE-8E93B67D9076.JPG")?;
    // println!("{}", m.to_name());
    // let m = meta::Meta::from("tests/IMG_0256.HEIC")?;
    // println!("{}, {}", m.to_file_name());
    // p("tests/IMG_0256.HEIC")?;
    // p("tests/IMG_0257.DNG")?;

    let home_value = std::env::var_os("HOME").unwrap();
    let home = home_value.to_str().unwrap();
    // let source = Path::new(&home).join("P0/DCIM");
    let source = Path::new("/Volumes/Untitled").to_path_buf();
    let dest = Path::new(&home).join("M0/P8");

    let mut req = cmd::import::Request { source, dest };
    cmd::import::import(&mut req)?;
    Ok(())
}
