use regex::Regex;

pub fn validate_handle(handle: &str) -> Result<(), String> {
    let re = Regex::new(r"^@?[a-zA-Z0-9_\-]{3,32}(\.fd)?$").unwrap();
    if !re.is_match(handle) {
        return Err("Invalid handle format.".into());
    }
    Ok(())
}
