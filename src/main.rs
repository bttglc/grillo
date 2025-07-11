mod task;
mod db;
mod cli;
mod error;

use error::Result;

fn main() -> Result<()> {
    cli::run()
}
