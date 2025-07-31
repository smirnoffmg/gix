use std::collections::HashMap;

/// Represents the type of a gitignore entry
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EntryType {
    /// A pattern line (e.g., "*.log", "build/")
    Pattern(String),
    /// A comment line (e.g., "# Logs")
    Comment(String),
    /// A blank line
    Blank,
}

/// Represents a single line in a .gitignore file
#[derive(Debug, Clone)]
pub struct GitignoreEntry {
    /// The original line content
    pub original: String,
    /// The parsed entry type
    pub entry_type: EntryType,
    /// Line number (1-indexed)
    pub line_number: usize,
}

impl GitignoreEntry {
    /// Create a new gitignore entry
    pub fn new(original: String, entry_type: EntryType, line_number: usize) -> Self {
        Self {
            original,
            entry_type,
            line_number,
        }
    }

    /// Check if this entry is a pattern
    pub fn is_pattern(&self) -> bool {
        matches!(self.entry_type, EntryType::Pattern(_))
    }

    /// Check if this entry is a comment
    pub fn is_comment(&self) -> bool {
        matches!(self.entry_type, EntryType::Comment(_))
    }

    /// Check if this entry is blank
    pub fn is_blank(&self) -> bool {
        matches!(self.entry_type, EntryType::Blank)
    }

    /// Get the normalized pattern for comparison (if this is a pattern)
    pub fn normalized_pattern(&self) -> Option<String> {
        match &self.entry_type {
            EntryType::Pattern(pattern) => Some(pattern.to_string()),
            _ => None,
        }
    }
}

/// Represents a complete .gitignore file
#[derive(Debug, Clone)]
pub struct GitignoreFile {
    /// All entries in the file
    pub entries: Vec<GitignoreEntry>,
    /// Statistics about the file
    pub stats: FileStats,
}

impl GitignoreFile {
    /// Create a new empty gitignore file
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            stats: FileStats::new(),
        }
    }

    /// Add an entry to the file
    pub fn add_entry(&mut self, entry: GitignoreEntry) {
        self.stats.update(&entry);
        self.entries.push(entry);
    }

    /// Get all pattern entries
    pub fn patterns(&self) -> Vec<&GitignoreEntry> {
        self.entries.iter().filter(|e| e.is_pattern()).collect()
    }

    /// Get all comment entries
    pub fn comments(&self) -> Vec<&GitignoreEntry> {
        self.entries.iter().filter(|e| e.is_comment()).collect()
    }

    /// Convert back to string representation
    pub fn to_string(&self) -> String {
        self.entries
            .iter()
            .map(|entry| entry.original.clone())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Find duplicate patterns
    pub fn find_duplicates(&self) -> HashMap<String, Vec<usize>> {
        let mut duplicates: HashMap<String, Vec<usize>> = HashMap::new();
        
        for entry in &self.entries {
            if let Some(normalized) = entry.normalized_pattern() {
                duplicates
                    .entry(normalized)
                    .or_insert_with(Vec::new)
                    .push(entry.line_number);
            }
        }

        // Keep only entries with more than one occurrence
        duplicates.retain(|_, line_numbers| line_numbers.len() > 1);
        duplicates
    }
}

/// Statistics about a gitignore file
#[derive(Debug, Clone)]
pub struct FileStats {
    pub total_lines: usize,
    pub pattern_lines: usize,
    pub comment_lines: usize,
    pub blank_lines: usize,
    pub duplicate_patterns: usize,
}

impl FileStats {
    pub fn new() -> Self {
        Self {
            total_lines: 0,
            pattern_lines: 0,
            comment_lines: 0,
            blank_lines: 0,
            duplicate_patterns: 0,
        }
    }

    pub fn update(&mut self, entry: &GitignoreEntry) {
        self.total_lines += 1;
        match entry.entry_type {
            EntryType::Pattern(_) => self.pattern_lines += 1,
            EntryType::Comment(_) => self.comment_lines += 1,
            EntryType::Blank => self.blank_lines += 1,
        }
    }
}

impl Default for GitignoreFile {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for FileStats {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gitignore_entry_creation() {
        let entry = GitignoreEntry::new(
            "*.log".to_string(),
            EntryType::Pattern("*.log".to_string()),
            1,
        );
        assert!(entry.is_pattern());
        assert!(!entry.is_comment());
        assert!(!entry.is_blank());
        assert_eq!(entry.normalized_pattern(), Some("*.log".to_string()));
    }

    #[test]
    fn test_comment_entry() {
        let entry = GitignoreEntry::new(
            "# Logs".to_string(),
            EntryType::Comment("# Logs".to_string()),
            2,
        );
        assert!(!entry.is_pattern());
        assert!(entry.is_comment());
        assert!(!entry.is_blank());
        assert_eq!(entry.normalized_pattern(), None);
    }

    #[test]
    fn test_blank_entry() {
        let entry = GitignoreEntry::new("".to_string(), EntryType::Blank, 3);
        assert!(!entry.is_pattern());
        assert!(!entry.is_comment());
        assert!(entry.is_blank());
        assert_eq!(entry.normalized_pattern(), None);
    }

    #[test]
    fn test_gitignore_file_creation() {
        let file = GitignoreFile::new();
        assert_eq!(file.entries.len(), 0);
        assert_eq!(file.stats.total_lines, 0);
    }

    #[test]
    fn test_add_entries() {
        let mut file = GitignoreFile::new();
        
        file.add_entry(GitignoreEntry::new(
            "*.log".to_string(),
            EntryType::Pattern("*.log".to_string()),
            1,
        ));
        file.add_entry(GitignoreEntry::new(
            "# Logs".to_string(),
            EntryType::Comment("# Logs".to_string()),
            2,
        ));
        file.add_entry(GitignoreEntry::new("".to_string(), EntryType::Blank, 3));

        assert_eq!(file.entries.len(), 3);
        assert_eq!(file.stats.total_lines, 3);
        assert_eq!(file.stats.pattern_lines, 1);
        assert_eq!(file.stats.comment_lines, 1);
        assert_eq!(file.stats.blank_lines, 1);
    }

    #[test]
    fn test_find_duplicates() {
        let mut file = GitignoreFile::new();
        
        // Add duplicate patterns
        file.add_entry(GitignoreEntry::new(
            "*.log".to_string(),
            EntryType::Pattern("*.log".to_string()),
            1,
        ));
        file.add_entry(GitignoreEntry::new(
            "*.log".to_string(),
            EntryType::Pattern("*.log".to_string()),
            2,
        ));
        file.add_entry(GitignoreEntry::new(
            "build/".to_string(),
            EntryType::Pattern("build/".to_string()),
            3,
        ));

        let duplicates = file.find_duplicates();
        assert_eq!(duplicates.len(), 1);
        assert!(duplicates.contains_key("*.log"));
        assert_eq!(duplicates["*.log"], vec![1, 2]);
    }

    #[test]
    fn test_to_string() {
        let mut file = GitignoreFile::new();
        file.add_entry(GitignoreEntry::new(
            "*.log".to_string(),
            EntryType::Pattern("*.log".to_string()),
            1,
        ));
        file.add_entry(GitignoreEntry::new(
            "# Logs".to_string(),
            EntryType::Comment("# Logs".to_string()),
            2,
        ));

        let result = file.to_string();
        assert_eq!(result, "*.log\n# Logs");
    }

    // Test cases from TEST_MATRIX.md
    #[test]
    fn test_tc01_exact_deduplication() {
        let mut file = GitignoreFile::new();
        file.add_entry(GitignoreEntry::new(
            "*.log".to_string(),
            EntryType::Pattern("*.log".to_string()),
            1,
        ));
        file.add_entry(GitignoreEntry::new(
            "*.log".to_string(),
            EntryType::Pattern("*.log".to_string()),
            2,
        ));

        let duplicates = file.find_duplicates();
        assert_eq!(duplicates.len(), 1);
        assert_eq!(duplicates["*.log"], vec![1, 2]);
    }

    #[test]
    fn test_tc02_comment_preservation() {
        let mut file = GitignoreFile::new();
        file.add_entry(GitignoreEntry::new(
            "*.log".to_string(),
            EntryType::Pattern("*.log".to_string()),
            1,
        ));
        file.add_entry(GitignoreEntry::new(
            "*.log".to_string(),
            EntryType::Pattern("*.log".to_string()),
            2,
        ));
        file.add_entry(GitignoreEntry::new(
            "# comment".to_string(),
            EntryType::Comment("# comment".to_string()),
            3,
        ));

        let comments = file.comments();
        assert_eq!(comments.len(), 1);
        assert_eq!(comments[0].original, "# comment");
    }

    #[test]
    fn test_tc03_negation_support() {
        let mut file = GitignoreFile::new();
        file.add_entry(GitignoreEntry::new(
            "*.log".to_string(),
            EntryType::Pattern("*.log".to_string()),
            1,
        ));
        file.add_entry(GitignoreEntry::new(
            "!debug.log".to_string(),
            EntryType::Pattern("!debug.log".to_string()),
            2,
        ));

        let patterns = file.patterns();
        assert_eq!(patterns.len(), 2);
        assert_eq!(patterns[0].original, "*.log");
        assert_eq!(patterns[1].original, "!debug.log");
    }

    #[test]
    fn test_tc09_trailing_space() {
        let mut file = GitignoreFile::new();
        file.add_entry(GitignoreEntry::new(
            "*.log ".to_string(),
            EntryType::Pattern("*.log ".to_string()),
            1,
        ));
        file.add_entry(GitignoreEntry::new(
            "*.log".to_string(),
            EntryType::Pattern("*.log".to_string()),
            2,
        ));

        // These should be treated as different patterns due to trailing space
        let duplicates = file.find_duplicates();
        assert_eq!(duplicates.len(), 0);
    }

    #[test]
    fn test_tc15_case_sensitivity() {
        let mut file = GitignoreFile::new();
        file.add_entry(GitignoreEntry::new(
            "build/".to_string(),
            EntryType::Pattern("build/".to_string()),
            1,
        ));
        file.add_entry(GitignoreEntry::new(
            "BUILD/".to_string(),
            EntryType::Pattern("BUILD/".to_string()),
            2,
        ));

        // These should be treated as different patterns due to case
        let duplicates = file.find_duplicates();
        assert_eq!(duplicates.len(), 0);
    }
} 