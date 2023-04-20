use async_session::{Session, SessionStore};
use async_sqlx_session::PostgresSessionStore;
use std::time::Duration;

use crate::constants::{database_url, AXUM_SESSION_COOKIE_NAME, AXUM_SESSION_USER_ID_KEY};
use crate::entities::Account;
use crate::repos::Accounts;

pub struct SessionToken(String);

impl SessionToken {
    pub fn cookie(&self) -> String {
        format!(
            "{}={}; aMax-Age=604800; Path=/; HttpOnly;",
            AXUM_SESSION_COOKIE_NAME, &self.0
        )
    }
}

pub async fn create_account(repo: &impl Accounts, email: &str, password: &str, display_name: &str) {
    let account = Account::create(email, password, display_name);
    repo.create(&account).await;
}

pub async fn authenticate(
    repo: &impl Accounts,
    email: &str,
    password: &str,
) -> Option<SessionToken> {
    let account = repo.find_by_email(email).await;
    if let Some(account) = account {
        if !account.matches_password(password) {
            return None;
        }

        let database_url = database_url();
        let store = PostgresSessionStore::new(&database_url).await.unwrap();

        let mut session = Session::new();
        session
            .insert(AXUM_SESSION_USER_ID_KEY, account.id().unwrap())
            .unwrap();
        session.expire_in(Duration::from_secs(604800));

        let cookie = store.store_session(session).await.unwrap().unwrap();

        Some(SessionToken(cookie))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use sha2::{Digest, Sha256};

    use crate::entities::Account;
    use crate::repos::MockAccounts;

    fn account(id: i32) -> Account {
        Account::new(
            id,
            format!("{}@exmaple.com", id),
            to_sha256(format!("password{}", id)),
            format!("display_name{}", id),
        )
    }

    fn to_sha256(str: String) -> String {
        let str = str.as_bytes();
        let hashed_str = Sha256::digest(str);
        format!("{:x}", hashed_str)
    }

    #[tokio::test]
    async fn test_create_account() {
        let mut accounts = MockAccounts::new();
        accounts
            .expect_create()
            .withf(move |e| {
                let account = account(1);
                e.email == account.email
                    && e.hashed_password == account.hashed_password
                    && e.display_name == account.display_name
            })
            .once()
            .return_const(());

        let account = account(1);
        super::create_account(
            &accounts,
            &account.email,
            "password1",
            &account.display_name,
        )
        .await;
    }

    #[tokio::test]
    async fn test_authenticate() {
        let mut accounts = MockAccounts::new();
        accounts
            .expect_find_by_email()
            .returning(|_| Some(account(1)));

        let account = account(1);
        let result = super::authenticate(&accounts, &account.email, "password1").await;
        assert_eq!(result.is_some(), true);
    }

    #[tokio::test]
    async fn test_authenticate_not_found() {
        let mut accounts = MockAccounts::new();
        accounts.expect_find_by_email().returning(|_| None);

        let account = account(1);
        let result = super::authenticate(&accounts, &account.email, "password1").await;
        assert_eq!(result.is_some(), false);
    }
}
