pub mod errors;
pub mod gitignore;

pub use errors::GixError;
pub use gitignore::{EntryType, GitignoreEntry, GitignoreFile};
