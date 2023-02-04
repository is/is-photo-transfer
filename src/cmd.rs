use std::error::Error;

use clap::{Parser, Subcommand};

use crate::task::import::ImportCommand;
use crate::task::rename::RenameCommand;

pub type CommandResult = Result<(), Box<dyn Error>>;

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
    #[command(about = "Rename photo in directory")]
    Rename(RenameCommand),
}

pub trait Command {
    fn run(&self) -> CommandResult;
}

pub fn run() -> CommandResult {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Import(cmd)) => cmd.run(),
        Some(Commands::Rename(cmd)) => cmd.run(),
        _ => Ok(()),
    }
}
