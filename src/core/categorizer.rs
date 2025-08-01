use std::collections::HashMap;

/// Represents a category of gitignore patterns
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PatternCategory {
    /// Programming language patterns
    Language(String),
    /// Framework patterns
    Framework(String),
    /// Tool patterns (IDE, build tools, etc.)
    Tool(String),
    /// Operating system patterns
    OperatingSystem(String),
    /// Custom/user-defined patterns
    Custom(String),
    /// Uncategorized patterns
    Uncategorized,
}

impl PatternCategory {
    /// Get the display name for the category
    pub fn display_name(&self) -> String {
        match self {
            PatternCategory::Language(lang) => format!("Language: {}", lang),
            PatternCategory::Framework(framework) => format!("Framework: {}", framework),
            PatternCategory::Tool(tool) => format!("Tool: {}", tool),
            PatternCategory::OperatingSystem(os) => format!("OS: {}", os),
            PatternCategory::Custom(custom) => format!("Custom: {}", custom),
            PatternCategory::Uncategorized => "Uncategorized".to_string(),
        }
    }

    /// Get the short name for the category
    pub fn short_name(&self) -> String {
        match self {
            PatternCategory::Language(lang) => lang.clone(),
            PatternCategory::Framework(framework) => framework.clone(),
            PatternCategory::Tool(tool) => tool.clone(),
            PatternCategory::OperatingSystem(os) => os.clone(),
            PatternCategory::Custom(custom) => custom.clone(),
            PatternCategory::Uncategorized => "Uncategorized".to_string(),
        }
    }
}

/// Categorizer for gitignore patterns
pub struct PatternCategorizer {
    /// Language-specific patterns
    language_patterns: HashMap<String, Vec<String>>,
    /// Framework-specific patterns
    framework_patterns: HashMap<String, Vec<String>>,
    /// Tool-specific patterns
    tool_patterns: HashMap<String, Vec<String>>,
    /// OS-specific patterns
    os_patterns: HashMap<String, Vec<String>>,
}

impl Default for PatternCategorizer {
    fn default() -> Self {
        let mut categorizer = Self {
            language_patterns: HashMap::new(),
            framework_patterns: HashMap::new(),
            tool_patterns: HashMap::new(),
            os_patterns: HashMap::new(),
        };

        // Initialize with common patterns
        categorizer.initialize_common_patterns();
        categorizer
    }
}

impl PatternCategorizer {
    /// Create a new pattern categorizer
    pub fn new() -> Self {
        Self::default()
    }

    /// Initialize with common gitignore patterns
    fn initialize_common_patterns(&mut self) {
        // Language patterns
        self.add_language_patterns(
            "Python",
            &[
                "*.py[cod]",
                "*.so",
                "__pycache__/",
                "*.egg",
                "*.egg-info/",
                "dist/",
                "build/",
                "eggs/",
                "parts/",
                "bin/",
                "var/",
                "sdist/",
                "develop-eggs/",
                "*.egg-info/",
                ".installed.cfg",
                "*.manifest",
                "*.spec",
                "pip-log.txt",
                "pip-delete-this-directory.txt",
                ".Python",
                "env/",
                "venv/",
                "ENV/",
                "env.bak/",
                "venv.bak/",
                ".pytest_cache/",
                ".coverage",
                "htmlcov/",
                ".tox/",
                ".nox/",
                ".cache",
                ".mypy_cache/",
                ".dmypy.json",
                "dmypy.json",
            ],
        );

        self.add_language_patterns(
            "Node.js",
            &[
                "node_modules/",
                "npm-debug.log*",
                "yarn-debug.log*",
                "yarn-error.log*",
                "lerna-debug.log*",
                ".npm",
                ".eslintcache",
                ".node_repl_history",
                "*.tgz",
                ".yarn-integrity",
                ".env.local",
                ".env.development.local",
                ".env.test.local",
                ".env.production.local",
                "coverage/",
                ".nyc_output",
                ".grunt",
                "bower_components/",
                ".lock-wscript",
                "build/Release",
                ".node_repl_history",
                "*.tgz",
                ".yarn-integrity",
                ".next/",
                "out/",
            ],
        );

        self.add_language_patterns(
            "Java",
            &[
                "*.class",
                "*.log",
                "*.ctxt",
                ".mtj.tmp/",
                "*.jar",
                "*.war",
                "*.nar",
                "*.ear",
                "*.zip",
                "*.tar.gz",
                "*.rar",
                "hs_err_pid*",
                "replay_pid*",
                "target/",
                "!.mvn/wrapper/maven-wrapper.jar",
                "!**/src/main/**/target/",
                "!**/src/test/**/target/",
                ".idea/",
                "*.iws",
                "*.iml",
                "*.ipr",
                ".gradle/",
                "build/",
                "!gradle/wrapper/gradle-wrapper.jar",
            ],
        );

        self.add_language_patterns(
            "Rust",
            &[
                "target/",
                "Cargo.lock",
                "*.pdb",
                "*.exe",
                "*.dll",
                "*.so",
                "*.dylib",
                "*.rlib",
                "*.rmeta",
                "*.rbc",
                "*.dSYM/",
                "*.su",
                "*.idb",
                "*.pdb",
                "*.ilk",
                "*.exp",
                "*.lib",
                "*.a",
                "*.o",
                "*.so",
                "*.dylib",
            ],
        );

        self.add_language_patterns(
            "Go",
            &[
                "*.exe",
                "*.exe~",
                "*.dll",
                "*.so",
                "*.dylib",
                "*.test",
                "*.out",
                "go.work",
                "vendor/",
                ".go-version",
            ],
        );

        // Framework patterns
        self.add_framework_patterns(
            "React",
            &[
                "node_modules/",
                ".pnp",
                ".pnp.js",
                "coverage/",
                "build/",
                ".DS_Store",
                ".env.local",
                ".env.development.local",
                ".env.test.local",
                ".env.production.local",
                "npm-debug.log*",
                "yarn-debug.log*",
                "yarn-error.log*",
                ".next/",
                "out/",
            ],
        );

        self.add_framework_patterns(
            "Django",
            &[
                "*.log",
                "local_settings.py",
                "db.sqlite3",
                "db.sqlite3-journal",
                "media/",
                "staticfiles/",
                ".env",
                ".venv",
                "env/",
                "venv/",
                "ENV/",
                "env.bak/",
                "venv.bak/",
                ".pytest_cache/",
            ],
        );

        self.add_framework_patterns(
            "Spring",
            &[
                "*.class",
                "*.log",
                "*.ctxt",
                ".mtj.tmp/",
                "*.jar",
                "*.war",
                "*.nar",
                "*.ear",
                "*.zip",
                "*.tar.gz",
                "*.rar",
                "hs_err_pid*",
                "replay_pid*",
                "target/",
                ".idea/",
                "*.iws",
                "*.iml",
                "*.ipr",
            ],
        );

        // Tool patterns
        self.add_tool_patterns(
            "VSCode",
            &[
                ".vscode/",
                "*.code-workspace",
                ".vscode/settings.json",
                ".vscode/tasks.json",
                ".vscode/launch.json",
                ".vscode/extensions.json",
            ],
        );

        self.add_tool_patterns(
            "IntelliJ",
            &[".idea/", "*.iws", "*.iml", "*.ipr", ".idea_modules/"],
        );

        self.add_tool_patterns(
            "Eclipse",
            &[
                ".metadata",
                "bin/",
                "tmp/",
                "*.tmp",
                "*.bak",
                "*.swp",
                "*~.nib",
                "local.properties",
                ".settings/",
                ".loadpath",
                ".recommenders",
            ],
        );

        self.add_tool_patterns(
            "Vim",
            &["*.swp", "*.swo", "*~", ".vim/", ".viminfo", ".vimrc"],
        );

        self.add_tool_patterns(
            "Emacs",
            &[
                "*~",
                "#*#",
                ".#*",
                ".emacs.desktop",
                ".emacs.desktop.lock",
                "*.elc",
                "auto-save-list",
                "tramp",
                ".emacs.desktop.lock",
            ],
        );

        // OS patterns
        self.add_os_patterns(
            "macOS",
            &[
                ".DS_Store",
                ".AppleDouble",
                ".LSOverride",
                "Icon",
                "._*",
                ".DocumentRevisions-V100",
                ".fseventsd",
                ".Spotlight-V100",
                ".TemporaryItems",
                ".Trashes",
                ".VolumeIcon.icns",
                ".com.apple.timemachine.donotpresent",
                ".AppleDB",
                ".AppleDesktop",
                "Network Trash Folder",
                "Temporary Items",
                ".apdisk",
                ".VolumeIcon.icns",
                ".fseventsd",
                ".Spotlight-V100",
            ],
        );

        self.add_os_patterns(
            "Windows",
            &[
                "Thumbs.db",
                "Thumbs.db:encryptable",
                "ehthumbs.db",
                "ehthumbs_vista.db",
                "*.tmp",
                "*.temp",
                "Desktop.ini",
                "$RECYCLE.BIN/",
                "*.cab",
                "*.msi",
                "*.msix",
                "*.msm",
                "*.msp",
                "*.lnk",
                "*.stackdump",
            ],
        );

        self.add_os_patterns(
            "Linux",
            &[
                "*~",
                "*.swp",
                "*.swo",
                "*~",
                ".nfs*",
                ".fuse_hidden*",
                ".directory",
                ".Trash-*",
                ".nfs*",
                ".fuse_hidden*",
            ],
        );
    }

    /// Add language-specific patterns
    fn add_language_patterns(&mut self, language: &str, patterns: &[&str]) {
        self.language_patterns.insert(
            language.to_string(),
            patterns.iter().map(|s| s.to_string()).collect(),
        );
    }

    /// Add framework-specific patterns
    fn add_framework_patterns(&mut self, framework: &str, patterns: &[&str]) {
        self.framework_patterns.insert(
            framework.to_string(),
            patterns.iter().map(|s| s.to_string()).collect(),
        );
    }

    /// Add tool-specific patterns
    fn add_tool_patterns(&mut self, tool: &str, patterns: &[&str]) {
        self.tool_patterns.insert(
            tool.to_string(),
            patterns.iter().map(|s| s.to_string()).collect(),
        );
    }

    /// Add OS-specific patterns
    fn add_os_patterns(&mut self, os: &str, patterns: &[&str]) {
        self.os_patterns.insert(
            os.to_string(),
            patterns.iter().map(|s| s.to_string()).collect(),
        );
    }

    /// Categorize a single pattern
    pub fn categorize_pattern(&self, pattern: &str) -> PatternCategory {
        let normalized_pattern = pattern.trim();

        // Check OS patterns first (most specific)
        for (os, patterns) in &self.os_patterns {
            if patterns
                .iter()
                .any(|p| self.pattern_matches(normalized_pattern, p))
            {
                return PatternCategory::OperatingSystem(os.clone());
            }
        }

        // Check language patterns
        for (language, patterns) in &self.language_patterns {
            if patterns
                .iter()
                .any(|p| self.pattern_matches(normalized_pattern, p))
            {
                return PatternCategory::Language(language.clone());
            }
        }

        // Check tool patterns
        for (tool, patterns) in &self.tool_patterns {
            if patterns
                .iter()
                .any(|p| self.pattern_matches(normalized_pattern, p))
            {
                return PatternCategory::Tool(tool.clone());
            }
        }

        // Check framework patterns last (least specific)
        for (framework, patterns) in &self.framework_patterns {
            if patterns
                .iter()
                .any(|p| self.pattern_matches(normalized_pattern, p))
            {
                return PatternCategory::Framework(framework.clone());
            }
        }

        // Check for common custom patterns
        if self.is_custom_pattern(normalized_pattern) {
            return PatternCategory::Custom("Project-specific".to_string());
        }

        PatternCategory::Uncategorized
    }

    /// Check if a pattern matches a known pattern (with wildcard support)
    fn pattern_matches(&self, pattern: &str, known_pattern: &str) -> bool {
        // Exact match
        if pattern == known_pattern {
            return true;
        }

        // Handle character classes like [cod]
        if known_pattern.contains('[') && known_pattern.contains(']') && self.matches_with_character_class(pattern, known_pattern) {
            return true;
        }

        // Check for wildcard matches
        if known_pattern.contains('*') {
            // Simple wildcard matching
            let parts: Vec<&str> = known_pattern.split('*').collect();
            if parts.len() == 2 {
                let prefix = parts[0];
                let suffix = parts[1];
                if pattern.starts_with(prefix) && pattern.ends_with(suffix) {
                    return true;
                }
            }
        }

        // Check if pattern contains the known pattern as a substring
        if pattern.contains(known_pattern) {
            return true;
        }

        // Check if known pattern contains the pattern as a substring
        if known_pattern.contains(pattern) {
            return true;
        }

        false
    }

    /// Check if a pattern matches a pattern with character class
    fn matches_with_character_class(&self, pattern: &str, known_pattern: &str) -> bool {
        // Simple character class matching for [cod] -> c, o, or d
        if let Some(start) = known_pattern.find('[') {
            if let Some(end) = known_pattern.find(']') {
                if start < end {
                    let before = &known_pattern[..start];
                    let after = &known_pattern[end + 1..];
                    let chars_in_class = &known_pattern[start + 1..end];

                    // Try each character in the class
                    for ch in chars_in_class.chars() {
                        let test_pattern = format!("{}{}{}", before, ch, after);
                        if pattern == test_pattern {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    /// Check if a pattern looks like a custom/project-specific pattern
    fn is_custom_pattern(&self, pattern: &str) -> bool {
        // Patterns that are likely custom
        pattern.starts_with("custom/")
            || pattern.starts_with("project/")
            || pattern.starts_with("local/")
            || pattern.starts_with("temp/")
            || pattern.starts_with("tmp/")
            || pattern.contains("config")
            || pattern.contains("settings")
            || pattern.contains("local")
            || pattern.contains("dev")
            || pattern.contains("prod")
            || pattern.contains("test")
    }

    /// Categorize multiple patterns and return grouped results
    pub fn categorize_patterns(
        &self,
        patterns: &[String],
    ) -> HashMap<PatternCategory, Vec<String>> {
        let mut categorized: HashMap<PatternCategory, Vec<String>> = HashMap::new();

        for pattern in patterns {
            let category = self.categorize_pattern(pattern);
            categorized
                .entry(category)
                .or_default()
                .push(pattern.clone());
        }

        categorized
    }

    /// Get a summary of pattern categories
    pub fn get_category_summary(&self, patterns: &[String]) -> CategorySummary {
        let categorized = self.categorize_patterns(patterns);
        let mut summary = CategorySummary::new();

        for (category, pattern_list) in categorized {
            summary.add_category(category, pattern_list.len());
        }

        summary
    }
}

/// Summary of pattern categories
#[derive(Debug, Clone)]
pub struct CategorySummary {
    /// Count by category
    pub category_counts: HashMap<PatternCategory, usize>,
    /// Total patterns
    pub total_patterns: usize,
}

impl CategorySummary {
    pub fn new() -> Self {
        Self {
            category_counts: HashMap::new(),
            total_patterns: 0,
        }
    }

    pub fn add_category(&mut self, category: PatternCategory, count: usize) {
        self.category_counts.insert(category, count);
        self.total_patterns += count;
    }

    pub fn get_top_categories(&self, limit: usize) -> Vec<(PatternCategory, usize)> {
        let mut categories: Vec<_> = self.category_counts.iter().collect();
        categories.sort_by(|a, b| b.1.cmp(a.1));
        categories
            .into_iter()
            .take(limit)
            .map(|(k, v)| (k.clone(), *v))
            .collect()
    }
}

impl Default for CategorySummary {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_categorize_python_pattern() {
        let categorizer = PatternCategorizer::new();
        let category = categorizer.categorize_pattern("*.pyc");
        assert_eq!(category, PatternCategory::Language("Python".to_string()));
    }

    #[test]
    fn test_categorize_node_pattern() {
        let categorizer = PatternCategorizer::new();
        let category = categorizer.categorize_pattern("node_modules/");
        assert_eq!(category, PatternCategory::Language("Node.js".to_string()));
    }

    #[test]
    fn test_categorize_vscode_pattern() {
        let categorizer = PatternCategorizer::new();
        let category = categorizer.categorize_pattern(".vscode/");
        assert_eq!(category, PatternCategory::Tool("VSCode".to_string()));
    }

    #[test]
    fn test_categorize_macos_pattern() {
        let categorizer = PatternCategorizer::new();
        let category = categorizer.categorize_pattern(".DS_Store");
        assert_eq!(
            category,
            PatternCategory::OperatingSystem("macOS".to_string())
        );
    }

    #[test]
    fn test_categorize_custom_pattern() {
        let categorizer = PatternCategorizer::new();
        let category = categorizer.categorize_pattern("custom/");
        assert_eq!(
            category,
            PatternCategory::Custom("Project-specific".to_string())
        );
    }

    #[test]
    fn test_categorize_uncategorized_pattern() {
        let categorizer = PatternCategorizer::new();
        let category = categorizer.categorize_pattern("random_file.txt");
        assert_eq!(category, PatternCategory::Uncategorized);
    }

    #[test]
    fn test_categorize_multiple_patterns() {
        let categorizer = PatternCategorizer::new();
        let patterns = vec![
            "*.pyc".to_string(),
            "node_modules/".to_string(),
            ".vscode/".to_string(),
            "custom_file.txt".to_string(),
        ];

        let categorized = categorizer.categorize_patterns(&patterns);

        assert_eq!(categorized.len(), 4);
        assert!(categorized.contains_key(&PatternCategory::Language("Python".to_string())));
        assert!(categorized.contains_key(&PatternCategory::Language("Node.js".to_string())));
        assert!(categorized.contains_key(&PatternCategory::Tool("VSCode".to_string())));
        assert!(categorized.contains_key(&PatternCategory::Uncategorized));
    }

    #[test]
    fn test_category_summary() {
        let categorizer = PatternCategorizer::new();
        let patterns = vec![
            "*.pyc".to_string(),
            "*.pyo".to_string(),
            "node_modules/".to_string(),
            ".vscode/".to_string(),
        ];

        let summary = categorizer.get_category_summary(&patterns);

        assert_eq!(summary.total_patterns, 4);
        assert_eq!(
            summary
                .category_counts
                .get(&PatternCategory::Language("Python".to_string())),
            Some(&2)
        );
        assert_eq!(
            summary
                .category_counts
                .get(&PatternCategory::Language("Node.js".to_string())),
            Some(&1)
        );
        assert_eq!(
            summary
                .category_counts
                .get(&PatternCategory::Tool("VSCode".to_string())),
            Some(&1)
        );
    }

    #[test]
    fn test_pattern_matching() {
        let categorizer = PatternCategorizer::new();

        // Test exact match
        assert!(categorizer.pattern_matches("*.pyc", "*.pyc"));

        // Test substring match
        assert!(categorizer.pattern_matches("__pycache__/", "__pycache__/"));

        // Test wildcard match
        assert!(categorizer.pattern_matches("file.pyc", "*.pyc"));

        // Test no match
        assert!(!categorizer.pattern_matches("file.txt", "*.pyc"));
    }
}
