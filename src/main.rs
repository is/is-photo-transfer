mod cmd;
mod core;
mod task;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    cmd::run()
}
