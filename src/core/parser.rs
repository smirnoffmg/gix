use crate::models::{EntryType, GitignoreEntry, GitignoreFile, GixError};

/// Parse a .gitignore file content into a structured representation
pub fn parse_gitignore(content: &str) -> Result<GitignoreFile, GixError> {
    let mut file = GitignoreFile::new();

    for (line_number, line) in content.lines().enumerate() {
        let entry = parse_line(line, line_number + 1)?;
        file.add_entry(entry);
    }

    Ok(file)
}

/// Parse a single line from a .gitignore file
fn parse_line(line: &str, line_number: usize) -> Result<GitignoreEntry, GixError> {
    let original = line.to_string();

    // Handle blank lines
    if line.trim().is_empty() {
        return Ok(GitignoreEntry::new(original, EntryType::Blank, line_number));
    }

    // Handle comments (lines starting with #, but not escaped)
    if line.starts_with('#') && !line.starts_with("\\#") {
        return Ok(GitignoreEntry::new(
            original.clone(),
            EntryType::Comment(original.clone()),
            line_number,
        ));
    }

    // Handle patterns (everything else)
    // Remove inline comments (everything after # that's not escaped)
    let pattern = remove_inline_comment(line);

    Ok(GitignoreEntry::new(
        original,
        EntryType::Pattern(pattern),
        line_number,
    ))
}

/// Remove inline comments from a pattern line
fn remove_inline_comment(line: &str) -> String {
    let mut result = String::new();
    let chars = line.chars().peekable();
    let mut escaped = false;

    for ch in chars {
        if escaped {
            result.push(ch);
            escaped = false;
        } else if ch == '\\' {
            escaped = true;
            result.push(ch);
        } else if ch == '#' {
            // Found unescaped #, this is the start of an inline comment
            break;
        } else {
            result.push(ch);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_blank_line() {
        let entry = parse_line("", 1).unwrap();
        assert!(entry.is_blank());
        assert_eq!(entry.original, "");
    }

    #[test]
    fn test_parse_whitespace_line() {
        let entry = parse_line("   ", 1).unwrap();
        assert!(entry.is_blank());
        assert_eq!(entry.original, "   ");
    }

    #[test]
    fn test_parse_comment_line() {
        let entry = parse_line("# This is a comment", 1).unwrap();
        assert!(entry.is_comment());
        assert_eq!(entry.original, "# This is a comment");
    }

    #[test]
    fn test_parse_pattern_line() {
        let entry = parse_line("*.log", 1).unwrap();
        assert!(entry.is_pattern());
        assert_eq!(entry.original, "*.log");
        assert_eq!(entry.normalized_pattern(), Some("*.log".to_string()));
    }

    #[test]
    fn test_parse_negation_pattern() {
        let entry = parse_line("!debug.log", 1).unwrap();
        assert!(entry.is_pattern());
        assert_eq!(entry.original, "!debug.log");
        assert_eq!(entry.normalized_pattern(), Some("!debug.log".to_string()));
    }

    #[test]
    fn test_parse_escaped_hash() {
        let entry = parse_line("\\#notacomment", 1).unwrap();
        assert!(entry.is_pattern());
        assert_eq!(entry.original, "\\#notacomment");
        assert_eq!(
            entry.normalized_pattern(),
            Some("\\#notacomment".to_string())
        );
    }

    #[test]
    fn test_parse_escaped_negation() {
        let entry = parse_line("\\!notnegation", 1).unwrap();
        assert!(entry.is_pattern());
        assert_eq!(entry.original, "\\!notnegation");
        assert_eq!(
            entry.normalized_pattern(),
            Some("\\!notnegation".to_string())
        );
    }

    #[test]
    fn test_parse_inline_comment() {
        let entry = parse_line("*.log # inline comment", 1).unwrap();
        assert!(entry.is_pattern());
        assert_eq!(entry.original, "*.log # inline comment");
        assert_eq!(entry.normalized_pattern(), Some("*.log ".to_string()));
    }

    #[test]
    fn test_parse_escaped_inline_comment() {
        let entry = parse_line("*.log \\# not a comment", 1).unwrap();
        assert!(entry.is_pattern());
        assert_eq!(entry.original, "*.log \\# not a comment");
        assert_eq!(
            entry.normalized_pattern(),
            Some("*.log \\# not a comment".to_string())
        );
    }

    #[test]
    fn test_parse_unicode_pattern() {
        let entry = parse_line("Данные/", 1).unwrap();
        assert!(entry.is_pattern());
        assert_eq!(entry.original, "Данные/");
        assert_eq!(entry.normalized_pattern(), Some("Данные/".to_string()));
    }

    #[test]
    fn test_parse_emoji_comment() {
        let entry = parse_line("# 📝", 1).unwrap();
        assert!(entry.is_comment());
        assert_eq!(entry.original, "# 📝");
    }

    #[test]
    fn test_parse_complete_file() {
        let content = "*.log\n# Logs\n*.log\n\nbuild/";
        let file = parse_gitignore(content).unwrap();

        assert_eq!(file.entries.len(), 5);
        assert_eq!(file.stats.pattern_lines, 3);
        assert_eq!(file.stats.comment_lines, 1);
        assert_eq!(file.stats.blank_lines, 1);
    }

    // Test cases from TEST_MATRIX.md
    #[test]
    fn test_tc01_exact_deduplication_parsing() {
        let content = "*.log\n*.log";
        let file = parse_gitignore(content).unwrap();

        let duplicates = file.find_duplicates();
        assert_eq!(duplicates.len(), 1);
        assert_eq!(duplicates["*.log"], vec![1, 2]);
    }

    #[test]
    fn test_tc02_comment_preservation_parsing() {
        let content = "*.log\n*.log\n# comment";
        let file = parse_gitignore(content).unwrap();

        let comments = file.comments();
        assert_eq!(comments.len(), 1);
        assert_eq!(comments[0].original, "# comment");
    }

    #[test]
    fn test_tc03_negation_support_parsing() {
        let content = "*.log\n!debug.log";
        let file = parse_gitignore(content).unwrap();

        let patterns = file.patterns();
        assert_eq!(patterns.len(), 2);
        assert_eq!(patterns[0].original, "*.log");
        assert_eq!(patterns[1].original, "!debug.log");
    }

    #[test]
    fn test_tc05_escaped_hash_parsing() {
        let content = "\\#notacomment";
        let file = parse_gitignore(content).unwrap();

        let patterns = file.patterns();
        assert_eq!(patterns.len(), 1);
        assert_eq!(patterns[0].original, "\\#notacomment");
    }

    #[test]
    fn test_tc06_escaped_negation_parsing() {
        let content = "\\!notnegation";
        let file = parse_gitignore(content).unwrap();

        let patterns = file.patterns();
        assert_eq!(patterns.len(), 1);
        assert_eq!(patterns[0].original, "\\!notnegation");
    }

    #[test]
    fn test_tc07_inline_comment_parsing() {
        let content = "*.log # inline";
        let file = parse_gitignore(content).unwrap();

        let patterns = file.patterns();
        assert_eq!(patterns.len(), 1);
        assert_eq!(patterns[0].normalized_pattern(), Some("*.log ".to_string()));
    }

    #[test]
    fn test_tc16_layout_preservation_parsing() {
        let content = "# comment\n\n*.log\n";
        let file = parse_gitignore(content).unwrap();

        assert_eq!(file.stats.comment_lines, 1);
        assert_eq!(file.stats.blank_lines, 1);
        assert_eq!(file.stats.pattern_lines, 1);
    }

    #[test]
    fn test_tc17_unicode_entries_parsing() {
        let content = "Данные/\n*.лог";
        let file = parse_gitignore(content).unwrap();

        let patterns = file.patterns();
        assert_eq!(patterns.len(), 2);
        assert_eq!(patterns[0].original, "Данные/");
        assert_eq!(patterns[1].original, "*.лог");
    }

    #[test]
    fn test_tc19_emoji_support_parsing() {
        let content = "# 📝\n*.md\n*.md";
        let file = parse_gitignore(content).unwrap();

        let comments = file.comments();
        let patterns = file.patterns();
        assert_eq!(comments.len(), 1);
        assert_eq!(comments[0].original, "# 📝");
        assert_eq!(patterns.len(), 2);
    }
}
