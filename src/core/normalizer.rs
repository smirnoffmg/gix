/// Normalize a gitignore pattern for comparison
pub fn normalize_pattern(pattern: &str) -> String {
    pattern.trim().to_string()
}

/// Check if two patterns are equivalent (case-insensitive comparison)
pub fn patterns_equivalent(pattern1: &str, pattern2: &str) -> bool {
    normalize_pattern(pattern1) == normalize_pattern(pattern2)
}

/// Check if two patterns are equivalent with case sensitivity
pub fn patterns_equivalent_case_sensitive(pattern1: &str, pattern2: &str) -> bool {
    pattern1.trim() == pattern2.trim()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_pattern() {
        assert_eq!(normalize_pattern("*.log"), "*.log");
        assert_eq!(normalize_pattern(" *.log "), "*.log");
        assert_eq!(normalize_pattern("  *.log  "), "*.log");
    }

    #[test]
    fn test_patterns_equivalent() {
        assert!(patterns_equivalent("*.log", "*.log"));
        assert!(patterns_equivalent(" *.log ", "*.log"));
        assert!(!patterns_equivalent("*.log", "*.txt"));
    }

    #[test]
    fn test_patterns_equivalent_case_sensitive() {
        assert!(patterns_equivalent_case_sensitive("*.log", "*.log"));
        assert!(patterns_equivalent_case_sensitive(" *.log ", "*.log"));
        assert!(!patterns_equivalent_case_sensitive("*.log", "*.LOG"));
        assert!(!patterns_equivalent_case_sensitive("*.log", "*.txt"));
    }
}
