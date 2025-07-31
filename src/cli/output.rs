use crate::models::{GitignoreFile, GixError};
use crate::cli::args::{Args, OptimizationMode};
use std::path::Path;

/// Print optimization results to the user
pub fn print_results(
    args: &Args,
    original_file: &GitignoreFile,
    optimized_file: &GitignoreFile,
    duplicates: &std::collections::HashMap<String, Vec<usize>>,
) -> Result<(), GixError> {
    if args.verbose {
        println!("Optimizing .gitignore file...");
    }

    let removed_lines = original_file.entries.len() - optimized_file.entries.len();
    
    if args.dry_run {
        println!("DRY RUN - No changes will be made");
    }
    
    if removed_lines > 0 {
        println!("✅ Removed {} duplicate line(s)", removed_lines);
        
        if args.verbose && !duplicates.is_empty() {
            println!("\nDuplicate patterns found:");
            for (pattern, line_numbers) in duplicates {
                println!("  {} (lines: {:?})", pattern, line_numbers);
            }
        }
    } else {
        println!("✅ No duplicates found - file is already optimized");
    }
    
    if args.stats {
        print_statistics(original_file, optimized_file);
    }
    
    if args.verbose {
        println!("\nOriginal file: {} lines", original_file.entries.len());
        println!("Optimized file: {} lines", optimized_file.entries.len());
    }
    
    Ok(())
}

/// Print detailed statistics about the optimization
fn print_statistics(original: &GitignoreFile, optimized: &GitignoreFile) {
    println!("\n📊 Statistics:");
    println!("  Original file:");
    println!("    Total lines: {}", original.stats.total_lines);
    println!("    Pattern lines: {}", original.stats.pattern_lines);
    println!("    Comment lines: {}", original.stats.comment_lines);
    println!("    Blank lines: {}", original.stats.blank_lines);
    
    println!("  Optimized file:");
    println!("    Total lines: {}", optimized.stats.total_lines);
    println!("    Pattern lines: {}", optimized.stats.pattern_lines);
    println!("    Comment lines: {}", optimized.stats.comment_lines);
    println!("    Blank lines: {}", optimized.stats.blank_lines);
    
    let reduction = original.stats.total_lines - optimized.stats.total_lines;
    let reduction_percent = if original.stats.total_lines > 0 {
        (reduction as f64 / original.stats.total_lines as f64) * 100.0
    } else {
        0.0
    };
    
    println!("  Optimization:");
    println!("    Lines removed: {}", reduction);
    println!("    Size reduction: {:.1}%", reduction_percent);
}

/// Print error messages to the user
pub fn print_error(error: &GixError) {
    eprintln!("❌ Error: {}", error);
}

/// Print success message
pub fn print_success(path: &Path) {
    println!("✅ Successfully optimized {}", path.display());
}

/// Print backup message
pub fn print_backup(path: &Path) {
    println!("💾 Created backup: {}", path.with_extension("backup").display());
}

/// Print mode information
pub fn print_mode(mode: &OptimizationMode) {
    match mode {
        OptimizationMode::Standard => println!("🔧 Using standard optimization mode"),
        OptimizationMode::Aggressive => println!("⚡ Using aggressive optimization mode"),
        OptimizationMode::Conservative => println!("🛡️ Using conservative optimization mode"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{GitignoreFile, GitignoreEntry, EntryType};


    #[test]
    fn test_print_statistics() {
        let mut original = GitignoreFile::new();
        original.add_entry(GitignoreEntry::new(
            "*.log".to_string(),
            EntryType::Pattern("*.log".to_string()),
            1,
        ));
        original.add_entry(GitignoreEntry::new(
            "*.log".to_string(),
            EntryType::Pattern("*.log".to_string()),
            2,
        ));
        
        let mut optimized = GitignoreFile::new();
        optimized.add_entry(GitignoreEntry::new(
            "*.log".to_string(),
            EntryType::Pattern("*.log".to_string()),
            1,
        ));
        
        // This test just ensures the function doesn't panic
        print_statistics(&original, &optimized);
    }

    #[test]
    fn test_print_error() {
        let error = GixError::FileNotFound("test.gitignore".to_string());
        // This test just ensures the function doesn't panic
        print_error(&error);
    }
} 