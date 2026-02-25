use crate::domain::models::Account;
use crate::domain::types::AccountId;
use crate::error::MmexError;

pub trait AccountRepository {
    fn find_all(&self) -> Result<Vec<Account>, MmexError>;
    fn find_by_id(&self, id: AccountId) -> Result<Option<Account>, MmexError>;
}
