use crate::models::GixError;

/// Validate a gitignore pattern
pub fn validate_pattern(pattern: &str) -> Result<(), GixError> {
    // Basic validation - ensure pattern is not empty after trimming
    if pattern.trim().is_empty() {
        return Err(GixError::InvalidPattern(
            "Pattern cannot be empty".to_string(),
        ));
    }

    // Check for invalid characters or patterns
    // This is a basic implementation - could be expanded for more complex validation

    Ok(())
}

/// Check if a pattern is valid for gitignore
pub fn is_valid_pattern(pattern: &str) -> bool {
    validate_pattern(pattern).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_pattern() {
        assert!(validate_pattern("*.log").is_ok());
        assert!(validate_pattern("build/").is_ok());
        assert!(validate_pattern("!debug.log").is_ok());
        assert!(validate_pattern("\\#notacomment").is_ok());
    }

    #[test]
    fn test_validate_empty_pattern() {
        assert!(validate_pattern("").is_err());
        assert!(validate_pattern("   ").is_err());
    }

    #[test]
    fn test_is_valid_pattern() {
        assert!(is_valid_pattern("*.log"));
        assert!(is_valid_pattern("build/"));
        assert!(!is_valid_pattern(""));
        assert!(!is_valid_pattern("   "));
    }
}
