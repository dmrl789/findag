use regex::Regex;
use once_cell::sync::Lazy;

static HANDLE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^@?[a-zA-Z0-9_\-]{3,32}(\.fd)?$").unwrap()
});

pub fn validate_handle(handle: &str) -> Result<(), String> {
    if !HANDLE_REGEX.is_match(handle) {
        return Err("Invalid handle format.".into());
    }
    Ok(())
}
