pub mod categorizer;
pub mod comment_generator;
pub mod normalizer;
pub mod optimizer;
pub mod parser;
pub mod pattern_analyzer;
pub mod validator;

pub use categorizer::{CategorySummary, PatternCategorizer, PatternCategory};
pub use comment_generator::CommentGenerator;
pub use normalizer::{normalize_pattern, patterns_equivalent, patterns_equivalent_case_sensitive};
pub use optimizer::{
    analyze_gitignore, optimize_gitignore, optimize_gitignore_aggressive, GitignoreAnalysis,
};
pub use parser::parse_gitignore;
pub use pattern_analyzer::{PatternAnalysis, PatternAnalyzer, PatternType};
pub use validator::{is_valid_pattern, validate_pattern};
