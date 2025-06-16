use std::error::Error;
use async_trait::async_trait;
use crate::blockchain::transaction::Transaction;
use crate::blockchain::auto_transaction::AutoTransactionExecutor;

pub struct DefaultAutoTransactionExecutor;

impl DefaultAutoTransactionExecutor {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl AutoTransactionExecutor for DefaultAutoTransactionExecutor {
    async fn execute(&self, transaction: &Transaction) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    async fn validate(&self, transaction: &Transaction) -> Result<bool, Box<dyn Error>> {
        Ok(true)
    }
}