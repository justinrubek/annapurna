use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx_ulid::Ulid;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub task_id: Ulid,
    pub description: String,
    pub completed: bool,
    pub duration: Option<Duration>,
    pub start_time: Option<DateTime<Utc>>,
    pub completion_time: Option<DateTime<Utc>>,
}
