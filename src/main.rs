mod task;
mod db;
mod parser;

use db::Database;
use parser::{parse_args, CliCommand};
use task::{Task, TaskStatus};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Database::new("tasks.db")?;
    
    match parse_args() {
        CliCommand::Add { description } => {
            let mut task = Task::new();
            task.description = description;
            db.save_task(&mut task)?;
            println!("Added task: {}", task.description);
        }
        CliCommand::Delete { ids } => {
            if !ids.is_empty() {
                for task_id in ids {
                    db.delete_task(task_id)?;
                    println!("Deleted task {}", task_id);
                }
            } else {
                let tasks = db.get_all_tasks()?;
                if tasks.is_empty() {
                    println!("No tasks to delete.");
                } else {
                    println!("Select tasks to delete:");
                    println!("{:<6} {:<2} {:<30} {:<10}", "ID", "✓", "Description", "Scheduled");
                    println!("{}", "-".repeat(50));
                    for task in &tasks {
                        println!("{:<6} {:<2} {:<30} {:<10}", 
                            task.id.unwrap(),
                            task.status.display_symbol(),
                            task.description,
                            task.scheduled
                        );
                    }
                    print!("Enter task IDs (space-separated): ");
                    use std::io::{self, Write};
                    io::stdout().flush()?;
                    
                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;
                    
                    let ids: Vec<u64> = input
                        .split_whitespace()
                        .filter_map(|s| s.parse().ok())
                        .collect();
                    
                    for task_id in ids {
                        db.delete_task(task_id)?;
                        println!("Deleted task {}", task_id);
                    }
                }
            }
        }
        CliCommand::Done { ids } => {
            if !ids.is_empty() {
                for task_id in ids {
                    db.complete_task(task_id)?;
                    println!("Marked task {} as done", task_id);
                }
            } else {
                let tasks = db.get_all_tasks()?;
                let active_tasks: Vec<_> = tasks.into_iter().filter(|t| matches!(t.status, TaskStatus::Active)).collect();
                if active_tasks.is_empty() {
                    println!("No active tasks to mark as done.");
                } else {
                    println!("Select tasks to mark as done:");
                    println!("{:<6} {:<2} {:<30} {:<10}", "ID", "✓", "Description", "Scheduled");
                    println!("{}", "-".repeat(50));
                    for task in &active_tasks {
                        println!("{:<6} {:<2} {:<30} {:<10}", 
                            task.id.unwrap(),
                            task.status.display_symbol(),
                            task.description, 
                            task.scheduled
                        );
                    }
                    print!("Enter task IDs (space-separated): ");
                    use std::io::{self, Write};
                    io::stdout().flush()?;
                    
                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;
                    
                    let ids: Vec<u64> = input
                        .split_whitespace()
                        .filter_map(|s| s.parse().ok())
                        .collect();
                    
                    for task_id in ids {
                        db.complete_task(task_id)?;
                        println!("Marked task {} as done", task_id);
                    }
                }
            }
        }
        CliCommand::List => {
            let tasks = db.get_all_tasks()?;
            if tasks.is_empty() {
                println!("No tasks found.");
            } else {
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
        }
        CliCommand::Help => {
            println!("Usage: grillo [COMMAND]");
            println!("Commands:");
            println!("  add <description>  Add a new task");
            println!("  ls                 List all tasks");
            println!("  del [id...]        Delete tasks");
            println!("  done [id...]       Mark tasks as done");
        }
    }
    
    Ok(())
}
