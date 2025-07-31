use gix::{
    core::{parse_gitignore, optimize_gitignore},
};

#[test]
fn test_tc01_exact_deduplication_integration() {
    let content = "*.log\n*.log";
    let file = parse_gitignore(content).unwrap();
    let optimized = optimize_gitignore(&file).unwrap();
    
    assert_eq!(optimized.entries.len(), 1);
    assert_eq!(optimized.stats.pattern_lines, 1);
    assert_eq!(optimized.entries[0].original, "*.log");
}

#[test]
fn test_tc02_comment_preservation_integration() {
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
fn test_tc03_negation_support_integration() {
    let content = "*.log\n!debug.log";
    let file = parse_gitignore(content).unwrap();
    let optimized = optimize_gitignore(&file).unwrap();
    
    assert_eq!(optimized.entries.len(), 2);
    assert_eq!(optimized.stats.pattern_lines, 2);
    assert_eq!(optimized.entries[0].original, "*.log");
    assert_eq!(optimized.entries[1].original, "!debug.log");
}

#[test]
fn test_tc04_root_vs_relative_integration() {
    let content = "/build\nbuild";
    let file = parse_gitignore(content).unwrap();
    let optimized = optimize_gitignore(&file).unwrap();
    
    assert_eq!(optimized.entries.len(), 2);
    assert_eq!(optimized.stats.pattern_lines, 2);
    assert_eq!(optimized.entries[0].original, "/build");
    assert_eq!(optimized.entries[1].original, "build");
}

#[test]
fn test_tc05_escaped_hash_integration() {
    let content = "\\#notacomment";
    let file = parse_gitignore(content).unwrap();
    let optimized = optimize_gitignore(&file).unwrap();
    
    assert_eq!(optimized.entries.len(), 1);
    assert_eq!(optimized.stats.pattern_lines, 1);
    assert_eq!(optimized.entries[0].original, "\\#notacomment");
}

#[test]
fn test_tc06_escaped_negation_integration() {
    let content = "\\!notnegation";
    let file = parse_gitignore(content).unwrap();
    let optimized = optimize_gitignore(&file).unwrap();
    
    assert_eq!(optimized.entries.len(), 1);
    assert_eq!(optimized.stats.pattern_lines, 1);
    assert_eq!(optimized.entries[0].original, "\\!notnegation");
}

#[test]
fn test_tc07_inline_comment_integration() {
    let content = "*.log # inline";
    let file = parse_gitignore(content).unwrap();
    let optimized = optimize_gitignore(&file).unwrap();
    
    assert_eq!(optimized.entries.len(), 1);
    assert_eq!(optimized.stats.pattern_lines, 1);
    assert_eq!(optimized.entries[0].normalized_pattern(), Some("*.log ".to_string()));
}

#[test]
fn test_tc08_directory_override_integration() {
    let content = "debug/\n!debug/";
    let file = parse_gitignore(content).unwrap();
    let optimized = optimize_gitignore(&file).unwrap();
    
    assert_eq!(optimized.entries.len(), 2);
    assert_eq!(optimized.stats.pattern_lines, 2);
    assert_eq!(optimized.entries[0].original, "debug/");
    assert_eq!(optimized.entries[1].original, "!debug/");
}

#[test]
fn test_tc09_trailing_space_integration() {
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
fn test_tc10_non_consecutive_deduplication_integration() {
    let content = "*.swp\n*.log\n*.swp";
    let file = parse_gitignore(content).unwrap();
    let optimized = optimize_gitignore(&file).unwrap();
    
    assert_eq!(optimized.entries.len(), 2);
    assert_eq!(optimized.stats.pattern_lines, 2);
    assert_eq!(optimized.entries[0].original, "*.swp");
    assert_eq!(optimized.entries[1].original, "*.log");
}

#[test]
fn test_tc11_wildcard_semantics_integration() {
    let content = "node_modules/\n**/node_modules/";
    let file = parse_gitignore(content).unwrap();
    let optimized = optimize_gitignore(&file).unwrap();
    
    assert_eq!(optimized.entries.len(), 2);
    assert_eq!(optimized.stats.pattern_lines, 2);
    assert_eq!(optimized.entries[0].original, "node_modules/");
    assert_eq!(optimized.entries[1].original, "**/node_modules/");
}

#[test]
fn test_tc12_file_vs_directory_integration() {
    let content = "/tmp\n/tmp/";
    let file = parse_gitignore(content).unwrap();
    let optimized = optimize_gitignore(&file).unwrap();
    
    assert_eq!(optimized.entries.len(), 2);
    assert_eq!(optimized.stats.pattern_lines, 2);
    assert_eq!(optimized.entries[0].original, "/tmp");
    assert_eq!(optimized.entries[1].original, "/tmp/");
}

#[test]
fn test_tc13_duplicate_with_comment_integration() {
    let content = "# Logs\n*.log\n# Logs\n*.log";
    let file = parse_gitignore(content).unwrap();
    let optimized = optimize_gitignore(&file).unwrap();
    
    // Should deduplicate patterns but preserve comments
    assert_eq!(optimized.entries.len(), 3);
    assert_eq!(optimized.stats.pattern_lines, 1);
    assert_eq!(optimized.stats.comment_lines, 2);
}

#[test]
fn test_tc14_directory_repetition_integration() {
    let content = "build/\nbuild/";
    let file = parse_gitignore(content).unwrap();
    let optimized = optimize_gitignore(&file).unwrap();
    
    assert_eq!(optimized.entries.len(), 1);
    assert_eq!(optimized.stats.pattern_lines, 1);
    assert_eq!(optimized.entries[0].original, "build/");
}

#[test]
fn test_tc15_case_sensitivity_integration() {
    let content = "build/\nBUILD/";
    let file = parse_gitignore(content).unwrap();
    let optimized = optimize_gitignore(&file).unwrap();
    
    assert_eq!(optimized.entries.len(), 2);
    assert_eq!(optimized.stats.pattern_lines, 2);
    assert_eq!(optimized.entries[0].original, "build/");
    assert_eq!(optimized.entries[1].original, "BUILD/");
}

#[test]
fn test_tc16_layout_preservation_integration() {
    let content = "# comment\n\n*.log\n";
    let file = parse_gitignore(content).unwrap();
    let optimized = optimize_gitignore(&file).unwrap();
    
    assert_eq!(optimized.entries.len(), 3);
    assert_eq!(optimized.stats.comment_lines, 1);
    assert_eq!(optimized.stats.blank_lines, 1);
    assert_eq!(optimized.stats.pattern_lines, 1);
}

#[test]
fn test_tc17_unicode_entries_integration() {
    let content = "Данные/\n*.лог";
    let file = parse_gitignore(content).unwrap();
    let optimized = optimize_gitignore(&file).unwrap();
    
    assert_eq!(optimized.entries.len(), 2);
    assert_eq!(optimized.stats.pattern_lines, 2);
    assert_eq!(optimized.entries[0].original, "Данные/");
    assert_eq!(optimized.entries[1].original, "*.лог");
}

#[test]
fn test_tc18_wildcard_range_integration() {
    let content = "**/*.log\n*.log";
    let file = parse_gitignore(content).unwrap();
    let optimized = optimize_gitignore(&file).unwrap();
    
    assert_eq!(optimized.entries.len(), 2);
    assert_eq!(optimized.stats.pattern_lines, 2);
    assert_eq!(optimized.entries[0].original, "**/*.log");
    assert_eq!(optimized.entries[1].original, "*.log");
}

#[test]
fn test_tc19_emoji_support_integration() {
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
fn test_tc20_pattern_conflicts_integration() {
    let content = "foo\nfoo\n!foo";
    let file = parse_gitignore(content).unwrap();
    let optimized = optimize_gitignore(&file).unwrap();
    
    assert_eq!(optimized.entries.len(), 2);
    assert_eq!(optimized.stats.pattern_lines, 2);
    assert_eq!(optimized.entries[0].original, "foo");
    assert_eq!(optimized.entries[1].original, "!foo");
}

#[test]
fn test_complex_real_world_scenario() {
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

    let file = parse_gitignore(content).unwrap();
    let optimized = optimize_gitignore(&file).unwrap();
    
    // Should remove duplicate *.log and build/ patterns
    assert!(optimized.entries.len() < file.entries.len());
    assert_eq!(optimized.stats.pattern_lines, 14); // Reduced from 17
    assert_eq!(optimized.stats.comment_lines, 10);
    assert_eq!(optimized.stats.blank_lines, 7);
}

#[test]
fn test_empty_file() {
    let content = "";
    let file = parse_gitignore(content).unwrap();
    let optimized = optimize_gitignore(&file).unwrap();
    
    assert_eq!(optimized.entries.len(), 0);
    assert_eq!(optimized.stats.total_lines, 0);
}

#[test]
fn test_file_with_only_comments() {
    let content = "# This is a comment\n# Another comment";
    let file = parse_gitignore(content).unwrap();
    let optimized = optimize_gitignore(&file).unwrap();
    
    assert_eq!(optimized.entries.len(), 2);
    assert_eq!(optimized.stats.comment_lines, 2);
    assert_eq!(optimized.stats.pattern_lines, 0);
}

#[test]
fn test_file_with_only_blank_lines() {
    let content = "\n\n\n";
    let file = parse_gitignore(content).unwrap();
    let optimized = optimize_gitignore(&file).unwrap();
    
    assert_eq!(optimized.entries.len(), 3);
    assert_eq!(optimized.stats.blank_lines, 3);
    assert_eq!(optimized.stats.pattern_lines, 0);
} 