use crate::entities::Account;

#[cfg_attr(test, mockall::automock)]
#[axum::async_trait]
pub trait Accounts {
    async fn find_by_email(&self, email: &str) -> Option<Account>;
    async fn create(&self, entity: &Account);
}
