use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "gix",
    about = "Optimize .gitignore files by removing duplicates and normalizing patterns",
    version,
    long_about = "GIX is a command-line tool that optimizes .gitignore files by detecting and removing duplicate patterns, normalizing whitespace, and preserving comments and blank lines while maintaining the file's functionality."
)]
pub struct Args {
    /// Path to the .gitignore file (defaults to .gitignore in current directory)
    #[arg(value_name = "FILE")]
    pub file: Option<PathBuf>,

    /// Output file (defaults to overwriting the input file)
    #[arg(short, long, value_name = "OUTPUT")]
    pub output: Option<PathBuf>,

    /// Create a backup of the original file before modifying
    #[arg(short, long)]
    pub backup: bool,

    /// Optimization mode
    #[arg(short, long, value_enum, default_value_t = OptimizationMode::Standard)]
    pub mode: OptimizationMode,

    /// Show detailed statistics about the optimization
    #[arg(short, long)]
    pub stats: bool,

    /// Dry run - show what would be changed without modifying the file
    #[arg(long)]
    pub dry_run: bool,

    /// Verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// Analyze patterns and show categorization
    #[arg(long)]
    pub analyze: bool,

    /// Detect and report pattern conflicts
    #[arg(long)]
    pub detect_conflicts: bool,

    /// Generate comments for patterns
    #[arg(long)]
    pub generate_comments: bool,

    /// Show pattern categories
    #[arg(long)]
    pub show_categories: bool,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum OptimizationMode {
    /// Standard optimization (remove duplicate patterns, preserve comments and blank lines)
    Standard,
    /// Aggressive optimization (also remove duplicate comments and limit blank lines)
    Aggressive,
    /// Conservative optimization (only remove exact duplicates)
    Conservative,
    /// Advanced optimization (use pattern analysis for better deduplication)
    Advanced,
}

impl Args {
    /// Get the input file path, defaulting to .gitignore in current directory
    pub fn input_file(&self) -> PathBuf {
        self.file.clone().unwrap_or_else(|| PathBuf::from(".gitignore"))
    }

    /// Get the output file path
    pub fn output_file(&self) -> PathBuf {
        self.output.clone().unwrap_or_else(|| self.input_file())
    }

    /// Check if we should create a backup
    pub fn should_backup(&self) -> bool {
        self.backup || !self.dry_run
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_args() {
        let args = Args::parse_from(&["gix"]);
        assert_eq!(args.input_file(), PathBuf::from(".gitignore"));
        assert_eq!(args.output_file(), PathBuf::from(".gitignore"));
        assert!(!args.backup);
        assert!(!args.stats);
        assert!(!args.dry_run);
        assert!(!args.verbose);
        assert!(!args.analyze);
        assert!(!args.detect_conflicts);
        assert!(!args.generate_comments);
        assert!(!args.show_categories);
    }

    #[test]
    fn test_custom_file() {
        let args = Args::parse_from(&["gix", "custom.gitignore"]);
        assert_eq!(args.input_file(), PathBuf::from("custom.gitignore"));
        assert_eq!(args.output_file(), PathBuf::from("custom.gitignore"));
    }

    #[test]
    fn test_output_file() {
        let args = Args::parse_from(&["gix", "--output", "output.gitignore"]);
        assert_eq!(args.input_file(), PathBuf::from(".gitignore"));
        assert_eq!(args.output_file(), PathBuf::from("output.gitignore"));
    }

    #[test]
    fn test_backup_flag() {
        let args = Args::parse_from(&["gix", "--backup"]);
        assert!(args.should_backup());
    }

    #[test]
    fn test_dry_run() {
        let args = Args::parse_from(&["gix", "--dry-run"]);
        assert!(!args.should_backup());
    }

    #[test]
    fn test_analyze_flag() {
        let args = Args::parse_from(&["gix", "--analyze"]);
        assert!(args.analyze);
    }

    #[test]
    fn test_detect_conflicts_flag() {
        let args = Args::parse_from(&["gix", "--detect-conflicts"]);
        assert!(args.detect_conflicts);
    }

    #[test]
    fn test_generate_comments_flag() {
        let args = Args::parse_from(&["gix", "--generate-comments"]);
        assert!(args.generate_comments);
    }

    #[test]
    fn test_show_categories_flag() {
        let args = Args::parse_from(&["gix", "--show-categories"]);
        assert!(args.show_categories);
    }
} 