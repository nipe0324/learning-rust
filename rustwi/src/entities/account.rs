use sha2::{Digest, Sha256};

pub struct Account {
    #[allow(dead_code)]
    pub id: Option<i32>,
    pub email: String,
    pub hashed_password: String,
    pub display_name: String,
}

impl Account {
    pub fn new(id: i32, email: String, hashed_password: String, display_name: String) -> Account {
        Account {
            id: Some(id),
            email,
            hashed_password,
            display_name,
        }
    }

    pub fn create(email: &str, password: &str, display_name: &str) -> Account {
        Account {
            id: None,
            email: email.to_string(),
            hashed_password: to_sha256(password),
            display_name: display_name.to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn id(&self) -> Option<i32> {
        self.id
    }

    pub fn mathes_password(&self, password: &str) -> bool {
        self.hashed_password == to_sha256(password)
    }
}

fn to_sha256(str: &str) -> String {
    let str = str.as_bytes();
    let hashed_str = Sha256::digest(str);
    format!("{:x}", hashed_str)
}
