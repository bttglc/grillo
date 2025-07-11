use rusqlite::{Connection, Result, Row};
use crate::task::{Task, TaskStatus};
use chrono::{DateTime, Utc, NaiveDate, Duration};

/// Database abstraction layer for task management
pub struct Database {
    conn: Connection,
}

impl Database {
    /// Creates a new database connection
    /// - Creates tables if they don't exist
    /// - Inserts sample data on first run
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        let db = Database { conn };
        let is_new = db.create_tables()?;
        if is_new {
            db.insert_sample_data()?;
        }
        Ok(db)
    }

    /// Creates the tasks table if it doesn't exist
    /// Returns true if table was created, false if it already existed
    fn create_tables(&self) -> Result<bool> {
        let table_exists: bool = self.conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='tasks'")?
            .exists([])?;
        
        if !table_exists {
            self.conn.execute(
                "CREATE TABLE tasks (
                    id INTEGER PRIMARY KEY,
                    description TEXT NOT NULL,
                    created DATETIME NOT NULL,
                    scheduled DATE NOT NULL,
                    deadline DATE,
                    status TEXT NOT NULL,
                    context INTEGER,
                    project INTEGER
                )",
                [],
            )?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Inserts GTD-style sample tasks for demonstration
    pub fn insert_sample_data(&self) -> Result<()> {
        let now = Utc::now();
        let today = now.date_naive();
        let yesterday = today - Duration::days(1);
        let tomorrow = today + Duration::days(1);
        let next_week = today + Duration::days(7);

        // Sample tasks with contexts (1=work, 2=personal, 3=computer)
        let samples: [(&str, NaiveDate, Option<NaiveDate>, &str, Option<u64>, Option<u64>); 5] = [
            ("Review project proposal", today, None, "Active", Some(1), None),
            ("Buy groceries", today, None, "Active", Some(2), None),
            ("Fix bug in parser", yesterday, None, "Done", Some(3), None),
            ("Team meeting", tomorrow, Some(next_week), "Active", Some(1), None),
            ("Read research paper", tomorrow, None, "Active", None, None),
        ];

        for (desc, scheduled, deadline, status, context, project) in samples {
            self.conn.execute(
                "INSERT INTO tasks (description, created, scheduled, deadline, status, context, project) 
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                rusqlite::params![
                    desc,
                    now.format("%Y-%m-%d %H:%M:%S").to_string(),
                    scheduled.format("%Y-%m-%d").to_string(),
                    deadline.map(|d| d.format("%Y-%m-%d").to_string()),
                    status,
                    context,
                    project
                ],
            )?;
        }
        Ok(())
    }

    /// Saves a task (insert if new, update if existing)
    /// Updates the task's ID on insert
    pub fn save_task(&self, task: &mut Task) -> Result<()> {
        if task.id.is_none() {
            // Insert new task
            let mut stmt = self.conn.prepare(
                "INSERT INTO tasks (description, created, scheduled, deadline, status, context, project) 
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)"
            )?;
            
            let id = stmt.insert(rusqlite::params![
                task.description,
                task.created.format("%Y-%m-%d %H:%M:%S").to_string(),
                task.scheduled.format("%Y-%m-%d").to_string(),
                task.deadline.map(|d| d.format("%Y-%m-%d").to_string()),
                task.status.to_string(),
                task.context,
                task.project
            ])?;
            
            task.id = Some(id as u64);
        } else {
            // Update existing task
            self.conn.execute(
                "UPDATE tasks SET description=?1, scheduled=?2, deadline=?3, status=?4, context=?5 
                 WHERE id=?6",
                rusqlite::params![
                    task.description,
                    task.scheduled.format("%Y-%m-%d").to_string(),
                    task.deadline.map(|d| d.format("%Y-%m-%d").to_string()),
                    task.status.to_string(),
                    task.context,
                    task.id.unwrap()
                ],
            )?;
        }
        Ok(())
    }

    /// Retrieves all tasks ordered by scheduled date and ID
    pub fn get_all_tasks(&self) -> Result<Vec<Task>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, description, created, scheduled, deadline, status, context, project 
             FROM tasks ORDER BY scheduled ASC, id ASC"
        )?;
        
        let task_iter = stmt.query_map([], |row| {
            Ok(self.row_to_task(row)?)
        })?;

        let mut tasks = Vec::new();
        for task in task_iter {
            tasks.push(task?);
        }
        Ok(tasks)
    }

    /// Marks a task as done by ID
    pub fn complete_task(&self, id: u64) -> Result<()> {
        self.conn.execute("UPDATE tasks SET status='Done' WHERE id=?1", [id])?;
        Ok(())
    }

    /// Deletes a task by ID
    pub fn delete_task(&self, id: u64) -> Result<()> {
        self.conn.execute("DELETE FROM tasks WHERE id=?1", [id])?;
        Ok(())
    }

    /// Converts a database row to a Task struct
    /// Handles date parsing and empty values
    fn row_to_task(&self, row: &Row) -> Result<Task> {
        // Handle optional deadline
        let deadline_str: Option<String> = row.get(4)?;
        let deadline = deadline_str
            .and_then(|s| if s.is_empty() { None } else { Some(s) })
            .and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok());

        // Parse status enum
        let status_str: String = row.get(5)?;
        let status = match status_str.as_str() {
            "Done" => TaskStatus::Done,
            _ => TaskStatus::Active,
        };

        // Parse created datetime with timezone
        let created_str: String = row.get(2)?;
        let created = DateTime::parse_from_str(&format!("{}+00:00", created_str), "%Y-%m-%d %H:%M:%S%z")
            .map_err(|_| rusqlite::Error::InvalidColumnType(2, "created".to_string(), rusqlite::types::Type::Text))?
            .with_timezone(&Utc);

        // Parse scheduled date
        let scheduled_str: String = row.get(3)?;
        let scheduled = NaiveDate::parse_from_str(&scheduled_str, "%Y-%m-%d")
            .map_err(|_| rusqlite::Error::InvalidColumnType(3, "scheduled".to_string(), rusqlite::types::Type::Text))?;

        Ok(Task {
            id: row.get(0)?,
            description: row.get(1)?,
            created,
            scheduled,
            deadline,
            status,
            context: row.get(6)?,
            project: row.get(7)?,
        })
    }
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::Active => write!(f, "Active"),
            TaskStatus::Done => write!(f, "Done"),
        }
    }
}