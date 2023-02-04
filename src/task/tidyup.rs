use clap::Parser;

use crate::cmd::{Cmd, CmdResult};

// ==== COMMAND ====
#[derive(Parser, Debug)]
pub struct TidyupCommand {
    #[arg(default_value = ".")]
    source: String,
    #[arg(default_value = None)]
    dest: Option<String>,
    #[arg(short, long, default_value_t = false)]
    exif: bool,
    #[arg(short, long, default_value_t = false)]
    dry: bool,
    #[arg(short, long, default_value_t = false)]
    compact: bool,
    #[arg(short, long, default_value_t = false)]
    touch: bool,
    #[arg(short, long, default_value_t = false)]
    usemove: bool,
}

impl Cmd for TidyupCommand {
    fn run(&self) -> CmdResult {
        todo!()
    }
}

// ==== TASK ====
