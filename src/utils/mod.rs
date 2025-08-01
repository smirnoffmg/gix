pub mod file;
pub mod patterns;

pub use file::{create_backup, read_gitignore_file, write_gitignore_file};
pub use patterns::*;
