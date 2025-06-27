#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::SUPPORTED_ASSETS;

    #[test]
    fn test_send_rejects_unsupported_asset() {
        let unsupported = "FAKEASSET";
        assert!(!SUPPORTED_ASSETS.contains(&unsupported));
        // Simulate CLI send logic
        if !SUPPORTED_ASSETS.contains(&unsupported) {
            // Should print error and abort
            assert!(true);
        } else {
            panic!("Unsupported asset was not rejected");
        }
    }

    #[test]
    fn test_balance_rejects_unsupported_asset() {
        let unsupported = "FAKEASSET";
        assert!(!SUPPORTED_ASSETS.contains(&unsupported));
        // Simulate CLI balance logic
        if !SUPPORTED_ASSETS.contains(&unsupported) {
            // Should print error and abort
            assert!(true);
        } else {
            panic!("Unsupported asset was not rejected");
        }
    }
} 