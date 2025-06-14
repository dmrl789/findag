use std::error::Error;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::types::transaction::Transaction;
use crate::utils::time::get_findag_time_micro;
use super::{
    SmartContract, ContractType, ClearingContract, LendingContract, ComplianceContract,
    Trade, Loan, AuditEntry,
};

pub struct ContractExecutor {
    contracts: Arc<Mutex<HashMap<String, SmartContract>>>,
    clearing_contracts: Arc<Mutex<HashMap<String, ClearingContract>>>,
    lending_contracts: Arc<Mutex<HashMap<String, LendingContract>>>,
    compliance_contracts: Arc<Mutex<HashMap<String, ComplianceContract>>>,
}

impl ContractExecutor {
    pub fn new() -> Self {
        Self {
            contracts: Arc::new(Mutex::new(HashMap::new())),
            clearing_contracts: Arc::new(Mutex::new(HashMap::new())),
            lending_contracts: Arc::new(Mutex::new(HashMap::new())),
            compliance_contracts: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn deploy_contract(&self, contract_type: ContractType, code: Vec<u8>) -> Result<String, Box<dyn Error>> {
        let contract = SmartContract::new(contract_type.clone(), code);
        let address = self.generate_contract_address(&contract);
        
        let mut contracts = self.contracts.lock().unwrap();
        contracts.insert(address.clone(), contract);

        // Initialize specific contract type
        match contract_type {
            ContractType::Clearing => {
                let mut clearing = self.clearing_contracts.lock().unwrap();
                clearing.insert(address.clone(), ClearingContract::new());
            }
            ContractType::Lending => {
                let mut lending = self.lending_contracts.lock().unwrap();
                lending.insert(address.clone(), LendingContract::new());
            }
            ContractType::Compliance => {
                let mut compliance = self.compliance_contracts.lock().unwrap();
                compliance.insert(address.clone(), ComplianceContract::new());
            }
        }

        Ok(address)
    }

    pub fn execute_transaction(&self, contract_address: &str, transaction: &Transaction) -> Result<(), Box<dyn Error>> {
        let contracts = self.contracts.lock().unwrap();
        if let Some(contract) = contracts.get(contract_address) {
            match contract.contract_type {
                ContractType::Clearing => {
                    let mut clearing = self.clearing_contracts.lock().unwrap();
                    if let Some(clearing_contract) = clearing.get_mut(contract_address) {
                        // Handle clearing-specific transaction
                        self.handle_clearing_transaction(clearing_contract, transaction)?;
                    }
                }
                ContractType::Lending => {
                    let mut lending = self.lending_contracts.lock().unwrap();
                    if let Some(lending_contract) = lending.get_mut(contract_address) {
                        // Handle lending-specific transaction
                        self.handle_lending_transaction(lending_contract, transaction)?;
                    }
                }
                ContractType::Compliance => {
                    let mut compliance = self.compliance_contracts.lock().unwrap();
                    if let Some(compliance_contract) = compliance.get_mut(contract_address) {
                        // Handle compliance-specific transaction
                        self.handle_compliance_transaction(compliance_contract, transaction)?;
                    }
                }
            }
        }
        Ok(())
    }

    fn handle_clearing_transaction(&self, contract: &mut ClearingContract, transaction: &Transaction) -> Result<(), Box<dyn Error>> {
        // Parse transaction data and execute clearing logic
        let trade = Trade {
            id: String::from_utf8(transaction.payload.clone())?,
            asset_id: String::from_utf8(transaction.data.clone())?,
            amount: transaction.amount,
            price: 0, // Extract from transaction data
            buyer: String::from_utf8(transaction.to.clone())?,
            seller: String::from_utf8(transaction.from.clone())?,
            timestamp: transaction.timestamp,
            status: super::TradeStatus::Pending,
        };

        contract.add_trade(trade)?;
        Ok(())
    }

    fn handle_lending_transaction(&self, contract: &mut LendingContract, transaction: &Transaction) -> Result<(), Box<dyn Error>> {
        // Parse transaction data and execute lending logic
        let loan = Loan {
            id: String::from_utf8(transaction.payload.clone())?,
            borrower: String::from_utf8(transaction.from.clone())?,
            lender: String::from_utf8(transaction.to.clone())?,
            amount: transaction.amount,
            interest_rate: 0.05, // Extract from transaction data
            collateral: HashMap::new(), // Extract from transaction data
            start_time: transaction.timestamp,
            end_time: transaction.timestamp + 30 * 24 * 60 * 60 * 1_000_000, // 30 days in microseconds
            status: super::LoanStatus::Active,
        };

        contract.create_loan(loan)?;
        Ok(())
    }

    fn handle_compliance_transaction(&self, contract: &mut ComplianceContract, transaction: &Transaction) -> Result<(), Box<dyn Error>> {
        // Parse transaction data and execute compliance logic
        let address = String::from_utf8(transaction.from.clone())?;
        
        // Verify KYC if needed
        if !contract.kyc_verified.contains(&address) {
            contract.verify_kyc(&address)?;
        }

        // Check transaction limits
        if !contract.verify_transaction(&address, transaction.amount) {
            return Err("Transaction limit exceeded".into());
        }

        Ok(())
    }

    fn generate_contract_address(&self, contract: &SmartContract) -> String {
        // Generate a unique contract address based on contract type and timestamp
        format!("fd2_{}_{}", 
            match contract.contract_type {
                ContractType::Clearing => "clear",
                ContractType::Lending => "lend",
                ContractType::Compliance => "comp",
            },
            contract.created_at
        )
    }
}
