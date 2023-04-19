use chrono::{DateTime, Utc};

pub struct Tweet {
    #[allow(dead_code)]
    id: Option<i32>,
    pub message: String,
    pub posted_at: DateTime<Utc>,
}

impl Tweet {
    pub fn new(id: i32, message: String, posted_at: DateTime<Utc>) -> Tweet {
        Tweet {
            id: Some(id),
            message,
            posted_at,
        }
    }

    #[allow(dead_code)]
    pub fn id(&self) -> Option<i32> {
        self.id
    }
}
