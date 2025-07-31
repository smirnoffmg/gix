pub mod file;
pub mod patterns;

pub use file::{read_gitignore_file, write_gitignore_file, create_backup};
pub use patterns::*; 