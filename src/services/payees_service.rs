use crate::domain::payees::{Payee, PayeeError, PayeeId, PayeeRepository};
use crate::infrastructure::payees_repository::SqlPayeeRepository;
use rusqlite::Connection;

pub struct PayeeService<'a> {
    conn: &'a Connection,
}

impl<'a> PayeeService<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn get_all_payees(&self) -> Result<Vec<Payee>, PayeeError> {
        let repo = SqlPayeeRepository::new(self.conn);
        repo.find_all()
    }

    pub fn get_payee_by_id(&self, id: PayeeId) -> Result<Option<Payee>, PayeeError> {
        let repo = SqlPayeeRepository::new(self.conn);
        repo.find_by_id(id)
    }

    pub fn create_payee(&self, name: &str) -> Result<Payee, PayeeError> {
        if name.trim().is_empty() {
            return Err(PayeeError::NameRequired);
        }
        let repo = SqlPayeeRepository::new(self.conn);
        let new_payee = Payee {
            id: PayeeId { v1: 0 },
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

    pub fn update_payee(&self, payee: &Payee) -> Result<(), PayeeError> {
        if payee.name.trim().is_empty() {
            return Err(PayeeError::NameRequired);
        }
        let repo = SqlPayeeRepository::new(self.conn);
        repo.update(payee)
    }

    pub fn delete_payee(&self, id: PayeeId) -> Result<(), PayeeError> {
        let repo = SqlPayeeRepository::new(self.conn);
        repo.delete(id)
    }
}
