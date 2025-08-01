use std::collections::HashMap;

/// Represents the type of a gitignore pattern
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatternType {
    /// Matches files (e.g., "*.log", "file.txt")
    File,
    /// Matches directories (e.g., "build/", "node_modules/")
    Directory,
    /// Matches both files and directories (e.g., "build", "*.tmp")
    Both,
}

/// Represents the analysis of a gitignore pattern
#[derive(Debug, Clone)]
pub struct PatternAnalysis {
    /// The original pattern
    pub original: String,
    /// The normalized pattern
    pub normalized: String,
    /// Type of pattern
    pub pattern_type: PatternType,
    /// Whether this is a negation pattern (starts with !)
    pub is_negation: bool,
    /// Whether this is an absolute path (starts with /)
    pub is_absolute: bool,
    /// Whether this pattern has wildcards (*, ?, [])
    pub has_wildcards: bool,
    /// Whether this pattern has globstar (**)
    pub has_globstar: bool,
    /// Whether this pattern matches files
    pub matches_files: bool,
    /// Whether this pattern matches directories
    pub matches_directories: bool,
    /// Whether this pattern is case sensitive
    pub is_case_sensitive: bool,
}

impl PatternAnalysis {
    /// Create a new pattern analysis
    pub fn new(original: String, normalized: String) -> Self {
        let is_negation = normalized.starts_with('!');
        let pattern = if is_negation { &normalized[1..] } else { &normalized };
        
        let is_absolute = pattern.starts_with('/');
        let has_wildcards = pattern.contains('*') || pattern.contains('?') || pattern.contains('[');
        let has_globstar = pattern.contains("**");
        // Gitignore patterns are case-sensitive by default
        let is_case_sensitive = true;
        
        // Determine pattern type
        let pattern_type = if pattern.ends_with('/') {
            PatternType::Directory
        } else if has_wildcards || pattern.contains('.') {
            PatternType::Both
        } else {
            PatternType::File
        };
        
        let matches_files = matches!(pattern_type, PatternType::File | PatternType::Both);
        let matches_directories = matches!(pattern_type, PatternType::Directory | PatternType::Both);
        
        Self {
            original,
            normalized,
            pattern_type,
            is_negation,
            is_absolute,
            has_wildcards,
            has_globstar,
            matches_files,
            matches_directories,
            is_case_sensitive,
        }
    }
    
    /// Get the base pattern (without negation)
    pub fn base_pattern(&self) -> &str {
        if self.is_negation {
            &self.normalized[1..]
        } else {
            &self.normalized
        }
    }
    
    /// Check if this pattern could potentially conflict with another
    pub fn could_conflict_with(&self, other: &PatternAnalysis) -> bool {
        // Negation patterns can conflict with non-negation patterns
        if self.is_negation != other.is_negation {
            let base1 = self.base_pattern();
            let base2 = other.base_pattern();
            
            // Check if base patterns are equivalent
            self.are_base_patterns_equivalent(base1, base2)
        } else {
            false
        }
    }
    
    /// Check if two base patterns are functionally equivalent
    fn are_base_patterns_equivalent(&self, pattern1: &str, pattern2: &str) -> bool {
        // Exact match
        if pattern1 == pattern2 {
            return true;
        }
        
        // Handle trailing slash differences
        if pattern1.ends_with('/') && pattern2 == &pattern1[..pattern1.len()-1] {
            return true;
        }
        if pattern2.ends_with('/') && pattern1 == &pattern2[..pattern2.len()-1] {
            return true;
        }
        
        // Handle leading slash differences for relative patterns
        if pattern1.starts_with('/') && pattern2 == &pattern1[1..] {
            return true;
        }
        if pattern2.starts_with('/') && pattern1 == &pattern2[1..] {
            return true;
        }
        
        false
    }
}

/// Analyzer for gitignore patterns
pub struct PatternAnalyzer {
    /// Whether to normalize patterns (remove trailing spaces, etc.)
    pub normalize_patterns: bool,
    /// Whether to detect case-insensitive patterns
    pub case_sensitive: bool,
}

impl Default for PatternAnalyzer {
    fn default() -> Self {
        Self {
            normalize_patterns: true,
            case_sensitive: true,
        }
    }
}

impl PatternAnalyzer {
    /// Create a new pattern analyzer with custom settings
    pub fn new(normalize_patterns: bool, case_sensitive: bool) -> Self {
        Self {
            normalize_patterns,
            case_sensitive,
        }
    }
    
    /// Normalize a pattern by removing trailing spaces and handling separators
    pub fn normalize_pattern(&self, pattern: &str) -> String {
        if !self.normalize_patterns {
            return pattern.to_string();
        }
        
        let mut normalized = pattern.to_string();
        
        // Remove trailing spaces
        normalized = normalized.trim_end().to_string();
        
        // Normalize path separators (convert backslashes to forward slashes)
        if cfg!(windows) {
            normalized = normalized.replace('\\', "/");
        }
        
        // Remove duplicate slashes (except for globstar)
        let mut result = String::new();
        let mut chars = normalized.chars().peekable();
        
        while let Some(ch) = chars.next() {
            if ch == '/' {
                result.push(ch);
                // Skip consecutive slashes (but preserve globstar)
                while let Some(&next_ch) = chars.peek() {
                    if next_ch == '/' {
                        chars.next();
                    } else {
                        break;
                    }
                }
            } else {
                result.push(ch);
            }
        }
        
        result
    }
    
    /// Analyze a pattern and return detailed analysis
    pub fn analyze_pattern(&self, pattern: &str) -> PatternAnalysis {
        let normalized = self.normalize_pattern(pattern);
        PatternAnalysis::new(pattern.to_string(), normalized)
    }
    
    /// Check if two patterns are functionally equivalent
    pub fn are_equivalent(&self, pattern1: &str, pattern2: &str) -> bool {
        let analysis1 = self.analyze_pattern(pattern1);
        let analysis2 = self.analyze_pattern(pattern2);
        
        // Check if they're exactly the same after normalization
        if analysis1.normalized == analysis2.normalized {
            return true;
        }
        
        // Check if they're functionally equivalent
        analysis1.are_base_patterns_equivalent(analysis1.base_pattern(), analysis2.base_pattern())
    }
    
    /// Check if two patterns conflict (one negates the other)
    pub fn are_conflicting(&self, pattern1: &str, pattern2: &str) -> bool {
        let analysis1 = self.analyze_pattern(pattern1);
        let analysis2 = self.analyze_pattern(pattern2);
        
        analysis1.could_conflict_with(&analysis2)
    }
    
    /// Find all conflicts in a set of patterns
    pub fn find_conflicts(&self, patterns: &[String]) -> Vec<(String, String)> {
        let mut conflicts = Vec::new();
        
        for (i, pattern1) in patterns.iter().enumerate() {
            for pattern2 in patterns.iter().skip(i + 1) {
                if self.are_conflicting(pattern1, pattern2) {
                    conflicts.push((pattern1.clone(), pattern2.clone()));
                }
            }
        }
        
        conflicts
    }
    
    /// Group patterns by their base pattern (for deduplication)
    pub fn group_by_base_pattern(&self, patterns: &[String]) -> std::collections::HashMap<String, Vec<String>> {
        let mut groups = std::collections::HashMap::new();
        
        for pattern in patterns {
            let analysis = self.analyze_pattern(pattern);
            let base = analysis.base_pattern().to_string();
            let normalized_base = self.normalize_pattern(&base);
            groups.entry(normalized_base).or_insert_with(Vec::new).push(pattern.clone());
        }
        
        groups
    }
    
    /// Get a representative pattern from each group (for deduplication)
    pub fn get_representative_patterns(&self, patterns: &[String]) -> Vec<String> {
        let groups = self.group_by_base_pattern(patterns);
        let mut representatives = Vec::new();
        
        for (_base, group) in groups {
            // Choose the shortest pattern as representative
            let representative = group.iter()
                .min_by_key(|p| p.len())
                .unwrap()
                .clone();
            representatives.push(representative);
        }
        
        representatives
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_pattern_removes_trailing_spaces() {
        let analyzer = PatternAnalyzer::default();
        let normalized = analyzer.normalize_pattern("*.log ");
        assert_eq!(normalized, "*.log");
    }

    #[test]
    fn test_normalize_pattern_preserves_leading_spaces() {
        let analyzer = PatternAnalyzer::default();
        let normalized = analyzer.normalize_pattern(" *.log");
        assert_eq!(normalized, " *.log");
    }

    #[test]
    fn test_normalize_pattern_handles_duplicate_slashes() {
        let analyzer = PatternAnalyzer::default();
        let normalized = analyzer.normalize_pattern("build//output");
        assert_eq!(normalized, "build/output");
    }

    #[test]
    fn test_normalize_pattern_preserves_globstar() {
        let analyzer = PatternAnalyzer::default();
        let normalized = analyzer.normalize_pattern("**/node_modules");
        assert_eq!(normalized, "**/node_modules");
    }

    #[test]
    fn test_analyze_pattern_file_type() {
        let analyzer = PatternAnalyzer::default();
        let analysis = analyzer.analyze_pattern("*.log");
        
        assert_eq!(analysis.pattern_type, PatternType::Both);
        assert!(!analysis.is_negation);
        assert!(!analysis.is_absolute);
        assert!(analysis.has_wildcards);
        assert!(!analysis.has_globstar);
        assert!(analysis.matches_files);
        assert!(analysis.matches_directories);
    }

    #[test]
    fn test_analyze_pattern_directory_type() {
        let analyzer = PatternAnalyzer::default();
        let analysis = analyzer.analyze_pattern("build/");
        
        assert_eq!(analysis.pattern_type, PatternType::Directory);
        assert!(!analysis.is_negation);
        assert!(!analysis.is_absolute);
        assert!(!analysis.has_wildcards);
        assert!(!analysis.has_globstar);
        assert!(!analysis.matches_files);
        assert!(analysis.matches_directories);
    }

    #[test]
    fn test_analyze_pattern_negation() {
        let analyzer = PatternAnalyzer::default();
        let analysis = analyzer.analyze_pattern("!debug.log");
        
        assert!(analysis.is_negation);
        assert_eq!(analysis.base_pattern(), "debug.log");
    }

    #[test]
    fn test_analyze_pattern_absolute() {
        let analyzer = PatternAnalyzer::default();
        let analysis = analyzer.analyze_pattern("/build");
        
        assert!(analysis.is_absolute);
        assert!(!analysis.is_negation);
    }

    #[test]
    fn test_are_equivalent_exact_match() {
        let analyzer = PatternAnalyzer::default();
        assert!(analyzer.are_equivalent("*.log", "*.log"));
    }

    #[test]
    fn test_are_equivalent_trailing_slash() {
        let analyzer = PatternAnalyzer::default();
        assert!(analyzer.are_equivalent("build", "build/"));
        assert!(analyzer.are_equivalent("build/", "build"));
    }

    #[test]
    fn test_are_equivalent_leading_slash() {
        let analyzer = PatternAnalyzer::default();
        assert!(analyzer.are_equivalent("build", "/build"));
        assert!(analyzer.are_equivalent("/build", "build"));
    }

    #[test]
    fn test_are_not_equivalent_different_patterns() {
        let analyzer = PatternAnalyzer::default();
        assert!(!analyzer.are_equivalent("*.log", "*.tmp"));
    }

    #[test]
    fn test_are_conflicting_negation() {
        let analyzer = PatternAnalyzer::default();
        assert!(analyzer.are_conflicting("*.log", "!*.log"));
        assert!(analyzer.are_conflicting("!*.log", "*.log"));
    }

    #[test]
    fn test_are_conflicting_equivalent_base() {
        let analyzer = PatternAnalyzer::default();
        assert!(analyzer.are_conflicting("build/", "!build"));
        assert!(analyzer.are_conflicting("!build", "build/"));
    }

    #[test]
    fn test_are_not_conflicting_different_patterns() {
        let analyzer = PatternAnalyzer::default();
        assert!(!analyzer.are_conflicting("*.log", "*.tmp"));
        assert!(!analyzer.are_conflicting("*.log", "!*.tmp"));
    }

    #[test]
    fn test_find_conflicts() {
        let analyzer = PatternAnalyzer::default();
        let patterns = vec![
            "*.log".to_string(),
            "!*.log".to_string(),
            "build/".to_string(),
            "*.tmp".to_string(),
        ];
        
        let conflicts = analyzer.find_conflicts(&patterns);
        assert_eq!(conflicts.len(), 1);
        assert!((conflicts[0].0 == "*.log" && conflicts[0].1 == "!*.log") ||
                (conflicts[0].0 == "!*.log" && conflicts[0].1 == "*.log"));
    }

    #[test]
    fn test_group_by_base_pattern() {
        let analyzer = PatternAnalyzer::default();
        let patterns = vec![
            "*.log".to_string(),
            "*.log ".to_string(),
            "build/".to_string(),
            "build".to_string(),
        ];
        
        let groups = analyzer.group_by_base_pattern(&patterns);
        assert_eq!(groups.len(), 2); // *.log and build groups
        
        let log_group = groups.get("*.log").unwrap();
        assert_eq!(log_group.len(), 2);
        
        let build_group = groups.get("build").unwrap();
        assert_eq!(build_group.len(), 2);
    }

    #[test]
    fn test_get_representative_patterns() {
        let analyzer = PatternAnalyzer::default();
        let patterns = vec![
            "*.log ".to_string(),
            "*.log".to_string(),
            "build/".to_string(),
            "build".to_string(),
        ];
        
        let representatives = analyzer.get_representative_patterns(&patterns);
        assert_eq!(representatives.len(), 2);
        assert!(representatives.contains(&"*.log".to_string()));
        assert!(representatives.contains(&"build".to_string()));
    }
} 