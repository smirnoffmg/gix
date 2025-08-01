pub mod categorizer;
pub mod comment_generator;
pub mod normalizer;
pub mod optimizer;
pub mod parser;
pub mod pattern_analyzer;
pub mod validator;

pub use categorizer::{PatternCategorizer, PatternCategory, CategorySummary};
pub use comment_generator::CommentGenerator;
pub use normalizer::{normalize_pattern, patterns_equivalent, patterns_equivalent_case_sensitive};
pub use optimizer::{optimize_gitignore, optimize_gitignore_aggressive, analyze_gitignore, GitignoreAnalysis};
pub use parser::parse_gitignore;
pub use pattern_analyzer::{PatternAnalyzer, PatternAnalysis, PatternType};
pub use validator::{validate_pattern, is_valid_pattern}; 