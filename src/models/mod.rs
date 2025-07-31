pub mod errors;
pub mod gitignore;

pub use errors::GixError;
pub use gitignore::{GitignoreEntry, GitignoreFile, EntryType}; 