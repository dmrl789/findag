pub mod types;
pub mod validation;

// Explicitly re-export only necessary types
pub use types::{Domain, DomainAnalysis, DomainValidationResult};
pub use validation::validate_domain;
// pub use validation::validate_domain_analysis; // REMOVE, does not exist 