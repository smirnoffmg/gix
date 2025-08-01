use gix::core::{optimize_gitignore, parse_gitignore};

// Shared test utilities following TDD best practices
mod test_helpers {
    use super::*;

    /// Helper function to create and optimize a gitignore file
    /// This follows TDD principle of reducing duplication and improving test readability
    pub fn create_optimized_gitignore(content: &str) -> gix::models::gitignore::GitignoreFile {
        let file = parse_gitignore(content).expect("Failed to parse gitignore content");
        optimize_gitignore(&file).expect("Failed to optimize gitignore file")
    }

    /// Helper function to assert entry count and pattern count
    /// Makes tests more readable and reduces assertion duplication
    pub fn assert_entry_counts(
        optimized: &gix::models::gitignore::GitignoreFile,
        expected_entries: usize,
        expected_patterns: usize,
    ) {
        assert_eq!(
            optimized.entries.len(),
            expected_entries,
            "Expected {} entries, got {}",
            expected_entries,
            optimized.entries.len()
        );
        assert_eq!(
            optimized.stats.pattern_lines, expected_patterns,
            "Expected {} patterns, got {}",
            expected_patterns, optimized.stats.pattern_lines
        );
    }

    /// Helper function to assert specific entry exists
    /// Makes tests more expressive about what they're checking
    pub fn assert_entry_exists(
        optimized: &gix::models::gitignore::GitignoreFile,
        expected_pattern: &str,
    ) {
        let found = optimized
            .entries
            .iter()
            .any(|entry| entry.original == expected_pattern);
        assert!(
            found,
            "Expected pattern '{}' not found in optimized entries",
            expected_pattern
        );
    }
}

use test_helpers::*;

// Test modules organized by functionality - following TDD principle of clear test organization
mod deduplication_tests {
    use super::*;

    #[test]
    fn should_remove_exact_duplicate_patterns() {
        // Arrange: Create content with duplicate patterns
        let content = "*.log\n*.log";

        // Act: Parse and optimize
        let optimized = create_optimized_gitignore(content);

        // Assert: Only one entry should remain
        assert_entry_counts(&optimized, 1, 1);
        assert_entry_exists(&optimized, "*.log");
    }

    #[test]
    fn should_remove_non_consecutive_duplicates() {
        // Arrange: Create content with non-consecutive duplicates
        let content = "*.swp\n*.log\n*.swp";

        // Act: Parse and optimize
        let optimized = create_optimized_gitignore(content);

        // Assert: Should keep both unique patterns
        assert_entry_counts(&optimized, 2, 2);
        assert_entry_exists(&optimized, "*.swp");
        assert_entry_exists(&optimized, "*.log");
    }

    #[test]
    fn should_remove_directory_duplicates() {
        // Arrange: Create content with duplicate directory patterns
        let content = "build/\nbuild/";

        // Act: Parse and optimize
        let optimized = create_optimized_gitignore(content);

        // Assert: Only one directory entry should remain
        assert_entry_counts(&optimized, 1, 1);
        assert_entry_exists(&optimized, "build/");
    }
}

mod comment_tests {
    use super::*;

    #[test]
    fn should_preserve_comments_when_deduplicating_patterns() {
        // Arrange: Create content with comments and duplicate patterns
        let content = "*.log\n*.log\n# comment";

        // Act: Parse and optimize
        let optimized = create_optimized_gitignore(content);

        // Assert: Should keep comment and deduplicate pattern
        assert_entry_counts(&optimized, 2, 1);
        assert_eq!(optimized.stats.comment_lines, 1);
        assert_entry_exists(&optimized, "*.log");
        assert_entry_exists(&optimized, "# comment");
    }

    #[test]
    fn should_handle_escaped_hash_as_pattern_not_comment() {
        // Arrange: Create content with escaped hash
        let content = "\\#notacomment";

        // Act: Parse and optimize
        let optimized = create_optimized_gitignore(content);

        // Assert: Should treat as pattern, not comment
        assert_entry_counts(&optimized, 1, 1);
        assert_entry_exists(&optimized, "\\#notacomment");
    }

    #[test]
    fn should_handle_inline_comments() {
        // Arrange: Create content with inline comment
        let content = "*.log # inline";

        // Act: Parse and optimize
        let optimized = create_optimized_gitignore(content);

        // Assert: Should preserve pattern with inline comment
        assert_entry_counts(&optimized, 1, 1);
        assert_eq!(
            optimized.entries[0].normalized_pattern(),
            Some("*.log ".to_string())
        );
    }
}

mod negation_tests {
    use super::*;

    #[test]
    fn should_preserve_negation_patterns() {
        // Arrange: Create content with negation
        let content = "*.log\n!debug.log";

        // Act: Parse and optimize
        let optimized = create_optimized_gitignore(content);

        // Assert: Should keep both patterns
        assert_entry_counts(&optimized, 2, 2);
        assert_entry_exists(&optimized, "*.log");
        assert_entry_exists(&optimized, "!debug.log");
    }

    #[test]
    fn should_handle_escaped_negation_as_pattern() {
        // Arrange: Create content with escaped negation
        let content = "\\!notnegation";

        // Act: Parse and optimize
        let optimized = create_optimized_gitignore(content);

        // Assert: Should treat as pattern, not negation
        assert_entry_counts(&optimized, 1, 1);
        assert_entry_exists(&optimized, "\\!notnegation");
    }

    #[test]
    fn should_handle_directory_override_patterns() {
        // Arrange: Create content with directory override
        let content = "debug/\n!debug/";

        // Act: Parse and optimize
        let optimized = create_optimized_gitignore(content);

        // Assert: Should keep both patterns
        assert_entry_counts(&optimized, 2, 2);
        assert_entry_exists(&optimized, "debug/");
        assert_entry_exists(&optimized, "!debug/");
    }
}

mod path_semantics_tests {
    use super::*;

    #[test]
    fn should_distinguish_root_vs_relative_paths() {
        // Arrange: Create content with root and relative paths
        let content = "/build\nbuild";

        // Act: Parse and optimize
        let optimized = create_optimized_gitignore(content);

        // Assert: Should treat as different patterns
        assert_entry_counts(&optimized, 2, 2);
        assert_entry_exists(&optimized, "/build");
        assert_entry_exists(&optimized, "build");
    }

    #[test]
    fn should_distinguish_file_vs_directory_patterns() {
        // Arrange: Create content with file and directory patterns
        let content = "/tmp\n/tmp/";

        // Act: Parse and optimize
        let optimized = create_optimized_gitignore(content);

        // Assert: Should treat as different patterns
        assert_entry_counts(&optimized, 2, 2);
        assert_entry_exists(&optimized, "/tmp");
        assert_entry_exists(&optimized, "/tmp/");
    }

    #[test]
    fn should_distinguish_wildcard_patterns() {
        // Arrange: Create content with different wildcard patterns
        let content = "node_modules/\n**/node_modules/";

        // Act: Parse and optimize
        let optimized = create_optimized_gitignore(content);

        // Assert: Should treat as different patterns
        assert_entry_counts(&optimized, 2, 2);
        assert_entry_exists(&optimized, "node_modules/");
        assert_entry_exists(&optimized, "**/node_modules/");
    }

    #[test]
    fn should_distinguish_wildcard_range_patterns() {
        // Arrange: Create content with different wildcard ranges
        let content = "**/*.log\n*.log";

        // Act: Parse and optimize
        let optimized = create_optimized_gitignore(content);

        // Assert: Should treat as different patterns
        assert_entry_counts(&optimized, 2, 2);
        assert_entry_exists(&optimized, "**/*.log");
        assert_entry_exists(&optimized, "*.log");
    }
}

mod whitespace_tests {
    use super::*;

    #[test]
    fn should_treat_trailing_space_as_different_pattern() {
        // Arrange: Create content with trailing space
        let content = "*.log \n*.log";

        // Act: Parse and optimize
        let optimized = create_optimized_gitignore(content);

        // Assert: Should treat as different patterns due to trailing space
        assert_entry_counts(&optimized, 2, 2);
        assert_entry_exists(&optimized, "*.log ");
        assert_entry_exists(&optimized, "*.log");
    }

    #[test]
    fn should_preserve_layout_with_blank_lines() {
        // Arrange: Create content with comments and blank lines
        let content = "# comment\n\n*.log\n";

        // Act: Parse and optimize
        let optimized = create_optimized_gitignore(content);

        // Assert: Should preserve layout
        assert_eq!(optimized.entries.len(), 3);
        assert_eq!(optimized.stats.comment_lines, 1);
        assert_eq!(optimized.stats.blank_lines, 1);
        assert_eq!(optimized.stats.pattern_lines, 1);
    }
}

mod case_sensitivity_tests {
    use super::*;

    #[test]
    fn should_treat_case_different_patterns_as_distinct() {
        // Arrange: Create content with case-different patterns
        let content = "build/\nBUILD/";

        // Act: Parse and optimize
        let optimized = create_optimized_gitignore(content);

        // Assert: Should treat as different patterns
        assert_entry_counts(&optimized, 2, 2);
        assert_entry_exists(&optimized, "build/");
        assert_entry_exists(&optimized, "BUILD/");
    }
}

mod unicode_tests {
    use super::*;

    #[test]
    fn should_handle_unicode_patterns() {
        // Arrange: Create content with unicode patterns
        let content = "Данные/\n*.лог";

        // Act: Parse and optimize
        let optimized = create_optimized_gitignore(content);

        // Assert: Should handle unicode correctly
        assert_entry_counts(&optimized, 2, 2);
        assert_entry_exists(&optimized, "Данные/");
        assert_entry_exists(&optimized, "*.лог");
    }

    #[test]
    fn should_handle_emoji_in_comments() {
        // Arrange: Create content with emoji in comments
        let content = "# 📝\n*.md\n*.md";

        // Act: Parse and optimize
        let optimized = create_optimized_gitignore(content);

        // Assert: Should handle emoji correctly
        assert_entry_counts(&optimized, 2, 1);
        assert_eq!(optimized.stats.comment_lines, 1);
        assert_entry_exists(&optimized, "# 📝");
        assert_entry_exists(&optimized, "*.md");
    }
}

mod edge_case_tests {
    use super::*;

    #[test]
    fn should_handle_empty_file() {
        // Arrange: Create empty content
        let content = "";

        // Act: Parse and optimize
        let optimized = create_optimized_gitignore(content);

        // Assert: Should handle empty file correctly
        assert_eq!(optimized.entries.len(), 0);
        assert_eq!(optimized.stats.total_lines, 0);
    }

    #[test]
    fn should_handle_file_with_only_comments() {
        // Arrange: Create content with only comments
        let content = "# This is a comment\n# Another comment";

        // Act: Parse and optimize
        let optimized = create_optimized_gitignore(content);

        // Assert: Should preserve comments
        assert_eq!(optimized.entries.len(), 2);
        assert_eq!(optimized.stats.comment_lines, 2);
        assert_eq!(optimized.stats.pattern_lines, 0);
    }

    #[test]
    fn should_handle_file_with_only_blank_lines() {
        // Arrange: Create content with only blank lines
        let content = "\n\n\n";

        // Act: Parse and optimize
        let optimized = create_optimized_gitignore(content);

        // Assert: Should preserve blank lines
        assert_eq!(optimized.entries.len(), 3);
        assert_eq!(optimized.stats.blank_lines, 3);
        assert_eq!(optimized.stats.pattern_lines, 0);
    }

    #[test]
    fn should_handle_duplicate_comments_with_patterns() {
        // Arrange: Create content with duplicate comments and patterns
        let content = "# Logs\n*.log\n# Logs\n*.log";

        // Act: Parse and optimize
        let optimized = create_optimized_gitignore(content);

        // Assert: Should deduplicate patterns but preserve comments
        assert_eq!(optimized.entries.len(), 3);
        assert_eq!(optimized.stats.pattern_lines, 1);
        assert_eq!(optimized.stats.comment_lines, 2);
    }

    #[test]
    fn should_handle_pattern_conflicts() {
        // Arrange: Create content with conflicting patterns
        let content = "foo\nfoo\n!foo";

        // Act: Parse and optimize
        let optimized = create_optimized_gitignore(content);

        // Assert: Should deduplicate identical patterns but keep negation
        assert_entry_counts(&optimized, 2, 2);
        assert_entry_exists(&optimized, "foo");
        assert_entry_exists(&optimized, "!foo");
    }
}

mod real_world_scenario_tests {
    use super::*;

    #[test]
    fn should_optimize_complex_real_world_gitignore() {
        // Arrange: Create complex real-world gitignore content
        let content = r#"# Logs
*.log
*.log
logs/

# Build outputs
build/
BUILD/
dist/

# Dependencies
node_modules/
**/node_modules/

# IDE files
.vscode/
.idea/

# OS files
.DS_Store
Thumbs.db

# Temporary files
*.tmp
*.temp
*.swp

# Comments
# This is a comment
# Another comment

# More patterns
*.log
build/
"#;

        // Act: Parse and optimize
        let file = parse_gitignore(content).expect("Failed to parse complex gitignore");
        let optimized = optimize_gitignore(&file).expect("Failed to optimize complex gitignore");

        // Assert: Should remove duplicates and maintain structure
        assert!(
            optimized.entries.len() < file.entries.len(),
            "Optimization should reduce entry count"
        );
        assert_eq!(
            optimized.stats.pattern_lines, 14,
            "Should have 14 unique patterns"
        ); // Reduced from 17
        assert_eq!(
            optimized.stats.comment_lines, 10,
            "Should preserve 10 comment lines"
        );
        assert_eq!(
            optimized.stats.blank_lines, 7,
            "Should preserve 7 blank lines"
        );
    }
}
