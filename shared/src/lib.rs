use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct Message {
    pub message: String,
    pub time: DateTime<Utc>,
    pub user: String,
}

impl Message {
    pub fn time_passed(&self) -> String {
        let now = Utc::now();
        let duration = now.signed_duration_since(self.time);

        if duration.num_seconds() < 60 {
            format!("{} seconds ago", duration.num_seconds())
        } else if duration.num_minutes() < 60 {
            format!("{} minutes ago", duration.num_minutes())
        } else {
            format!("{} hours ago", duration.num_hours())
        }
    }
}