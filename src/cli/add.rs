use clap::{Command, Arg, ArgMatches};
use crate::{db::Database, task::Task, error::Result};

pub fn command() -> Command {
    Command::new("add")
        .about("Add a new task")
        .arg(
            Arg::new("description")
                .help("Task description")
                .required(true)
                .index(1)
        )
}

pub fn execute(args: &ArgMatches, db: Database) -> Result<()> {
    let description = args
        .get_one::<String>("description")
        .unwrap()
        .to_string();
    
    let mut task = Task::new();
    task.description = description;
    db.save_task(&mut task)?;
    
    println!("Added task: {}", task.description);
    Ok(())
}
