use clap::{Command, ArgMatches};
use crate::{db::Database, error::Result};

pub fn command() -> Command {
    Command::new("ls").about("List all tasks")
}

pub fn execute(_args: &ArgMatches, db: Database) -> Result<()> {
    let tasks = db.get_all_tasks()?;
    
    if tasks.is_empty() {
        println!("No tasks found.");
        return Ok(());
    }
    
    println!("{:<6} {:<2} {:<30} {:<10}", "ID", "✓", "Description", "Scheduled");
    println!("{}", "-".repeat(50));
    
    for task in tasks {
        println!("{:<6} {:<2} {:<30} {:<10}", 
            task.id.unwrap(),
            task.status.display_symbol(),
            task.description,
            task.scheduled
        );
    }
    
    Ok(())
}
