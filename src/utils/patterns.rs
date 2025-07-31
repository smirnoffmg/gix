/// Check if a pattern matches a file path
pub fn pattern_matches(pattern: &str, path: &str) -> bool {
    // This is a simplified implementation
    // In a real implementation, you would use proper glob pattern matching
    pattern == path
}

/// Check if two patterns are conflicting
pub fn patterns_conflict(pattern1: &str, pattern2: &str) -> bool {
    // This is a simplified implementation
    // In a real implementation, you would analyze pattern semantics
    pattern1 == pattern2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_matches() {
        assert!(pattern_matches("*.log", "*.log"));
        assert!(!pattern_matches("*.log", "*.txt"));
    }

    #[test]
    fn test_patterns_conflict() {
        assert!(patterns_conflict("*.log", "*.log"));
        assert!(!patterns_conflict("*.log", "*.txt"));
    }
} 