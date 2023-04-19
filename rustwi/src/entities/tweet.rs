use chrono::{DateTime, Utc};

pub struct Tweet {
    #[allow(dead_code)]
    id: Option<i32>,
    pub message: String,
    pub posted_at: DateTime<Utc>,
    deleted: bool,
}

impl Tweet {
    pub fn new(id: i32, message: String, posted_at: DateTime<Utc>) -> Tweet {
        Tweet {
            id: Some(id),
            message,
            posted_at,
            deleted: false,
        }
    }

    pub fn create(message: &str) -> Tweet {
        Tweet {
            id: None,
            message: message.to_string(),
            posted_at: Utc::now(),
            deleted: false,
        }
    }

    pub fn id(&self) -> Option<i32> {
        self.id
    }

    pub fn is_deleted(&self) -> bool {
        self.deleted
    }

    pub fn delete(&mut self) {
        self.deleted = true;
    }
}
