mod cmd;
mod core;
mod task;

use std::error::Error;

const VERSION: &str = "0.1.2";

fn main() -> Result<(), Box<dyn Error>> {
    cmd::run()
}
