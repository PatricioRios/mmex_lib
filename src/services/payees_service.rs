use rusqlite::Connection;
use crate::domain::payees::{Payee, PayeeId, PayeeRepository};
use crate::infrastructure::payees_repository::SqlPayeeRepository;
use crate::error::MmexError;

pub struct PayeeService<'a> {
    conn: &'a Connection,
}

impl<'a> PayeeService<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn get_all_payees(&self) -> Result<Vec<Payee>, MmexError> {
        let repo = SqlPayeeRepository::new(self.conn);
        repo.find_all()
    }

    pub fn get_payee_by_id(&self, id: PayeeId) -> Result<Option<Payee>, MmexError> {
        let repo = SqlPayeeRepository::new(self.conn);
        repo.find_by_id(id)
    }

    pub fn create_payee(&self, name: &str) -> Result<Payee, MmexError> {
        if name.trim().is_empty() {
            return Err(MmexError::Validation("Payee name cannot be empty".into()));
        }
        let repo = SqlPayeeRepository::new(self.conn);
        let new_payee = Payee {
            id: PayeeId(0),
            name: name.to_string(),
            category_id: None,
            number: None,
            website: None,
            notes: None,
            active: true,
            pattern: None,
        };
        repo.insert(&new_payee)
    }

    pub fn update_payee(&self, payee: &Payee) -> Result<(), MmexError> {
        if payee.name.trim().is_empty() {
            return Err(MmexError::Validation("Payee name cannot be empty".into()));
        }
        let repo = SqlPayeeRepository::new(self.conn);
        repo.update(payee)
    }

    pub fn delete_payee(&self, id: PayeeId) -> Result<(), MmexError> {
        let repo = SqlPayeeRepository::new(self.conn);
        repo.delete(id)
    }
}
