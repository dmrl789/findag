use std::error::Error;
use std::time::{SystemTime, Duration};
use serde::{Serialize, Deserialize};
use tokio::time;
use async_trait::async_trait;
use crate::blockchain::Transaction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoTransactionConfig {
    pub schedule: Schedule,
    pub transaction_template: Transaction,
    pub max_retries: u32,
    pub retry_delay: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Schedule {
    Interval(Duration),
    Cron(String),
    OneTime(SystemTime),
}

#[async_trait]
pub trait AutoTransactionExecutor {
    async fn execute(&self, transaction: &Transaction) -> Result<(), Box<dyn Error>>;
    async fn validate(&self, transaction: &Transaction) -> Result<bool, Box<dyn Error>>;
}

pub struct AutoTransactionProcessor {
    config: AutoTransactionConfig,
    executor: Box<dyn AutoTransactionExecutor + Send + Sync>,
}

impl AutoTransactionProcessor {
    pub fn new(config: AutoTransactionConfig, executor: Box<dyn AutoTransactionExecutor + Send + Sync>) -> Self {
        Self { config, executor }
    }

    pub async fn start(&self) -> Result<(), Box<dyn Error>> {
        match &self.config.schedule {
            Schedule::Interval(duration) => {
                let mut interval = time::interval(*duration);
                loop {
                    interval.tick().await;
                    self.process_transaction().await?;
                }
            }
            Schedule::Cron(cron_expr) => {
                // Implement cron-based scheduling
                todo!("Implement cron-based scheduling")
            }
            Schedule::OneTime(time) => {
                let now = SystemTime::now();
                if now >= *time {
                    self.process_transaction().await?;
                }
            }
        }
        Ok(())
    }

    async fn process_transaction(&self) -> Result<(), Box<dyn Error>> {
        let mut retries = 0;
        while retries < self.config.max_retries {
            if self.executor.validate(&self.config.transaction_template).await? {
                match self.executor.execute(&self.config.transaction_template).await {
                    Ok(_) => return Ok(()),
                    Err(e) => {
                        retries += 1;
                        if retries < self.config.max_retries {
                            time::sleep(self.config.retry_delay).await;
                            continue;
                        }
                        return Err(e);
                    }
                }
            }
            retries += 1;
        }
        Err("Max retries exceeded".into())
    }
} 