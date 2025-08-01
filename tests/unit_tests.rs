use gix::{
    core::{parse_gitignore, optimize_gitignore},
    models::gitignore::{GitignoreEntry, FileStats, EntryType},
};

// Unit tests following TDD principles - testing individual components
mod parser_tests {
    use super::*;

    #[test]
    fn should_parse_empty_content() {
        // Arrange: Empty content
        let content = "";
        
        // Act: Parse the content
        let result = parse_gitignore(content);
        
        // Assert: Should succeed with empty file
        assert!(result.is_ok());
        let file = result.unwrap();
        assert_eq!(file.entries.len(), 0);
        assert_eq!(file.stats.total_lines, 0);
    }

    #[test]
    fn should_parse_single_pattern() {
        // Arrange: Single pattern
        let content = "*.log";
        
        // Act: Parse the content
        let result = parse_gitignore(content);
        
        // Assert: Should parse correctly
        assert!(result.is_ok());
        let file = result.unwrap();
        assert_eq!(file.entries.len(), 1);
        assert_eq!(file.entries[0].original, "*.log");
        assert_eq!(file.stats.pattern_lines, 1);
    }

    #[test]
    fn should_parse_comment_line() {
        // Arrange: Comment line
        let content = "# This is a comment";
        
        // Act: Parse the content
        let result = parse_gitignore(content);
        
        // Assert: Should parse comment correctly
        assert!(result.is_ok());
        let file = result.unwrap();
        assert_eq!(file.entries.len(), 1);
        assert_eq!(file.entries[0].original, "# This is a comment");
        assert_eq!(file.stats.comment_lines, 1);
    }

    #[test]
    fn should_parse_blank_line() {
        // Arrange: Blank line
        let content = "\n";
        
        // Act: Parse the content
        let result = parse_gitignore(content);
        
        // Assert: Should parse blank line correctly
        assert!(result.is_ok());
        let file = result.unwrap();
        assert_eq!(file.entries.len(), 1);
        assert_eq!(file.stats.blank_lines, 1);
    }

    #[test]
    fn should_parse_multiple_lines() {
        // Arrange: Multiple lines with different types
        let content = "*.log\n# comment\n\nbuild/";
        
        // Act: Parse the content
        let result = parse_gitignore(content);
        
        // Assert: Should parse all lines correctly
        assert!(result.is_ok());
        let file = result.unwrap();
        assert_eq!(file.entries.len(), 4);
        assert_eq!(file.stats.pattern_lines, 2);
        assert_eq!(file.stats.comment_lines, 1);
        assert_eq!(file.stats.blank_lines, 1);
    }
}

mod optimizer_tests {
    use super::*;

    #[test]
    fn should_optimize_empty_file() {
        // Arrange: Empty gitignore file
        let file = parse_gitignore("").unwrap();
        
        // Act: Optimize the file
        let result = optimize_gitignore(&file);
        
        // Assert: Should succeed with empty optimized file
        assert!(result.is_ok());
        let optimized = result.unwrap();
        assert_eq!(optimized.entries.len(), 0);
    }

    #[test]
    fn should_remove_consecutive_duplicates() {
        // Arrange: File with consecutive duplicates
        let file = parse_gitignore("*.log\n*.log").unwrap();
        
        // Act: Optimize the file
        let result = optimize_gitignore(&file);
        
        // Assert: Should remove duplicates
        assert!(result.is_ok());
        let optimized = result.unwrap();
        assert_eq!(optimized.entries.len(), 1);
        assert_eq!(optimized.entries[0].original, "*.log");
    }

    #[test]
    fn should_preserve_different_patterns() {
        // Arrange: File with different patterns
        let file = parse_gitignore("*.log\n*.tmp").unwrap();
        
        // Act: Optimize the file
        let result = optimize_gitignore(&file);
        
        // Assert: Should preserve both patterns
        assert!(result.is_ok());
        let optimized = result.unwrap();
        assert_eq!(optimized.entries.len(), 2);
        assert!(optimized.entries.iter().any(|e| e.original == "*.log"));
        assert!(optimized.entries.iter().any(|e| e.original == "*.tmp"));
    }

    #[test]
    fn should_preserve_comments() {
        // Arrange: File with patterns and comments
        let file = parse_gitignore("*.log\n# comment\n*.tmp").unwrap();
        
        // Act: Optimize the file
        let result = optimize_gitignore(&file);
        
        // Assert: Should preserve comments
        assert!(result.is_ok());
        let optimized = result.unwrap();
        assert_eq!(optimized.entries.len(), 3);
        assert!(optimized.entries.iter().any(|e| e.original == "# comment"));
    }

    #[test]
    fn should_handle_case_sensitive_patterns() {
        // Arrange: File with case-different patterns
        let file = parse_gitignore("build/\nBUILD/").unwrap();
        
        // Act: Optimize the file
        let result = optimize_gitignore(&file);
        
        // Assert: Should treat as different patterns
        assert!(result.is_ok());
        let optimized = result.unwrap();
        assert_eq!(optimized.entries.len(), 2);
        assert!(optimized.entries.iter().any(|e| e.original == "build/"));
        assert!(optimized.entries.iter().any(|e| e.original == "BUILD/"));
    }
}

mod entry_tests {
    use super::*;

    #[test]
    fn should_create_pattern_entry() {
        // Arrange: Pattern string
        let pattern = "*.log";
        
        // Act: Create entry
        let entry = GitignoreEntry::new(
            pattern.to_string(),
            EntryType::Pattern(pattern.to_string()),
            1,
        );
        
        // Assert: Should create entry correctly
        assert_eq!(entry.original, pattern);
        assert_eq!(entry.line_number, 1);
        assert!(entry.is_pattern());
    }

    #[test]
    fn should_create_comment_entry() {
        // Arrange: Comment string
        let comment = "# This is a comment";
        
        // Act: Create entry
        let entry = GitignoreEntry::new(
            comment.to_string(),
            EntryType::Comment(comment.to_string()),
            2,
        );
        
        // Assert: Should create entry correctly
        assert_eq!(entry.original, comment);
        assert_eq!(entry.line_number, 2);
        assert!(entry.is_comment());
    }

    #[test]
    fn should_create_blank_entry() {
        // Arrange: Blank line
        let blank = "";
        
        // Act: Create entry
        let entry = GitignoreEntry::new(
            blank.to_string(),
            EntryType::Blank,
            3,
        );
        
        // Assert: Should create entry correctly
        assert_eq!(entry.original, blank);
        assert_eq!(entry.line_number, 3);
        assert!(entry.is_blank());
    }

    #[test]
    fn should_normalize_pattern_with_inline_comment() {
        // Arrange: Pattern with inline comment
        let pattern = "*.log # inline comment";
        
        // Act: Create entry and get normalized pattern
        let entry = GitignoreEntry::new(
            pattern.to_string(),
            EntryType::Pattern("*.log ".to_string()),
            1,
        );
        let normalized = entry.normalized_pattern();
        
        // Assert: Should normalize correctly
        assert_eq!(normalized, Some("*.log ".to_string()));
    }

    #[test]
    fn should_handle_pattern_without_inline_comment() {
        // Arrange: Pattern without inline comment
        let pattern = "*.log";
        
        // Act: Create entry and get normalized pattern
        let entry = GitignoreEntry::new(
            pattern.to_string(),
            EntryType::Pattern(pattern.to_string()),
            1,
        );
        let normalized = entry.normalized_pattern();
        
        // Assert: Should return original pattern
        assert_eq!(normalized, Some(pattern.to_string()));
    }

    #[test]
    fn should_return_none_for_non_pattern_entries() {
        // Arrange: Comment entry
        let comment = "# comment";
        
        // Act: Create entry and get normalized pattern
        let entry = GitignoreEntry::new(
            comment.to_string(),
            EntryType::Comment(comment.to_string()),
            1,
        );
        let normalized = entry.normalized_pattern();
        
        // Assert: Should return None for non-pattern entries
        assert_eq!(normalized, None);
    }
}

mod stats_tests {
    use super::*;

    #[test]
    fn should_initialize_empty_stats() {
        // Act: Create empty stats
        let stats = FileStats::new();
        
        // Assert: Should have zero counts
        assert_eq!(stats.total_lines, 0);
        assert_eq!(stats.pattern_lines, 0);
        assert_eq!(stats.comment_lines, 0);
        assert_eq!(stats.blank_lines, 0);
        assert_eq!(stats.duplicate_patterns, 0);
    }

    #[test]
    fn should_update_stats_with_pattern_entry() {
        // Arrange: Empty stats and pattern entry
        let mut stats = FileStats::new();
        let entry = GitignoreEntry::new(
            "*.log".to_string(),
            EntryType::Pattern("*.log".to_string()),
            1,
        );
        
        // Act: Update stats
        stats.update(&entry);
        
        // Assert: Should increment correctly
        assert_eq!(stats.total_lines, 1);
        assert_eq!(stats.pattern_lines, 1);
        assert_eq!(stats.comment_lines, 0);
        assert_eq!(stats.blank_lines, 0);
    }

    #[test]
    fn should_update_stats_with_comment_entry() {
        // Arrange: Empty stats and comment entry
        let mut stats = FileStats::new();
        let entry = GitignoreEntry::new(
            "# comment".to_string(),
            EntryType::Comment("# comment".to_string()),
            1,
        );
        
        // Act: Update stats
        stats.update(&entry);
        
        // Assert: Should increment correctly
        assert_eq!(stats.total_lines, 1);
        assert_eq!(stats.pattern_lines, 0);
        assert_eq!(stats.comment_lines, 1);
        assert_eq!(stats.blank_lines, 0);
    }

    #[test]
    fn should_update_stats_with_blank_entry() {
        // Arrange: Empty stats and blank entry
        let mut stats = FileStats::new();
        let entry = GitignoreEntry::new(
            "".to_string(),
            EntryType::Blank,
            1,
        );
        
        // Act: Update stats
        stats.update(&entry);
        
        // Assert: Should increment correctly
        assert_eq!(stats.total_lines, 1);
        assert_eq!(stats.pattern_lines, 0);
        assert_eq!(stats.comment_lines, 0);
        assert_eq!(stats.blank_lines, 1);
    }
}

// Integration tests for the complete workflow
mod workflow_tests {
    use super::*;

    #[test]
    fn should_parse_and_optimize_complete_workflow() {
        // Arrange: Complex gitignore content
        let content = r#"# Logs
*.log
*.log
logs/

# Build outputs
build/
BUILD/
"#;
        
        // Act: Parse and optimize
        let parse_result = parse_gitignore(content);
        assert!(parse_result.is_ok());
        
        let file = parse_result.unwrap();
        let optimize_result = optimize_gitignore(&file);
        assert!(optimize_result.is_ok());
        
        let optimized = optimize_result.unwrap();
        
        // Assert: Should optimize correctly
        assert!(optimized.entries.len() < file.entries.len());
        assert_eq!(optimized.stats.pattern_lines, 4); // Reduced from 5
        assert_eq!(optimized.stats.comment_lines, 2);
    }

    #[test]
    fn should_handle_error_cases_gracefully() {
        // Arrange: Invalid content (if parser supports validation)
        let content = "*.log\ninvalid pattern with spaces\n*.tmp";
        
        // Act: Parse and optimize
        let parse_result = parse_gitignore(content);
        
        // Assert: Should handle gracefully (assuming parser accepts this)
        // This test demonstrates how to handle potential error cases
        if parse_result.is_ok() {
            let file = parse_result.unwrap();
            let optimize_result = optimize_gitignore(&file);
            assert!(optimize_result.is_ok());
        }
    }
} 