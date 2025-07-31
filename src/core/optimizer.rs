use crate::models::{GitignoreFile, GixError};
use std::collections::HashSet;

/// Optimize a gitignore file by removing duplicate patterns while preserving structure
pub fn optimize_gitignore(file: &GitignoreFile) -> Result<GitignoreFile, GixError> {
    let mut optimized = GitignoreFile::new();
    let mut seen_patterns: HashSet<String> = HashSet::new();
    
    for entry in &file.entries {
        match &entry.entry_type {
            crate::models::EntryType::Pattern(pattern) => {
                // Don't normalize patterns - preserve exact whitespace
                if !seen_patterns.contains(pattern) {
                    seen_patterns.insert(pattern.clone());
                    optimized.add_entry(entry.clone());
                }
                // Skip duplicates
            }
            crate::models::EntryType::Comment(_) | crate::models::EntryType::Blank => {
                // Always preserve comments and blank lines
                optimized.add_entry(entry.clone());
            }
        }
    }
    
    Ok(optimized)
}

/// Optimize a gitignore file with more aggressive deduplication
pub fn optimize_gitignore_aggressive(file: &GitignoreFile) -> Result<GitignoreFile, GixError> {
    let mut optimized = GitignoreFile::new();
    let mut seen_patterns: HashSet<String> = HashSet::new();
    let mut seen_comments: HashSet<String> = HashSet::new();
    
    for entry in &file.entries {
        match &entry.entry_type {
            crate::models::EntryType::Pattern(pattern) => {
                // Don't normalize patterns - preserve exact whitespace
                if !seen_patterns.contains(pattern) {
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