use std::error::Error;

use clap::{Parser, Subcommand};

use crate::task::import::ImportCommand;
use crate::task::rename::RenameCommand;
use crate::task::tidyup::TidyupCommand;

pub type CmdResult = Result<(), Box<dyn Error>>;

#[derive(Parser)]
#[command(name = "is-armory-photo")]
#[command(author = "Yu Xin <scaner@gmail.com>")]
#[command(version = "0.1.1", about = "I.S. Photo Armory")]
#[command(about="Photograph toolbox", long_about=None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Import photographs from Camera")]
    Import(ImportCommand),
    #[command(about = "Rename photo in the directory")]
    Rename(RenameCommand),
    #[command(about = "Tidyup photo in the directory")]
    Tidyup(TidyupCommand),
}

pub trait Cmd {
    fn run(&self) -> CmdResult;
}

pub fn run() -> CmdResult {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Import(cmd)) => cmd.run(),
        Some(Commands::Rename(cmd)) => cmd.run(),
        Some(Commands::Tidyup(cmd)) => cmd.run(),
        _ => Ok(()),
    }
}
