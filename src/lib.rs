//! GIX - Gitignore Optimizer
//! 
//! A command-line Rust tool that optimizes `.gitignore` files by detecting and removing 
//! duplicate patterns, normalizing whitespace, and preserving comments and blank lines.

pub mod cli;
pub mod core;
pub mod models;
pub mod utils;

pub use models::errors::GixError;
pub use models::gitignore::GitignoreFile;
pub use core::parser::parse_gitignore;
pub use core::optimizer::optimize_gitignore; 