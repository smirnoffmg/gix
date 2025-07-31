pub mod parser;
pub mod optimizer;
pub mod normalizer;
pub mod validator;

pub use parser::parse_gitignore;
pub use optimizer::{optimize_gitignore, optimize_gitignore_aggressive};
pub use normalizer::normalize_pattern;
pub use validator::validate_pattern; 