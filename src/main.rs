mod task;
mod core;

use std::error::Error;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::core::utils;

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
    #[command(about = "Rename photo in directory")]
    Rename(RenameCommand),
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
        let home = utils::env_var("HOME").unwrap();
        match cmd.host.as_str() {
            // "mac" => format!("{home}/PI"),
            // "hi" => format!("{home}/PI"),
            // "mi2" => format!("{home}/PI"),
            _ => format!("{home}/PI"),
        }
    }
}

fn cmd_import(cmd: &ImportCommand) -> R0 {
    let source = PathBuf::from(cmd_import_source_dir(cmd));
    let dest = PathBuf::from(cmd_import_dest_dir(cmd));

    println!("name:{:?}, source:{:?}, dest:{:?}", cmd.host, source, dest);
    let mut req = task::import::Request { source, dest };
    task::import::import(&mut req)?;
    Ok(())
}


#[derive(Parser)]
struct RenameCommand {
    dir: String,
    #[arg(short, long, default_value_t=false)]
    dry: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Import(cmd)) => cmd_import(&cmd),
        _ => Ok(()),
    }
}
