use clap::Parser;

use std::process;

use gix::{
    cli::{args::Args, output::{print_results, print_error, print_success, print_backup, print_mode}},
    core::{parse_gitignore, optimize_gitignore, optimize_gitignore_aggressive},
    models::GixError,
    utils::{read_gitignore_file, write_gitignore_file, create_backup},
};

fn main() {
    let args = Args::parse();
    
    if let Err(e) = run(args) {
        print_error(&e);
        process::exit(1);
    }
}

fn run(args: Args) -> Result<(), GixError> {
    let input_path = args.input_file();
    let output_path = args.output_file();
    
    // Print mode information
    if args.verbose {
        print_mode(&args.mode);
    }
    
    // Read the .gitignore file
    let content = read_gitignore_file(&input_path)?;
    
    // Parse the file
    let original_file = parse_gitignore(&content)?;
    
    // Find duplicates for reporting
    let duplicates = original_file.find_duplicates();
    
    // Optimize the file based on mode
    let optimized_file = match args.mode {
        gix::cli::args::OptimizationMode::Standard => {
            optimize_gitignore(&original_file)?
        }
        gix::cli::args::OptimizationMode::Aggressive => {
            optimize_gitignore_aggressive(&original_file)?
        }
        gix::cli::args::OptimizationMode::Conservative => {
            // For conservative mode, we only remove exact duplicates
            optimize_gitignore(&original_file)?
        }
    };
    
    // Print results
    print_results(&args, &original_file, &optimized_file, &duplicates)?;
    
    // If this is a dry run, don't modify the file
    if args.dry_run {
        return Ok(());
    }
    
    // Create backup if requested
    if args.should_backup() {
        create_backup(&input_path)?;
        if args.verbose {
            print_backup(&input_path);
        }
    }
    
    // Write the optimized content
    let optimized_content = optimized_file.to_string();
    write_gitignore_file(&output_path, &optimized_content)?;
    
    // Print success message
    print_success(&output_path);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn test_run_with_dry_run() {
        let temp_file = NamedTempFile::new().unwrap();
        let content = "*.log\n*.log\nbuild/";
        writeln!(temp_file.as_file(), "{}", content).unwrap();
        
        let args = Args::parse_from(&["gix", "--dry-run", temp_file.path().to_str().unwrap()]);
        let result = run(args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_with_nonexistent_file() {
        let args = Args::parse_from(&["gix", "nonexistent.gitignore"]);
        let result = run(args);
        assert!(result.is_err());
    }
}
