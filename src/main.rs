mod core;
mod task;

use std::error::Error;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use lazy_static::__Deref;

use crate::core::utils;

#[derive(Parser)]
#[command(name = "is-armory-photo")]
#[command(author = "Yu Xin <scaner@gmail.com>")]
#[command(version = "0.1.1", about = "I.S. Photo Armory")]
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

// ---- IMPORT ----
#[derive(Parser)]
struct ImportCommand {
    source: Option<String>,
    dest: Option<String>,
    #[arg(long, default_value_t=String::from("mac"))]
    host: String,
}

type CmdResult = Result<(), Box<dyn Error>>;

fn cmd_import_source_dir(cmd: &ImportCommand) -> String {
    match cmd.source.as_ref() {
        Some(s) => s.clone(),
        #[rustfmt::skip]
        _ => match cmd.host.as_str() {
            "mac" => "/Volumns/Untitled",
            _ => "/Volumns/Untitled",
        }.to_string(),
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

fn cmd_import(cmd: &ImportCommand) -> CmdResult {
    let source = PathBuf::from(cmd_import_source_dir(cmd));
    let dest = PathBuf::from(cmd_import_dest_dir(cmd));

    println!("name:{:?}, source:{:?}, dest:{:?}", cmd.host, source, dest);
    let mut req = task::import::Request { source, dest };
    task::import::import(&mut req)?;
    Ok(())
}

// ---- RENAME ----
#[derive(Parser)]
pub struct RenameCommand {
    #[arg(default_value_t = String::from("."))]
    dir: String,
    #[arg(short, long, default_value_t = false)]
    dry: bool,
}

fn cmd_rename(cmd: &RenameCommand) -> CmdResult {
    task::rename::rename(&task::rename::Request::from(cmd))?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Import(cmd)) => cmd_import(&cmd),
        Some(Commands::Rename(cmd)) => cmd_rename(&cmd),
        _ => Ok(()),
    }
}
