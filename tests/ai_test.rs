use findag_core::types::vote::{VoteType, Ballot};
use findag_core::security::{SecurityManager, SecurityConfig, SecurityAuditLog};
use findag_core::security::audit::SecuritySeverity;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Serialize, Deserialize};

// ... existing code ... 