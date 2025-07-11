use clap::{Command, Arg, ArgMatches};
use crate::{db::Database, task::TaskStatus, error::Result};
use std::io::{self, Write};

pub fn command() -> Command {
   Command::new("done")
       .about("Mark tasks as done")
       .arg(
           Arg::new("ids")
               .help("Task IDs to mark as done")
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
       complete_tasks(&db, ids)
   } else {
       interactive_complete(&db)
   }
}

fn complete_tasks(db: &Database, ids: Vec<u64>) -> Result<()> {
   for id in ids {
       db.complete_task(id)?;
       println!("Marked task {} as done", id);
   }
   Ok(())
}

fn interactive_complete(db: &Database) -> Result<()> {
   let tasks = db.get_all_tasks()?;
   let active_tasks: Vec<_> = tasks
       .into_iter()
       .filter(|t| matches!(t.status, TaskStatus::Active))
       .collect();
   
   if active_tasks.is_empty() {
       println!("No active tasks to mark as done.");
       return Ok(());
   }
   
   display_tasks(&active_tasks);
   
   print!("Enter task IDs (space-separated): ");
   io::stdout().flush()?;
   
   let mut input = String::new();
   io::stdin().read_line(&mut input)?;
   
   let ids: Vec<u64> = input
       .split_whitespace()
       .filter_map(|s| s.parse().ok())
       .collect();
   
   complete_tasks(db, ids)
}

fn display_tasks(tasks: &[crate::task::Task]) {
   println!("Select tasks to mark as done:");
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
