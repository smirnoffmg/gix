use crate::models::{GitignoreFile, GixError};
use crate::core::pattern_analyzer::{PatternAnalyzer, PatternAnalysis};
use std::collections::{HashSet, HashMap};

/// Optimize a gitignore file by removing duplicate patterns while preserving structure
pub fn optimize_gitignore(file: &GitignoreFile) -> Result<GitignoreFile, GixError> {
    let analyzer = PatternAnalyzer::default();
    optimize_gitignore_with_analyzer(file, &analyzer)
}

/// Optimize a gitignore file with more aggressive deduplication
pub fn optimize_gitignore_aggressive(file: &GitignoreFile) -> Result<GitignoreFile, GixError> {
    let analyzer = PatternAnalyzer::default();
    optimize_gitignore_aggressive_with_analyzer(file, &analyzer)
}

/// Optimize a gitignore file using a specific pattern analyzer
pub fn optimize_gitignore_with_analyzer(file: &GitignoreFile, analyzer: &PatternAnalyzer) -> Result<GitignoreFile, GixError> {
    let mut optimized = GitignoreFile::new();
    let mut seen_patterns: HashSet<String> = HashSet::new();
    let mut pattern_analyses: HashMap<String, PatternAnalysis> = HashMap::new();
    
    // First pass: collect all patterns and their analyses
    for entry in &file.entries {
        if let crate::models::EntryType::Pattern(pattern) = &entry.entry_type {
            let analysis = analyzer.analyze_pattern(pattern);
            pattern_analyses.insert(pattern.clone(), analysis);
        }
    }
    
    // Second pass: deduplicate patterns using analysis
    for entry in &file.entries {
        match &entry.entry_type {
            crate::models::EntryType::Pattern(pattern) => {
                let analysis = &pattern_analyses[pattern];
                let normalized = &analysis.normalized;
                
                // Check if we've seen an equivalent pattern
                let mut should_add = true;
                for seen_pattern in &seen_patterns {
                    if analyzer.are_equivalent(pattern, seen_pattern) {
                        should_add = false;
                        break;
                    }
                }
                
                if should_add {
                    seen_patterns.insert(pattern.clone()); // Use original pattern, not normalized
                    optimized.add_entry(entry.clone());
                }
            }
            crate::models::EntryType::Comment(_) | crate::models::EntryType::Blank => {
                // Always preserve comments and blank lines
                optimized.add_entry(entry.clone());
            }
        }
    }
    
    Ok(optimized)
}

/// Optimize a gitignore file with aggressive deduplication using a specific analyzer
pub fn optimize_gitignore_aggressive_with_analyzer(file: &GitignoreFile, analyzer: &PatternAnalyzer) -> Result<GitignoreFile, GixError> {
    let mut optimized = GitignoreFile::new();
    let mut seen_patterns: HashSet<String> = HashSet::new();
    let mut seen_comments: HashSet<String> = HashSet::new();
    let mut pattern_analyses: HashMap<String, PatternAnalysis> = HashMap::new();
    
    // First pass: collect all patterns and their analyses
    for entry in &file.entries {
        if let crate::models::EntryType::Pattern(pattern) = &entry.entry_type {
            let analysis = analyzer.analyze_pattern(pattern);
            pattern_analyses.insert(pattern.clone(), analysis);
        }
    }
    
    // Second pass: aggressive deduplication
    for entry in &file.entries {
        match &entry.entry_type {
            crate::models::EntryType::Pattern(pattern) => {
                let analysis = &pattern_analyses[pattern];
                let normalized = &analysis.normalized;
                
                // Check if we've seen an equivalent pattern
                let mut should_add = true;
                for seen_pattern in &seen_patterns {
                    if analyzer.are_equivalent(pattern, seen_pattern) {
                        should_add = false;
                        break;
                    }
                }
                
                if should_add {
                    seen_patterns.insert(pattern.clone());
                    optimized.add_entry(entry.clone());
                }
            }
            crate::models::EntryType::Comment(comment) => {
                let normalized = comment.trim();
                
                // Only deduplicate identical comments
                if !seen_comments.contains(normalized) {
                    seen_comments.insert(normalized.to_string());
                    optimized.add_entry(entry.clone());
                }
            }
            crate::models::EntryType::Blank => {
                // Preserve blank lines but limit consecutive ones
                if optimized.entries.is_empty() || 
                   !matches!(optimized.entries.last().unwrap().entry_type, crate::models::EntryType::Blank) {
                    optimized.add_entry(entry.clone());
                }
            }
        }
    }
    
    Ok(optimized)
}

/// Optimize a gitignore file with conflict detection
pub fn optimize_gitignore_with_conflicts(file: &GitignoreFile) -> Result<(GitignoreFile, Vec<(String, String)>), GixError> {
    let analyzer = PatternAnalyzer::default();
    let mut optimized = GitignoreFile::new();
    let mut seen_patterns: HashSet<String> = HashSet::new();
    let mut pattern_analyses: HashMap<String, PatternAnalysis> = HashMap::new();
    
    // First pass: collect all patterns and their analyses
    for entry in &file.entries {
        if let crate::models::EntryType::Pattern(pattern) = &entry.entry_type {
            let analysis = analyzer.analyze_pattern(pattern);
            pattern_analyses.insert(pattern.clone(), analysis);
        }
    }
    
    // Find conflicts
    let pattern_strings: Vec<String> = file.entries.iter()
        .filter_map(|entry| {
            if let crate::models::EntryType::Pattern(pattern) = &entry.entry_type {
                Some(pattern.clone())
            } else {
                None
            }
        })
        .collect();
    
    let conflicts = analyzer.find_conflicts(&pattern_strings);
    
    // Second pass: deduplicate patterns using analysis
    for entry in &file.entries {
        match &entry.entry_type {
            crate::models::EntryType::Pattern(pattern) => {
                let analysis = &pattern_analyses[pattern];
                let normalized = &analysis.normalized;
                
                // Check if we've seen an equivalent pattern
                let mut should_add = true;
                for seen_pattern in &seen_patterns {
                    if analyzer.are_equivalent(pattern, seen_pattern) {
                        should_add = false;
                        break;
                    }
                }
                
                if should_add {
                    seen_patterns.insert(pattern.clone()); // Use original pattern, not normalized
                    optimized.add_entry(entry.clone());
                }
            }
            crate::models::EntryType::Comment(_) | crate::models::EntryType::Blank => {
                // Always preserve comments and blank lines
                optimized.add_entry(entry.clone());
            }
        }
    }
    
    Ok((optimized, conflicts))
}

/// Get detailed analysis of a gitignore file
pub fn analyze_gitignore(file: &GitignoreFile) -> Result<GitignoreAnalysis, GixError> {
    let analyzer = PatternAnalyzer::default();
    let mut analysis = GitignoreAnalysis::new();
    
    for entry in &file.entries {
        if let crate::models::EntryType::Pattern(pattern) = &entry.entry_type {
            let pattern_analysis = analyzer.analyze_pattern(pattern);
            analysis.add_pattern_analysis(pattern_analysis);
        }
    }
    
    // Find conflicts
    let pattern_strings: Vec<String> = file.entries.iter()
        .filter_map(|entry| {
            if let crate::models::EntryType::Pattern(pattern) = &entry.entry_type {
                Some(pattern.clone())
            } else {
                None
            }
        })
        .collect();
    
    analysis.conflicts = analyzer.find_conflicts(&pattern_strings);
    
    Ok(analysis)
}

/// Analysis results for a gitignore file
#[derive(Debug, Clone)]
pub struct GitignoreAnalysis {
    /// Total number of patterns
    pub total_patterns: usize,
    /// Number of file patterns
    pub file_patterns: usize,
    /// Number of directory patterns
    pub directory_patterns: usize,
    /// Number of patterns that match both files and directories
    pub both_patterns: usize,
    /// Number of negation patterns
    pub negation_patterns: usize,
    /// Number of absolute path patterns
    pub absolute_patterns: usize,
    /// Number of patterns with wildcards
    pub wildcard_patterns: usize,
    /// Number of patterns with globstar
    pub globstar_patterns: usize,
    /// Number of case-sensitive patterns
    pub case_sensitive_patterns: usize,
    /// Number of case-insensitive patterns
    pub case_insensitive_patterns: usize,
    /// List of conflicting patterns
    pub conflicts: Vec<(String, String)>,
    /// Pattern analyses
    pub pattern_analyses: Vec<PatternAnalysis>,
}

impl GitignoreAnalysis {
    pub fn new() -> Self {
        Self {
            total_patterns: 0,
            file_patterns: 0,
            directory_patterns: 0,
            both_patterns: 0,
            negation_patterns: 0,
            absolute_patterns: 0,
            wildcard_patterns: 0,
            globstar_patterns: 0,
            case_sensitive_patterns: 0,
            case_insensitive_patterns: 0,
            conflicts: Vec::new(),
            pattern_analyses: Vec::new(),
        }
    }
    
    pub fn add_pattern_analysis(&mut self, analysis: PatternAnalysis) {
        self.total_patterns += 1;
        
        match analysis.pattern_type {
            crate::core::pattern_analyzer::PatternType::File => self.file_patterns += 1,
            crate::core::pattern_analyzer::PatternType::Directory => self.directory_patterns += 1,
            crate::core::pattern_analyzer::PatternType::Both => self.both_patterns += 1,
        }
        
        if analysis.is_negation {
            self.negation_patterns += 1;
        }
        
        if analysis.is_absolute {
            self.absolute_patterns += 1;
        }
        
        if analysis.has_wildcards {
            self.wildcard_patterns += 1;
        }
        
        if analysis.has_globstar {
            self.globstar_patterns += 1;
        }
        
        if analysis.is_case_sensitive {
            self.case_sensitive_patterns += 1;
        } else {
            self.case_insensitive_patterns += 1;
        }
        
        self.pattern_analyses.push(analysis);
    }
    
    pub fn has_conflicts(&self) -> bool {
        !self.conflicts.is_empty()
    }
    
    pub fn conflict_count(&self) -> usize {
        self.conflicts.len()
    }
}

impl Default for GitignoreAnalysis {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::parser::parse_gitignore;

    #[test]
    fn test_basic_optimization() {
        let content = "*.log\n*.log\nbuild/";
        let file = parse_gitignore(content).unwrap();
        let optimized = optimize_gitignore(&file).unwrap();
        
        assert_eq!(optimized.entries.len(), 2);
        assert_eq!(optimized.stats.pattern_lines, 2);
    }

    #[test]
    fn test_preserve_comments() {
        let content = "*.log\n# Logs\n*.log\nbuild/";
        let file = parse_gitignore(content).unwrap();
        let optimized = optimize_gitignore(&file).unwrap();
        
        assert_eq!(optimized.entries.len(), 3);
        assert_eq!(optimized.stats.pattern_lines, 2);
        assert_eq!(optimized.stats.comment_lines, 1);
    }

    #[test]
    fn test_preserve_blank_lines() {
        let content = "*.log\n\n*.log\nbuild/";
        let file = parse_gitignore(content).unwrap();
        let optimized = optimize_gitignore(&file).unwrap();
        
        assert_eq!(optimized.entries.len(), 3);
        assert_eq!(optimized.stats.pattern_lines, 2);
        assert_eq!(optimized.stats.blank_lines, 1);
    }

    #[test]
    fn test_case_sensitive_patterns() {
        let content = "build/\nBUILD/";
        let file = parse_gitignore(content).unwrap();
        let optimized = optimize_gitignore(&file).unwrap();
        
        // Case-sensitive patterns should both be preserved
        assert_eq!(optimized.entries.len(), 2);
        assert_eq!(optimized.stats.pattern_lines, 2);
    }

    #[test]
    fn test_trailing_space_difference() {
        let content = "*.log \n*.log";
        let file = parse_gitignore(content).unwrap();
        let optimized = optimize_gitignore(&file).unwrap();
        
        // Patterns with different whitespace should both be preserved
        assert_eq!(optimized.entries.len(), 2);
        assert_eq!(optimized.stats.pattern_lines, 2);
    }

    #[test]
    fn test_negation_patterns() {
        let content = "*.log\n!debug.log\n*.log";
        let file = parse_gitignore(content).unwrap();
        let optimized = optimize_gitignore(&file).unwrap();
        
        // Negation patterns should be preserved
        assert_eq!(optimized.entries.len(), 2);
        assert_eq!(optimized.stats.pattern_lines, 2);
    }

    #[test]
    fn test_escaped_patterns() {
        let content = "\\#notacomment\n\\!notnegation";
        let file = parse_gitignore(content).unwrap();
        let optimized = optimize_gitignore(&file).unwrap();
        
        // Escaped patterns should be preserved
        assert_eq!(optimized.entries.len(), 2);
        assert_eq!(optimized.stats.pattern_lines, 2);
    }

    #[test]
    fn test_optimization_with_conflicts() {
        let content = "*.log\n!*.log\nbuild/";
        let file = parse_gitignore(content).unwrap();
        let (optimized, conflicts) = optimize_gitignore_with_conflicts(&file).unwrap();
        
        assert_eq!(optimized.entries.len(), 2);
        assert_eq!(conflicts.len(), 1);
        assert!((conflicts[0].0 == "*.log" && conflicts[0].1 == "!*.log") ||
                (conflicts[0].0 == "!*.log" && conflicts[0].1 == "*.log"));
    }

    #[test]
    fn test_analyze_gitignore() {
        let content = "*.log\nbuild/\n!debug.log\n# comment";
        let file = parse_gitignore(content).unwrap();
        let analysis = analyze_gitignore(&file).unwrap();
        
        assert_eq!(analysis.total_patterns, 3);
        assert_eq!(analysis.negation_patterns, 1);
        assert_eq!(analysis.conflict_count(), 1);
        assert!(analysis.has_conflicts());
    }

    // Test cases from TEST_MATRIX.md
    #[test]
    fn test_tc01_exact_deduplication_optimization() {
        let content = "*.log\n*.log";
        let file = parse_gitignore(content).unwrap();
        let optimized = optimize_gitignore(&file).unwrap();
        
        assert_eq!(optimized.entries.len(), 1);
        assert_eq!(optimized.stats.pattern_lines, 1);
        assert_eq!(optimized.entries[0].original, "*.log");
    }

    #[test]
    fn test_tc02_comment_preservation_optimization() {
        let content = "*.log\n*.log\n# comment";
        let file = parse_gitignore(content).unwrap();
        let optimized = optimize_gitignore(&file).unwrap();
        
        assert_eq!(optimized.entries.len(), 2);
        assert_eq!(optimized.stats.pattern_lines, 1);
        assert_eq!(optimized.stats.comment_lines, 1);
        assert_eq!(optimized.entries[0].original, "*.log");
        assert_eq!(optimized.entries[1].original, "# comment");
    }

    #[test]
    fn test_tc03_negation_support_optimization() {
        let content = "*.log\n!debug.log";
        let file = parse_gitignore(content).unwrap();
        let optimized = optimize_gitignore(&file).unwrap();
        
        assert_eq!(optimized.entries.len(), 2);
        assert_eq!(optimized.stats.pattern_lines, 2);
        assert_eq!(optimized.entries[0].original, "*.log");
        assert_eq!(optimized.entries[1].original, "!debug.log");
    }

    #[test]
    fn test_tc04_root_vs_relative_optimization() {
        let content = "/build\nbuild";
        let file = parse_gitignore(content).unwrap();
        let optimized = optimize_gitignore(&file).unwrap();
        
        assert_eq!(optimized.entries.len(), 2);
        assert_eq!(optimized.stats.pattern_lines, 2);
        assert_eq!(optimized.entries[0].original, "/build");
        assert_eq!(optimized.entries[1].original, "build");
    }

    #[test]
    fn test_tc09_trailing_space_optimization() {
        let content = "*.log \n*.log";
        let file = parse_gitignore(content).unwrap();
        let optimized = optimize_gitignore(&file).unwrap();
        
        // These should be treated as different patterns due to trailing space
        assert_eq!(optimized.entries.len(), 2);
        assert_eq!(optimized.stats.pattern_lines, 2);
        assert_eq!(optimized.entries[0].original, "*.log ");
        assert_eq!(optimized.entries[1].original, "*.log");
    }

    #[test]
    fn test_tc10_non_consecutive_deduplication() {
        let content = "*.swp\n*.log\n*.swp";
        let file = parse_gitignore(content).unwrap();
        let optimized = optimize_gitignore(&file).unwrap();
        
        assert_eq!(optimized.entries.len(), 2);
        assert_eq!(optimized.stats.pattern_lines, 2);
        assert_eq!(optimized.entries[0].original, "*.swp");
        assert_eq!(optimized.entries[1].original, "*.log");
    }

    #[test]
    fn test_tc11_wildcard_semantics_optimization() {
        let content = "node_modules/\n**/node_modules/";
        let file = parse_gitignore(content).unwrap();
        let optimized = optimize_gitignore(&file).unwrap();
        
        assert_eq!(optimized.entries.len(), 2);
        assert_eq!(optimized.stats.pattern_lines, 2);
        assert_eq!(optimized.entries[0].original, "node_modules/");
        assert_eq!(optimized.entries[1].original, "**/node_modules/");
    }

    #[test]
    fn test_tc12_file_vs_directory_optimization() {
        let content = "/tmp\n/tmp/";
        let file = parse_gitignore(content).unwrap();
        let optimized = optimize_gitignore(&file).unwrap();
        
        assert_eq!(optimized.entries.len(), 2);
        assert_eq!(optimized.stats.pattern_lines, 2);
        assert_eq!(optimized.entries[0].original, "/tmp");
        assert_eq!(optimized.entries[1].original, "/tmp/");
    }

    #[test]
    fn test_tc14_directory_repetition_optimization() {
        let content = "build/\nbuild/";
        let file = parse_gitignore(content).unwrap();
        let optimized = optimize_gitignore(&file).unwrap();
        
        assert_eq!(optimized.entries.len(), 1);
        assert_eq!(optimized.stats.pattern_lines, 1);
        assert_eq!(optimized.entries[0].original, "build/");
    }

    #[test]
    fn test_tc15_case_sensitivity_optimization() {
        let content = "build/\nBUILD/";
        let file = parse_gitignore(content).unwrap();
        let optimized = optimize_gitignore(&file).unwrap();
        
        assert_eq!(optimized.entries.len(), 2);
        assert_eq!(optimized.stats.pattern_lines, 2);
        assert_eq!(optimized.entries[0].original, "build/");
        assert_eq!(optimized.entries[1].original, "BUILD/");
    }

    #[test]
    fn test_tc16_layout_preservation_optimization() {
        let content = "# comment\n\n*.log\n";
        let file = parse_gitignore(content).unwrap();
        let optimized = optimize_gitignore(&file).unwrap();
        
        assert_eq!(optimized.entries.len(), 3);
        assert_eq!(optimized.stats.comment_lines, 1);
        assert_eq!(optimized.stats.blank_lines, 1);
        assert_eq!(optimized.stats.pattern_lines, 1);
    }

    #[test]
    fn test_tc17_unicode_entries_optimization() {
        let content = "Данные/\n*.лог";
        let file = parse_gitignore(content).unwrap();
        let optimized = optimize_gitignore(&file).unwrap();
        
        assert_eq!(optimized.entries.len(), 2);
        assert_eq!(optimized.stats.pattern_lines, 2);
        assert_eq!(optimized.entries[0].original, "Данные/");
        assert_eq!(optimized.entries[1].original, "*.лог");
    }

    #[test]
    fn test_tc19_emoji_support_optimization() {
        let content = "# 📝\n*.md\n*.md";
        let file = parse_gitignore(content).unwrap();
        let optimized = optimize_gitignore(&file).unwrap();
        
        assert_eq!(optimized.entries.len(), 2);
        assert_eq!(optimized.stats.comment_lines, 1);
        assert_eq!(optimized.stats.pattern_lines, 1);
        assert_eq!(optimized.entries[0].original, "# 📝");
        assert_eq!(optimized.entries[1].original, "*.md");
    }

    #[test]
    fn test_tc20_pattern_conflicts_optimization() {
        let content = "foo\nfoo\n!foo";
        let file = parse_gitignore(content).unwrap();
        let optimized = optimize_gitignore(&file).unwrap();
        
        assert_eq!(optimized.entries.len(), 2);
        assert_eq!(optimized.stats.pattern_lines, 2);
        assert_eq!(optimized.entries[0].original, "foo");
        assert_eq!(optimized.entries[1].original, "!foo");
    }
} 