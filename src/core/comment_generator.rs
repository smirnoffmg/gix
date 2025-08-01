use std::collections::HashMap;
use crate::core::pattern_analyzer::PatternAnalysis;
use crate::core::categorizer::PatternCategory;

/// Generator for automatic comments on gitignore patterns
pub struct CommentGenerator {
    /// Predefined comments for common patterns
    pattern_comments: HashMap<String, String>,
    /// Comments for pattern categories
    category_comments: HashMap<PatternCategory, String>,
}

impl Default for CommentGenerator {
    fn default() -> Self {
        let mut generator = Self {
            pattern_comments: HashMap::new(),
            category_comments: HashMap::new(),
        };
        
        generator.initialize_comments();
        generator
    }
}

impl CommentGenerator {
    /// Create a new comment generator
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Initialize predefined comments
    fn initialize_comments(&mut self) {
        // Language-specific pattern comments
        self.pattern_comments.insert("*.pyc".to_string(), "Python bytecode files".to_string());
        self.pattern_comments.insert("__pycache__/".to_string(), "Python cache directory".to_string());
        self.pattern_comments.insert("*.pyo".to_string(), "Python optimized bytecode files".to_string());
        self.pattern_comments.insert("*.pyd".to_string(), "Python dynamic modules".to_string());
        self.pattern_comments.insert("*.so".to_string(), "Shared object files".to_string());
        self.pattern_comments.insert("*.egg".to_string(), "Python egg packages".to_string());
        self.pattern_comments.insert("*.egg-info/".to_string(), "Python egg metadata".to_string());
        self.pattern_comments.insert("dist/".to_string(), "Distribution/packaging directory".to_string());
        self.pattern_comments.insert("build/".to_string(), "Build output directory".to_string());
        self.pattern_comments.insert("venv/".to_string(), "Python virtual environment".to_string());
        self.pattern_comments.insert("env/".to_string(), "Python virtual environment".to_string());
        self.pattern_comments.insert(".env".to_string(), "Environment variables file".to_string());
        self.pattern_comments.insert(".coverage".to_string(), "Python coverage data".to_string());
        self.pattern_comments.insert(".pytest_cache/".to_string(), "Pytest cache directory".to_string());
        
        // Node.js pattern comments
        self.pattern_comments.insert("node_modules/".to_string(), "Node.js dependencies".to_string());
        self.pattern_comments.insert("npm-debug.log*".to_string(), "NPM debug logs".to_string());
        self.pattern_comments.insert("yarn-debug.log*".to_string(), "Yarn debug logs".to_string());
        self.pattern_comments.insert("yarn-error.log*".to_string(), "Yarn error logs".to_string());
        self.pattern_comments.insert("coverage/".to_string(), "Test coverage reports".to_string());
        self.pattern_comments.insert(".nyc_output".to_string(), "NYC coverage output".to_string());
        self.pattern_comments.insert(".next/".to_string(), "Next.js build output".to_string());
        self.pattern_comments.insert("out/".to_string(), "Build output directory".to_string());
        
        // Java pattern comments
        self.pattern_comments.insert("*.class".to_string(), "Java compiled classes".to_string());
        self.pattern_comments.insert("*.jar".to_string(), "Java archive files".to_string());
        self.pattern_comments.insert("*.war".to_string(), "Web application archive".to_string());
        self.pattern_comments.insert("target/".to_string(), "Maven build output".to_string());
        self.pattern_comments.insert(".gradle/".to_string(), "Gradle cache directory".to_string());
        
        // Rust pattern comments
        self.pattern_comments.insert("Cargo.lock".to_string(), "Cargo lock file".to_string());
        self.pattern_comments.insert("target/".to_string(), "Rust build output".to_string());
        self.pattern_comments.insert("*.pdb".to_string(), "Program database files".to_string());
        self.pattern_comments.insert("*.exe".to_string(), "Executable files".to_string());
        self.pattern_comments.insert("*.dll".to_string(), "Dynamic link libraries".to_string());
        self.pattern_comments.insert("*.so".to_string(), "Shared object files".to_string());
        self.pattern_comments.insert("*.dylib".to_string(), "Dynamic libraries (macOS)".to_string());
        
        // IDE pattern comments
        self.pattern_comments.insert(".vscode/".to_string(), "VSCode workspace settings".to_string());
        self.pattern_comments.insert(".idea/".to_string(), "IntelliJ IDEA settings".to_string());
        self.pattern_comments.insert("*.swp".to_string(), "Vim swap files".to_string());
        self.pattern_comments.insert("*.swo".to_string(), "Vim swap files".to_string());
        self.pattern_comments.insert("*~".to_string(), "Backup files".to_string());
        
        // OS pattern comments
        self.pattern_comments.insert(".DS_Store".to_string(), "macOS system files".to_string());
        self.pattern_comments.insert("Thumbs.db".to_string(), "Windows thumbnail cache".to_string());
        self.pattern_comments.insert("Desktop.ini".to_string(), "Windows desktop configuration".to_string());
        
        // Common pattern comments
        self.pattern_comments.insert("*.log".to_string(), "Log files".to_string());
        self.pattern_comments.insert("*.tmp".to_string(), "Temporary files".to_string());
        self.pattern_comments.insert("*.temp".to_string(), "Temporary files".to_string());
        self.pattern_comments.insert("*.bak".to_string(), "Backup files".to_string());
        self.pattern_comments.insert("*.cache".to_string(), "Cache files".to_string());
        self.pattern_comments.insert("*.pid".to_string(), "Process ID files".to_string());
        self.pattern_comments.insert("*.lock".to_string(), "Lock files".to_string());
        
        // Category comments
        self.category_comments.insert(
            PatternCategory::Language("Python".to_string()),
            "Python language files".to_string()
        );
        self.category_comments.insert(
            PatternCategory::Language("Node.js".to_string()),
            "Node.js language files".to_string()
        );
        self.category_comments.insert(
            PatternCategory::Language("Java".to_string()),
            "Java language files".to_string()
        );
        self.category_comments.insert(
            PatternCategory::Language("Rust".to_string()),
            "Rust language files".to_string()
        );
        self.category_comments.insert(
            PatternCategory::Tool("VSCode".to_string()),
            "VSCode editor files".to_string()
        );
        self.category_comments.insert(
            PatternCategory::Tool("IntelliJ".to_string()),
            "IntelliJ IDEA files".to_string()
        );
        self.category_comments.insert(
            PatternCategory::OperatingSystem("macOS".to_string()),
            "macOS system files".to_string()
        );
        self.category_comments.insert(
            PatternCategory::OperatingSystem("Windows".to_string()),
            "Windows system files".to_string()
        );
        self.category_comments.insert(
            PatternCategory::OperatingSystem("Linux".to_string()),
            "Linux system files".to_string()
        );
    }
    
    /// Generate a comment for a specific pattern
    pub fn generate_pattern_comment(&self, pattern: &str, analysis: &PatternAnalysis) -> Option<String> {
        // Check for exact pattern match
        if let Some(comment) = self.pattern_comments.get(pattern) {
            return Some(comment.clone());
        }
        
        // Check for wildcard pattern matches
        for (known_pattern, comment) in &self.pattern_comments {
            if self.pattern_matches_wildcard(pattern, known_pattern) {
                return Some(comment.clone());
            }
        }
        
        // Generate comment based on pattern analysis
        self.generate_analysis_comment(analysis)
    }
    
    /// Generate a comment based on pattern analysis
    fn generate_analysis_comment(&self, analysis: &PatternAnalysis) -> Option<String> {
        let mut parts = Vec::new();
        
        // Add type information
        match analysis.pattern_type {
            crate::core::pattern_analyzer::PatternType::File => {
                parts.push("file".to_string());
            }
            crate::core::pattern_analyzer::PatternType::Directory => {
                parts.push("directory".to_string());
            }
            crate::core::pattern_analyzer::PatternType::Both => {
                parts.push("file or directory".to_string());
            }
        }
        
        // Add negation information
        if analysis.is_negation {
            parts.insert(0, "Don't ignore".to_string());
        } else {
            parts.insert(0, "Ignore".to_string());
        }
        
        // Add wildcard information
        if analysis.has_wildcards {
            parts.push("with wildcards".to_string());
        }
        
        // Add absolute path information
        if analysis.is_absolute {
            parts.push("from root".to_string());
        }
        
        if parts.len() > 2 {
            Some(format!("{} {}", parts[0], parts[1..].join(" ")))
        } else {
            Some(parts.join(" "))
        }
    }
    
    /// Generate a section header comment for a category
    pub fn generate_section_header(&self, category: &PatternCategory) -> String {
        match category {
            PatternCategory::Language(lang) => format!("# {}", lang),
            PatternCategory::Framework(framework) => format!("# {}", framework),
            PatternCategory::Tool(tool) => format!("# {}", tool),
            PatternCategory::OperatingSystem(os) => format!("# {}", os),
            PatternCategory::Custom(custom) => format!("# {}", custom),
            PatternCategory::Uncategorized => "# Other".to_string(),
        }
    }
    
    /// Generate a category description comment
    pub fn generate_category_comment(&self, category: &PatternCategory) -> Option<String> {
        self.category_comments.get(category).cloned()
    }
    
    /// Check if a pattern matches a wildcard pattern
    fn pattern_matches_wildcard(&self, pattern: &str, wildcard_pattern: &str) -> bool {
        if !wildcard_pattern.contains('*') {
            return pattern == wildcard_pattern;
        }
        
        // Simple wildcard matching
        let parts: Vec<&str> = wildcard_pattern.split('*').collect();
        if parts.len() == 2 {
            let prefix = parts[0];
            let suffix = parts[1];
            pattern.starts_with(prefix) && pattern.ends_with(suffix)
        } else {
            false
        }
    }
    
    /// Generate comments for a list of patterns
    pub fn generate_pattern_comments(&self, patterns: &[String], analyses: &[PatternAnalysis]) -> Vec<Option<String>> {
        patterns.iter()
            .zip(analyses.iter())
            .map(|(pattern, analysis)| self.generate_pattern_comment(pattern, analysis))
            .collect()
    }
    
    /// Generate a comprehensive comment for a pattern with context
    pub fn generate_detailed_comment(&self, pattern: &str, analysis: &PatternAnalysis, category: &PatternCategory) -> String {
        let mut comment_parts = Vec::new();
        
        // Add specific pattern comment if available
        if let Some(specific_comment) = self.generate_pattern_comment(pattern, analysis) {
            comment_parts.push(specific_comment);
        }
        
        // Add category information
        if let Some(category_comment) = self.generate_category_comment(category) {
            comment_parts.push(category_comment);
        }
        
        // Add analysis details
        if analysis.has_wildcards {
            comment_parts.push("Contains wildcards".to_string());
        }
        
        if analysis.is_absolute {
            comment_parts.push("Absolute path".to_string());
        }
        
        if analysis.is_negation {
            comment_parts.push("Negation pattern".to_string());
        }
        
        if comment_parts.is_empty() {
            self.generate_analysis_comment(analysis).unwrap_or_else(|| "Pattern".to_string())
        } else {
            comment_parts.join("; ")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_pattern_comment_exact_match() {
        let generator = CommentGenerator::new();
        let analysis = PatternAnalysis::new("*.pyc".to_string(), "*.pyc".to_string());
        let comment = generator.generate_pattern_comment("*.pyc", &analysis);
        
        assert_eq!(comment, Some("Python bytecode files".to_string()));
    }

    #[test]
    fn test_generate_pattern_comment_wildcard_match() {
        let generator = CommentGenerator::new();
        let analysis = PatternAnalysis::new("file.pyc".to_string(), "file.pyc".to_string());
        let comment = generator.generate_pattern_comment("file.pyc", &analysis);
        
        assert_eq!(comment, Some("Python bytecode files".to_string()));
    }

    #[test]
    fn test_generate_analysis_comment() {
        let generator = CommentGenerator::new();
        let analysis = PatternAnalysis::new("build/".to_string(), "build/".to_string());
        let comment = generator.generate_analysis_comment(&analysis);
        
        assert!(comment.is_some());
        assert!(comment.unwrap().contains("directory"));
    }

    #[test]
    fn test_generate_negation_comment() {
        let generator = CommentGenerator::new();
        let analysis = PatternAnalysis::new("!debug.log".to_string(), "!debug.log".to_string());
        let comment = generator.generate_analysis_comment(&analysis);
        
        assert!(comment.is_some());
        assert!(comment.unwrap().contains("Don't ignore"));
    }

    #[test]
    fn test_generate_section_header() {
        let generator = CommentGenerator::new();
        let header = generator.generate_section_header(&PatternCategory::Language("Python".to_string()));
        
        assert_eq!(header, "# Python");
    }

    #[test]
    fn test_generate_category_comment() {
        let generator = CommentGenerator::new();
        let comment = generator.generate_category_comment(&PatternCategory::Language("Python".to_string()));
        
        assert_eq!(comment, Some("Python language files".to_string()));
    }

    #[test]
    fn test_generate_detailed_comment() {
        let generator = CommentGenerator::new();
        let analysis = PatternAnalysis::new("*.pyc".to_string(), "*.pyc".to_string());
        let comment = generator.generate_detailed_comment(
            "*.pyc",
            &analysis,
            &PatternCategory::Language("Python".to_string())
        );
        
        assert!(comment.contains("Python bytecode files"));
        assert!(comment.contains("Python language files"));
    }

    #[test]
    fn test_pattern_matches_wildcard() {
        let generator = CommentGenerator::new();
        
        assert!(generator.pattern_matches_wildcard("file.pyc", "*.pyc"));
        assert!(generator.pattern_matches_wildcard("*.pyc", "*.pyc"));
        assert!(!generator.pattern_matches_wildcard("file.txt", "*.pyc"));
    }
} 