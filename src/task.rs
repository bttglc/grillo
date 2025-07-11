use chrono::{DateTime, Local, NaiveDate};

#[derive(Debug, Clone)]
pub struct Task {
    pub id: u64,                            // Only saved tasks have IDs
    pub description: String,                // The text of the task
    pub created: DateTime<Utc>,             // For uniqueness purposes
    pub scheduled: NaiveDate,               // When the task should be done
    pub deadline: Option<NaiveDate>,        // Not all tasks have deadlines
    pub status: TaskStatus,                 // v. below
    pub context: Option<u64>,               // @home, @computer for end user, mapped to ID
}

#[derive(Debug, Clone, Default)]
pub struct Draft {
    pub description: String,                // The text of the task
    pub created: DateTime<Utc>,             // For uniqueness purposes
    pub scheduled: NaiveDate,               // When the task should be done
    pub deadline: Option<NaiveDate>,        // Not all tasks have deadlines
    pub status: TaskStatus,                 // v. below
    pub context: Option<u64>,               // @home, @computer for end user, mapped to ID
}

impl Default for Draft {
    fn default() -> Draft {
        Draft {
            description: String::new(),
            created: chrono::Utc::now(),
            scheduled: chrono::Utc::now().date_naive(),
            deadline: None,
            status: TaskStatus::default(),
            context: None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub enum TaskStatus {
    Active,
    Done,
}

impl Default for TaskStatus {
    fn default() -> Self {
        TaskStatus::Active
    }
}
