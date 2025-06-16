use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InstitutionType {
    Bank,
    CreditUnion,
    InvestmentFirm,
    InsuranceCompany,
    FinTech,
    RegulatoryBody,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserRole {
    Administrator,    // Full system access
    Compliance,       // Regulatory compliance monitoring
    Trader,          // Trading operations
    RiskManager,     // Risk assessment and management
    Auditor,         // Internal/external audit
    Support,         // Technical support
    Viewer,          // Read-only access
    LiquidityManager, // Manages cross-chain liquidity
    SettlementManager, // Manages transaction settlement
    NetworkManager,   // Manages network health and consensus
    EmergencyController, // Controls emergency procedures
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RegulatoryFramework {
    BaselIII,
    DoddFrank,
    MiFIDII,
    GDPR,
    SOX,
    AML,
    KYC,
    FATF,           // Financial Action Task Force
    CFT,            // Counter-Financing of Terrorism
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChainType {
    Mainnet,
    Testnet,
    Private,
    Consortium,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionType {
    Spot,
    CrossChain,
    SmartContract,
    LiquidityProvision,
    Staking,
    AtomicSwap,
    Settlement,
    Emergency,
    Consensus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionStatus {
    Pending,
    Validating,
    Confirmed,
    Settled,
    Failed,
    Reverted,
    EmergencyHalted,
    ConsensusPending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SettlementType {
    Instant,
    Deferred,
    Atomic,
    Conditional,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsensusType {
    PoS,    // Proof of Stake
    PoA,    // Proof of Authority
    BFT,    // Byzantine Fault Tolerance
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkStatus {
    Healthy,
    Degraded,
    Critical,
    Emergency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMetrics {
    pub status: NetworkStatus,
    pub node_count: u32,
    pub active_validators: u32,
    pub consensus_participation: f64,
    pub block_time: u64,
    pub transaction_throughput: f64,
    pub error_rate: f64,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusConfig {
    pub type_: ConsensusType,
    pub minimum_validators: u32,
    pub confirmation_threshold: f64,
    pub timeout_seconds: u64,
    pub emergency_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergencyControl {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub required_approvals: u32,
    pub received_approvals: u32,
    pub status: bool,
    pub last_activated: Option<DateTime<Utc>>,
    pub activation_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartContract {
    pub address: String,
    pub chain: ChainType,
    pub name: String,
    pub version: String,
    pub risk_level: u8,
    pub audit_status: bool,
    pub last_audit: DateTime<Utc>,
    pub liquidity_requirements: Option<LiquidityRequirement>,
    pub settlement_requirements: Option<SettlementRequirement>,
    pub consensus_requirements: Option<ConsensusConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettlementRequirement {
    pub type_: SettlementType,
    pub confirmation_blocks: u32,
    pub timeout_blocks: u32,
    pub required_signatures: u32,
    pub atomic_guarantee: bool,
    pub consensus_requirement: Option<ConsensusConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiquidityRequirement {
    pub minimum_liquidity: f64,
    pub target_liquidity: f64,
    pub maximum_slippage: f64,
    pub rebalance_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionLimit {
    pub daily_limit: f64,
    pub monthly_limit: f64,
    pub per_transaction_limit: f64,
    pub cooldown_period: u32,
    pub cross_chain_limit: Option<f64>,
    pub liquidity_provision_limit: Option<f64>,
    pub settlement_limit: Option<f64>,
    pub emergency_limit: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskMetrics {
    pub exposure: f64,
    pub volatility: f64,
    pub value_at_risk: f64,
    pub stress_test_results: HashMap<String, f64>,
    pub last_updated: DateTime<Utc>,
    pub network_metrics: Option<NetworkMetrics>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceStatus {
    pub framework: RegulatoryFramework,
    pub status: bool,
    pub last_check: DateTime<Utc>,
    pub next_check: DateTime<Utc>,
    pub violations: Vec<String>,
    pub chain_specific_compliance: HashMap<ChainType, bool>,
    pub reporting_requirements: Vec<ReportingRequirement>,
    pub risk_metrics: RiskMetrics,
    pub network_metrics: NetworkMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportingRequirement {
    pub report_type: String,
    pub frequency: String,
    pub last_submitted: Option<DateTime<Utc>>,
    pub next_due: DateTime<Utc>,
    pub required_data: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: Uuid,
    pub user_id: Uuid,
    pub action: String,
    pub timestamp: DateTime<Utc>,
    pub ip_address: String,
    pub details: HashMap<String, String>,
    pub chain_type: Option<ChainType>,
    pub smart_contract_address: Option<String>,
    pub transaction_type: Option<TransactionType>,
    pub cross_chain_details: Option<CrossChainDetails>,
    pub settlement_details: Option<SettlementDetails>,
    pub consensus_details: Option<ConsensusDetails>,
    pub emergency_details: Option<EmergencyDetails>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusDetails {
    pub consensus_type: ConsensusType,
    pub validator_count: u32,
    pub participation_rate: f64,
    pub confirmation_status: bool,
    pub block_number: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergencyDetails {
    pub control_id: Uuid,
    pub activation_reason: String,
    pub affected_services: Vec<String>,
    pub resolution_plan: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettlementDetails {
    pub settlement_type: SettlementType,
    pub status: TransactionStatus,
    pub confirmation_blocks: u32,
    pub current_block: u32,
    pub required_signatures: u32,
    pub received_signatures: u32,
    pub timeout_block: u32,
    pub consensus_status: Option<ConsensusDetails>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossChainDetails {
    pub source_chain: ChainType,
    pub destination_chain: ChainType,
    pub bridge_contract: String,
    pub validation_status: bool,
    pub liquidity_check: bool,
    pub atomic_guarantee: bool,
    pub consensus_requirement: Option<ConsensusConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskProfile {
    pub risk_level: u8,
    pub trading_limits: HashMap<String, TransactionLimit>,
    pub restricted_assets: Vec<String>,
    pub required_approvals: Vec<UserRole>,
    pub allowed_chains: Vec<ChainType>,
    pub allowed_smart_contracts: Vec<SmartContract>,
    pub cross_chain_limits: HashMap<(ChainType, ChainType), f64>,
    pub liquidity_requirements: Option<LiquidityRequirement>,
    pub settlement_requirements: Option<SettlementRequirement>,
    pub risk_metrics: RiskMetrics,
    pub emergency_controls: Vec<EmergencyControl>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Institution {
    pub id: Uuid,
    pub name: String,
    pub institution_type: InstitutionType,
    pub regulatory_id: String,
    pub jurisdiction: String,
    pub compliance_status: Vec<ComplianceStatus>,
    pub risk_profile: RiskProfile,
    pub supported_chains: Vec<ChainType>,
    pub smart_contract_whitelist: Vec<SmartContract>,
    pub cross_chain_bridges: Vec<CrossChainBridge>,
    pub liquidity_pools: Vec<LiquidityPool>,
    pub settlement_contracts: Vec<SettlementContract>,
    pub consensus_config: ConsensusConfig,
    pub emergency_controls: Vec<EmergencyControl>,
    pub network_metrics: NetworkMetrics,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettlementContract {
    pub id: Uuid,
    pub address: String,
    pub chain: ChainType,
    pub type_: SettlementType,
    pub risk_level: u8,
    pub minimum_confirmations: u32,
    pub timeout_blocks: u32,
    pub required_signatures: u32,
    pub last_audit: DateTime<Utc>,
    pub consensus_requirements: ConsensusConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossChainBridge {
    pub id: Uuid,
    pub source_chain: ChainType,
    pub destination_chain: ChainType,
    pub contract_address: String,
    pub risk_level: u8,
    pub daily_volume_limit: f64,
    pub minimum_liquidity: f64,
    pub settlement_requirements: SettlementRequirement,
    pub consensus_requirements: ConsensusConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiquidityPool {
    pub id: Uuid,
    pub chain: ChainType,
    pub asset: String,
    pub current_liquidity: f64,
    pub target_liquidity: f64,
    pub minimum_liquidity: f64,
    pub last_rebalance: DateTime<Utc>,
    pub settlement_requirements: SettlementRequirement,
    pub consensus_requirements: ConsensusConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub institution_id: Uuid,
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub role: UserRole,
    pub department: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    pub last_transaction: Option<DateTime<Utc>>,
    pub enabled: bool,
    pub two_factor_enabled: bool,
    pub access_level: u8,
    pub risk_profile: RiskProfile,
    pub audit_logs: Vec<AuditLog>,
    pub cross_chain_permissions: Vec<(ChainType, ChainType)>,
    pub settlement_permissions: Vec<SettlementType>,
    pub emergency_permissions: Vec<Uuid>,
}

impl Institution {
    pub fn new(
        name: String,
        institution_type: InstitutionType,
        regulatory_id: String,
        jurisdiction: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            institution_type,
            regulatory_id,
            jurisdiction,
            compliance_status: vec![
                ComplianceStatus {
                    framework: RegulatoryFramework::BaselIII,
                    status: true,
                    last_check: now,
                    next_check: now + chrono::Duration::days(30),
                    violations: Vec::new(),
                    chain_specific_compliance: HashMap::new(),
                    reporting_requirements: vec![
                        ReportingRequirement {
                            report_type: "Daily Transaction Report".to_string(),
                            frequency: "Daily".to_string(),
                            last_submitted: None,
                            next_due: now + chrono::Duration::days(1),
                            required_data: vec!["transactions".to_string(), "volumes".to_string()],
                        }
                    ],
                    risk_metrics: RiskMetrics {
                        exposure: 0.0,
                        volatility: 0.0,
                        value_at_risk: 0.0,
                        stress_test_results: HashMap::new(),
                        last_updated: now,
                        network_metrics: None,
                    },
                    network_metrics: NetworkMetrics {
                        status: NetworkStatus::Healthy,
                        node_count: 0,
                        active_validators: 0,
                        consensus_participation: 1.0,
                        block_time: 0,
                        transaction_throughput: 0.0,
                        error_rate: 0.0,
                        last_updated: now,
                    },
                }
            ],
            risk_profile: RiskProfile {
                risk_level: 5,
                trading_limits: HashMap::new(),
                restricted_assets: Vec::new(),
                required_approvals: vec![UserRole::Compliance, UserRole::RiskManager],
                allowed_chains: vec![ChainType::Mainnet],
                allowed_smart_contracts: Vec::new(),
                cross_chain_limits: HashMap::new(),
                liquidity_requirements: Some(LiquidityRequirement {
                    minimum_liquidity: 1000000.0,
                    target_liquidity: 5000000.0,
                    maximum_slippage: 0.01,
                    rebalance_threshold: 0.1,
                }),
                settlement_requirements: Some(SettlementRequirement {
                    type_: SettlementType::Atomic,
                    confirmation_blocks: 12,
                    timeout_blocks: 100,
                    required_signatures: 3,
                    atomic_guarantee: true,
                    consensus_requirement: Some(ConsensusConfig {
                        type_: ConsensusType::BFT,
                        minimum_validators: 4,
                        confirmation_threshold: 0.67,
                        timeout_seconds: 30,
                        emergency_threshold: 0.33,
                    }),
                }),
                risk_metrics: RiskMetrics {
                    exposure: 0.0,
                    volatility: 0.0,
                    value_at_risk: 0.0,
                    stress_test_results: HashMap::new(),
                    last_updated: now,
                    network_metrics: None,
                },
                emergency_controls: vec![
                    EmergencyControl {
                        id: Uuid::new_v4(),
                        name: "System Halt".to_string(),
                        description: "Emergency system halt for critical issues".to_string(),
                        required_approvals: 3,
                        received_approvals: 0,
                        status: false,
                        last_activated: None,
                        activation_threshold: 0.67,
                    }
                ],
            },
            supported_chains: vec![ChainType::Mainnet],
            smart_contract_whitelist: Vec::new(),
            cross_chain_bridges: Vec::new(),
            liquidity_pools: Vec::new(),
            settlement_contracts: Vec::new(),
            consensus_config: ConsensusConfig {
                type_: ConsensusType::BFT,
                minimum_validators: 4,
                confirmation_threshold: 0.67,
                timeout_seconds: 30,
                emergency_threshold: 0.33,
            },
            emergency_controls: vec![
                EmergencyControl {
                    id: Uuid::new_v4(),
                    name: "System Halt".to_string(),
                    description: "Emergency system halt for critical issues".to_string(),
                    required_approvals: 3,
                    received_approvals: 0,
                    status: false,
                    last_activated: None,
                    activation_threshold: 0.67,
                }
            ],
            network_metrics: NetworkMetrics {
                status: NetworkStatus::Healthy,
                node_count: 0,
                active_validators: 0,
                consensus_participation: 1.0,
                block_time: 0,
                transaction_throughput: 0.0,
                error_rate: 0.0,
                last_updated: now,
            },
            created_at: now,
            updated_at: now,
        }
    }

    pub fn add_settlement_contract(&mut self, contract: SettlementContract) {
        self.settlement_contracts.push(contract);
        self.updated_at = Utc::now();
    }

    pub fn update_risk_metrics(&mut self, metrics: RiskMetrics) {
        self.risk_profile.risk_metrics = metrics;
        self.updated_at = Utc::now();
    }

    pub fn check_settlement_requirements(&self, amount: f64, settlement_type: &SettlementType) -> bool {
        if let Some(requirements) = &self.risk_profile.settlement_requirements {
            if requirements.type_ == *settlement_type {
                // Check if amount is within limits
                if let Some(limit) = self.risk_profile.trading_limits.get("SETTLEMENT") {
                    return amount <= limit.settlement_limit.unwrap_or(f64::MAX);
                }
            }
        }
        false
    }

    pub fn update_network_metrics(&mut self, metrics: NetworkMetrics) {
        self.network_metrics = metrics;
        self.updated_at = Utc::now();
    }

    pub fn activate_emergency_control(&mut self, control_id: Uuid) -> bool {
        if let Some(control) = self.emergency_controls.iter_mut()
            .find(|c| c.id == control_id) {
            control.received_approvals += 1;
            if control.received_approvals >= control.required_approvals {
                control.status = true;
                control.last_activated = Some(Utc::now());
                return true;
            }
        }
        false
    }

    pub fn check_consensus_health(&self) -> bool {
        self.network_metrics.consensus_participation >= self.consensus_config.confirmation_threshold
    }
}

impl User {
    pub fn new(
        institution_id: Uuid,
        username: String,
        email: String,
        first_name: String,
        last_name: String,
        role: UserRole,
        department: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            institution_id,
            username,
            email,
            first_name,
            last_name,
            role,
            department,
            created_at: now,
            updated_at: now,
            last_login: None,
            last_transaction: None,
            enabled: true,
            two_factor_enabled: true,
            access_level: 1,
            risk_profile: RiskProfile {
                risk_level: 1,
                trading_limits: HashMap::new(),
                restricted_assets: Vec::new(),
                required_approvals: vec![UserRole::Compliance],
                allowed_chains: vec![ChainType::Mainnet],
                allowed_smart_contracts: Vec::new(),
                cross_chain_limits: HashMap::new(),
                liquidity_requirements: None,
                settlement_requirements: Some(SettlementRequirement {
                    type_: SettlementType::Atomic,
                    confirmation_blocks: 12,
                    timeout_blocks: 100,
                    required_signatures: 3,
                    atomic_guarantee: true,
                    consensus_requirement: Some(ConsensusConfig {
                        type_: ConsensusType::BFT,
                        minimum_validators: 4,
                        confirmation_threshold: 0.67,
                        timeout_seconds: 30,
                        emergency_threshold: 0.33,
                    }),
                }),
                risk_metrics: RiskMetrics {
                    exposure: 0.0,
                    volatility: 0.0,
                    value_at_risk: 0.0,
                    stress_test_results: HashMap::new(),
                    last_updated: now,
                    network_metrics: None,
                },
                emergency_controls: Vec::new(),
            },
            audit_logs: Vec::new(),
            cross_chain_permissions: Vec::new(),
            settlement_permissions: vec![SettlementType::Atomic],
            emergency_permissions: Vec::new(),
        }
    }

    pub fn can_execute_settlement(
        &self,
        amount: f64,
        settlement_type: &SettlementType,
        chain: &ChainType,
    ) -> bool {
        // Check if user has permission for this settlement type
        if !self.settlement_permissions.contains(settlement_type) {
            return false;
        }

        // Check if chain is allowed
        if !self.risk_profile.allowed_chains.contains(chain) {
            return false;
        }

        // Check settlement limits
        if let Some(limit) = self.risk_profile.trading_limits.get("SETTLEMENT") {
            if let Some(settlement_limit) = limit.settlement_limit {
                if amount > settlement_limit {
                    return false;
                }
            }
        }

        true
    }

    pub fn log_settlement(
        &mut self,
        amount: f64,
        settlement_type: SettlementType,
        chain: ChainType,
        status: TransactionStatus,
        confirmation_blocks: u32,
        current_block: u32,
    ) {
        let settlement_details = SettlementDetails {
            settlement_type: settlement_type.clone(),
            status,
            confirmation_blocks,
            current_block,
            required_signatures: 3,
            received_signatures: 0,
            timeout_block: current_block + 100,
            consensus_status: None,
        };

        let mut details = HashMap::new();
        details.insert("amount".to_string(), amount.to_string());
        details.insert("settlement_type".to_string(), format!("{:?}", settlement_type));
        details.insert("chain".to_string(), format!("{:?}", chain));

        self.log_action(
            "SETTLEMENT_EXECUTED".to_string(),
            "127.0.0.1".to_string(),
            details,
            Some(chain),
            None,
            Some(TransactionType::Settlement),
            None,
            Some(settlement_details),
            None,
            None,
        );
    }

    pub fn log_action(
        &mut self,
        action: String,
        ip_address: String,
        details: HashMap<String, String>,
        chain_type: Option<ChainType>,
        smart_contract_address: Option<String>,
        transaction_type: Option<TransactionType>,
        cross_chain_details: Option<CrossChainDetails>,
        settlement_details: Option<SettlementDetails>,
        consensus_details: Option<ConsensusDetails>,
        emergency_details: Option<EmergencyDetails>,
    ) {
        let log = AuditLog {
            id: Uuid::new_v4(),
            user_id: self.id,
            action,
            timestamp: Utc::now(),
            ip_address,
            details,
            chain_type,
            smart_contract_address,
            transaction_type,
            cross_chain_details,
            settlement_details,
            consensus_details,
            emergency_details,
        };
        self.audit_logs.push(log);
    }

    pub fn can_execute_cross_chain_transaction(
        &self,
        amount: f64,
        source_chain: &ChainType,
        destination_chain: &ChainType,
    ) -> bool {
        // Check if user has permission for this chain pair
        if !self.cross_chain_permissions.contains(&(source_chain.clone(), destination_chain.clone())) {
            return false;
        }

        // Check cross-chain limits
        if let Some(limit) = self.risk_profile.cross_chain_limits.get(&(source_chain.clone(), destination_chain.clone())) {
            if amount > *limit {
                return false;
            }
        }

        // Check if chains are allowed
        if !self.risk_profile.allowed_chains.contains(source_chain) ||
           !self.risk_profile.allowed_chains.contains(destination_chain) {
            return false;
        }

        true
    }

    pub fn log_cross_chain_transaction(
        &mut self,
        amount: f64,
        source_chain: ChainType,
        destination_chain: ChainType,
        bridge_contract: String,
    ) {
        let cross_chain_details = CrossChainDetails {
            source_chain: source_chain.clone(),
            destination_chain: destination_chain.clone(),
            bridge_contract,
            validation_status: true,
            liquidity_check: true,
            atomic_guarantee: true,
            consensus_requirement: None,
        };

        let mut details = HashMap::new();
        details.insert("amount".to_string(), amount.to_string());
        details.insert("source_chain".to_string(), format!("{:?}", source_chain));
        details.insert("destination_chain".to_string(), format!("{:?}", destination_chain));

        self.log_action(
            "CROSS_CHAIN_TRANSACTION".to_string(),
            "127.0.0.1".to_string(),
            details,
            Some(source_chain),
            None,
            Some(TransactionType::CrossChain),
            Some(cross_chain_details),
            None,
            None,
            None,
        );
    }

    pub fn can_execute_transaction(&self, amount: f64, asset: &str, chain: &ChainType) -> bool {
        // Check if chain is allowed
        if !self.risk_profile.allowed_chains.contains(chain) {
            return false;
        }

        // Check trading limits
        if let Some(limits) = self.risk_profile.trading_limits.get(asset) {
            if amount > limits.per_transaction_limit {
                return false;
            }

            // Check cooldown period
            if let Some(last_tx) = self.last_transaction {
                let cooldown = chrono::Duration::seconds(limits.cooldown_period as i64);
                if Utc::now() - last_tx < cooldown {
                    return false;
                }
            }
        }

        true
    }

    pub fn log_transaction(&mut self, amount: f64, asset: &str, chain: ChainType, contract_address: Option<String>) {
        self.last_transaction = Some(Utc::now());
        
        let mut details = HashMap::new();
        details.insert("amount".to_string(), amount.to_string());
        details.insert("asset".to_string(), asset.to_string());
        
        self.log_action(
            "TRANSACTION_EXECUTED".to_string(),
            "127.0.0.1".to_string(), // This should be the actual IP
            details,
            Some(chain),
            contract_address,
            None,
            None,
            None,
            None,
            None,
        );
    }

    pub fn update_risk_profile(&mut self, risk_level: u8, trading_limits: HashMap<String, TransactionLimit>) {
        self.risk_profile.risk_level = risk_level.min(10);
        self.risk_profile.trading_limits = trading_limits;
        self.updated_at = Utc::now();
    }

    pub fn add_restricted_asset(&mut self, asset: String) {
        self.risk_profile.restricted_assets.push(asset);
        self.updated_at = Utc::now();
    }

    pub fn add_allowed_chain(&mut self, chain: ChainType) {
        if !self.risk_profile.allowed_chains.contains(&chain) {
            self.risk_profile.allowed_chains.push(chain);
            self.updated_at = Utc::now();
        }
    }

    pub fn add_allowed_smart_contract(&mut self, contract: SmartContract) {
        self.risk_profile.allowed_smart_contracts.push(contract);
        self.updated_at = Utc::now();
    }

    pub fn update_last_login(&mut self) {
        self.last_login = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    pub fn disable(&mut self) {
        self.enabled = false;
        self.updated_at = Utc::now();
    }

    pub fn enable(&mut self) {
        self.enabled = true;
        self.updated_at = Utc::now();
    }

    pub fn set_access_level(&mut self, level: u8) -> Result<(), &'static str> {
        if level > 5 {
            return Err("Access level cannot exceed 5");
        }
        self.access_level = level;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn can_activate_emergency_control(&self, control_id: Uuid) -> bool {
        self.emergency_permissions.contains(&control_id)
    }

    pub fn log_emergency_action(
        &mut self,
        control_id: Uuid,
        reason: String,
        affected_services: Vec<String>,
        resolution_plan: String,
    ) {
        let emergency_details = EmergencyDetails {
            control_id,
            activation_reason: reason,
            affected_services,
            resolution_plan,
        };

        let mut details = HashMap::new();
        details.insert("control_id".to_string(), control_id.to_string());

        self.log_action(
            "EMERGENCY_ACTION".to_string(),
            "127.0.0.1".to_string(),
            details,
            None,
            None,
            Some(TransactionType::Emergency),
            None,
            None,
            None,
            Some(emergency_details),
        );
    }

    pub fn log_consensus_action(
        &mut self,
        consensus_type: ConsensusType,
        validator_count: u32,
        participation_rate: f64,
        confirmation_status: bool,
        block_number: u64,
    ) {
        let consensus_details = ConsensusDetails {
            consensus_type,
            validator_count,
            participation_rate,
            confirmation_status,
            block_number,
        };

        let mut details = HashMap::new();
        details.insert("block_number".to_string(), block_number.to_string());
        details.insert("participation_rate".to_string(), participation_rate.to_string());

        self.log_action(
            "CONSENSUS_ACTION".to_string(),
            "127.0.0.1".to_string(),
            details,
            None,
            None,
            Some(TransactionType::Consensus),
            None,
            None,
            Some(consensus_details),
            None,
        );
    }
}

// Example usage:
pub fn create_example_institution() -> Institution {
    let mut institution = Institution::new(
        "Global Investment Bank".to_string(),
        InstitutionType::Bank,
        "FDIC123456".to_string(),
        "United States".to_string(),
    );
    
    // Add settlement contract with consensus requirements
    institution.add_settlement_contract(SettlementContract {
        id: Uuid::new_v4(),
        address: "0x789...".to_string(),
        chain: ChainType::Mainnet,
        type_: SettlementType::Atomic,
        risk_level: 2,
        minimum_confirmations: 12,
        timeout_blocks: 100,
        required_signatures: 3,
        last_audit: Utc::now(),
        consensus_requirements: ConsensusConfig {
            type_: ConsensusType::BFT,
            minimum_validators: 4,
            confirmation_threshold: 0.67,
            timeout_seconds: 30,
            emergency_threshold: 0.33,
        },
    });
    
    // Add liquidity pool with consensus requirements
    institution.add_liquidity_pool(LiquidityPool {
        id: Uuid::new_v4(),
        chain: ChainType::Mainnet,
        asset: "USDC".to_string(),
        current_liquidity: 5000000.0,
        target_liquidity: 10000000.0,
        minimum_liquidity: 1000000.0,
        last_rebalance: Utc::now(),
        settlement_requirements: SettlementRequirement {
            type_: SettlementType::Atomic,
            confirmation_blocks: 12,
            timeout_blocks: 100,
            required_signatures: 3,
            atomic_guarantee: true,
            consensus_requirement: Some(ConsensusConfig {
                type_: ConsensusType::BFT,
                minimum_validators: 4,
                confirmation_threshold: 0.67,
                timeout_seconds: 30,
                emergency_threshold: 0.33,
            }),
        },
        consensus_requirements: ConsensusConfig {
            type_: ConsensusType::BFT,
            minimum_validators: 4,
            confirmation_threshold: 0.67,
            timeout_seconds: 30,
            emergency_threshold: 0.33,
        },
    });
    
    institution
}

pub fn create_example_user(institution_id: Uuid) -> User {
    let mut user = User::new(
        institution_id,
        "jdoe".to_string(),
        "john.doe@globalbank.com".to_string(),
        "John".to_string(),
        "Doe".to_string(),
        UserRole::Trader,
        "Trading Desk".to_string(),
    );
    
    // Add emergency permissions
    user.emergency_permissions.push(Uuid::new_v4());
    
    // Set up trading limits with emergency limits
    let mut trading_limits = HashMap::new();
    trading_limits.insert("BTC".to_string(), TransactionLimit {
        daily_limit: 1000000.0,
        monthly_limit: 10000000.0,
        per_transaction_limit: 100000.0,
        cooldown_period: 300,
        cross_chain_limit: Some(50000.0),
        liquidity_provision_limit: Some(200000.0),
        settlement_limit: Some(1000000.0),
        emergency_limit: Some(5000000.0),
    });
    
    user.update_risk_profile(3, trading_limits);
    user.add_allowed_chain(ChainType::Mainnet);
    user.add_allowed_chain(ChainType::Private);
    
    user
} 