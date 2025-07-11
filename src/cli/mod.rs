mod add;
mod list;
mod delete;
mod done;

use clap::Command;
use crate::{db::Database, error::Result};

/// Entry point for the CLI application
pub fn run() -> Result<()> {
    let matches = build_cli().get_matches();
    let db = Database::new("tasks.db")?;
    
    // Route to appropriate command handler
    match matches.subcommand() {
        Some(("add", args)) => add::execute(args, db),
        Some(("ls", args)) => list::execute(args, db),
        Some(("del", args)) => delete::execute(args, db),
        Some(("done", args)) => done::execute(args, db),
        _ => print_help(),
    }
}

/// Builds the CLI structure with all subcommands
fn build_cli() -> Command {
    Command::new("grillo")
        .version("0.1.0")
        .about("CLI to-do list following GTD philosophy")
        .subcommand_required(false)
        .arg_required_else_help(false)
        .subcommand(add::command())
        .subcommand(list::command())
        .subcommand(delete::command())
        .subcommand(done::command())
}

/// Prints help when no valid command is provided
fn print_help() -> Result<()> {
    build_cli().print_help()?;
    Ok(())
}
