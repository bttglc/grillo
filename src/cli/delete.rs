use clap::{Command, Arg, ArgMatches};
use crate::{db::Database, error::Result};
use std::io::{self, Write};

pub fn command() -> Command {
    Command::new("del")
        .about("Delete tasks")
        .arg(
            Arg::new("ids")
                .help("Task IDs to delete")
                .num_args(1..)
                .value_parser(clap::value_parser!(u64))
        )
}

pub fn execute(args: &ArgMatches, db: Database) -> Result<()> {
    let ids: Vec<u64> = args
        .get_many::<u64>("ids")
        .map(|vals| vals.copied().collect())
        .unwrap_or_default();
    
    if !ids.is_empty() {
        delete_tasks(&db, ids)
    } else {
        interactive_delete(&db)
    }
}

fn delete_tasks(db: &Database, ids: Vec<u64>) -> Result<()> {
    for id in ids {
        db.delete_task(id)?;
        println!("Deleted task {}", id);
    }
    Ok(())
}

fn interactive_delete(db: &Database) -> Result<()> {
    let tasks = db.get_all_tasks()?;
    if tasks.is_empty() {
        println!("No tasks to delete.");
        return Ok(());
    }
    
    display_tasks(&tasks);
    
    print!("Enter task IDs (space-separated): ");
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    let ids: Vec<u64> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    
    delete_tasks(db, ids)
}

fn display_tasks(tasks: &[crate::task::Task]) {
    println!("Select tasks to delete:");
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
}
