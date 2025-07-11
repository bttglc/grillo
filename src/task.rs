use chrono::{DateTime, Utc, NaiveDate};

/// Represents a GTD task with scheduling and context information
#[derive(Debug, Clone)]
pub struct Task {
    /// Unique identifier (None for unsaved tasks)
    pub id: Option<u64>,
    /// Task description/title
    pub description: String,
    /// When the task was created
    pub created: DateTime<Utc>,
    /// When to work on this task
    pub scheduled: NaiveDate,
    /// Optional hard deadline
    pub deadline: Option<NaiveDate>,
    /// Current status
    pub status: TaskStatus,
    /// GTD project this task belongs to
    pub project: Option<u64>,
    /// GTD context (e.g., @home, @work, @computer)
    pub context: Option<u64>,
}

/// Task completion status
#[derive(Debug, Clone, Default)]
pub enum TaskStatus {
    /// Task is active and needs to be done
    #[default] 
    Active,
    /// Task has been completed
    Done,
}

impl Task {
    /// Creates a new task with default values
    /// - Scheduled for today
    /// - No deadline, project, or context
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

impl TaskStatus {
    /// Returns a visual symbol for the status
    pub fn display_symbol(&self) -> &'static str {
        match self {
            TaskStatus::Active => "○",
            TaskStatus::Done => "✓",
        }
    }
}