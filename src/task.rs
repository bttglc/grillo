use chrono::{DateTime, Local, NaiveDate};

#[derive(Debug, Clone)]
pub struct Task {
    pub id: Option<i32>,                    // None for new tasks, Some(id) for saved ones
    pub description: String,                // The text of the task
    pub created: DateTime<Local>,           // For uniqueness purposes
    pub date: NaiveDate,                    // When the task should be done
    pub deadline: Option<DateTime<Local>>,  // Not all tasks have deadlines
    pub status: TaskStatus,                 // v. below
    pub context: Option<String>,            // @home, @computer, etc.
}

impl Default for Task {
    fn default() -> Task {
        Task {
            id: None,
            description: String::new(),
            created: chrono::Local::now(),
            date: Local::now().date_naive(),
            deadline: None,
            status: TaskStatus::default(),
            context: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum TaskStatus {
    Active,
    Done,
}

impl Default for TaskStatus {
    fn default() -> Self {
        TaskStatus::Active
    }
}
