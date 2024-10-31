use std::error::Error;

use clap::{Parser, Subcommand};

use crate::task::import::ImportCommand;
use crate::task::rename::RenameCommand;
use crate::task::tidyup::TidyupCommand;

use crate::task::rename2::Rename2Command;
use crate::task::rename3::Rename3Command;

pub type CmdResult = Result<(), Box<dyn Error>>;

#[derive(Parser)]
#[command(name = "iphoto")]
#[command(author = "Yu Xin <scaner@gmail.com>")]
#[command(version = crate::VERSION, about = "I.S. Photo Armory")]
#[command(about="Photograph toolbox", long_about=None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Import photos from Camera")]
    Import(ImportCommand),
    #[command(about = "Rename photos in the directories")]
    Rename(RenameCommand),
    #[command(about = "Rename photos in the directories, v2")]
    Rename2(Rename2Command),
    #[command(about = "Rename photos in the directories, v3")]
    Rename3(Rename3Command),
    #[command(about = "Tidyup photos in the directories")]
    Tidyup(TidyupCommand),
}

pub trait Cmd {
    fn run(self) -> CmdResult;
}

pub fn run() -> CmdResult {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Import(cmd)) => cmd.run(),
        Some(Commands::Rename(cmd)) => cmd.run(),
        Some(Commands::Rename2(cmd)) => cmd.run(),
        Some(Commands::Rename3(cmd)) => cmd.run(),
        Some(Commands::Tidyup(cmd)) => cmd.run(),
        _ => Ok(()),
    }
}
