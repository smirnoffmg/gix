use gix::{
    core::{parse_gitignore, optimize_gitignore},
    models::GixError,
};

fn main() -> Result<(), GixError> {
    // Example .gitignore content with duplicates
    let content = r#"# Logs
*.log
*.log
logs/

# Build outputs
build/
BUILD/
dist/

# Dependencies
node_modules/
**/node_modules/

# IDE files
.vscode/
.idea/

# OS files
.DS_Store
Thumbs.db

# Temporary files
*.tmp
*.temp
*.swp

# Comments
# This is a comment
# Another comment

# More patterns
*.log
build/
"#;

    println!("Original .gitignore content:");
    println!("{}", content);
    println!();

    // Parse the content
    let original_file = parse_gitignore(content)?;
    println!("Parsed {} lines:", original_file.entries.len());
    println!("  - {} pattern lines", original_file.stats.pattern_lines);
    println!("  - {} comment lines", original_file.stats.comment_lines);
    println!("  - {} blank lines", original_file.stats.blank_lines);
    println!();

    // Find duplicates
    let duplicates = original_file.find_duplicates();
    if !duplicates.is_empty() {
        println!("Found duplicate patterns:");
        for (pattern, line_numbers) in &duplicates {
            println!("  {} (lines: {:?})", pattern, line_numbers);
        }
        println!();
    }

    // Optimize the file
    let optimized_file = optimize_gitignore(&original_file)?;
    println!("Optimized .gitignore content:");
    println!("{}", optimized_file.to_string());
    println!();

    println!("Optimization results:");
    println!("  - Original lines: {}", original_file.entries.len());
    println!("  - Optimized lines: {}", optimized_file.entries.len());
    println!("  - Lines removed: {}", original_file.entries.len() - optimized_file.entries.len());

    Ok(())
} 