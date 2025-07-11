use chrono::{DateTime, Utc, NaiveDate};

#[derive(Debug, Clone)]
pub struct Task {
    pub id: Option<u64>,
    pub description: String,
    pub created: DateTime<Utc>,
    pub scheduled: NaiveDate,
    pub deadline: Option<NaiveDate>,
    pub status: TaskStatus,
    pub project: Option<u64>,
    pub context: Option<u64>,
}

#[derive(Debug, Clone, Default)]
pub enum TaskStatus {
    #[default] Active,
    Done,
}

impl Task {
    pub fn new() -> Self {
        Self {
            id: None,
            description: String::new(),
            created: Utc::now(),
            scheduled: Utc::now().date_naive(),
            deadline: None,
            status: TaskStatus::Active,
            project: None,
            context: None,
        }
    }
}
