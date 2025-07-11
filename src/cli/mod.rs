mod add;
mod list;
mod delete;
mod done;

use clap::{Command};
use crate::{db::Database, error::Result};

pub fn run() -> Result<()> {
    let matches = build_cli().get_matches();
    let db = Database::new("tasks.db")?;
    
    match matches.subcommand() {
        Some(("add", args)) => add::execute(args, db),
        Some(("ls", args)) => list::execute(args, db),
        Some(("del", args)) => delete::execute(args, db),
        Some(("done", args)) => done::execute(args, db),
        _ => print_help(),
    }
}

fn build_cli() -> Command {
    Command::new("grillo")
        .version("0.1.0")
        .about("GTD task management")
        .subcommand_required(false)
        .arg_required_else_help(false)
        .subcommand(add::command())
        .subcommand(list::command())
        .subcommand(delete::command())
        .subcommand(done::command())
}

fn print_help() -> Result<()> {
    build_cli().print_help()?;
    Ok(())
}
