use gix::{
    core::{
        analyze_gitignore, optimize_gitignore, parse_gitignore, CommentGenerator,
        GitignoreAnalysis, PatternAnalyzer, PatternCategorizer, PatternCategory,
    },
    models::GitignoreFile,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== GIX Advanced Features Demo ===\n");

    // Sample gitignore content with various patterns
    let content = r#"# Logs
*.log
*.log
logs/

# Python
*.pyc
__pycache__/
venv/

# Node.js
node_modules/
npm-debug.log*

# Build outputs
build/
dist/

# IDE
.vscode/
.idea/

# OS files
.DS_Store
Thumbs.db

# Custom patterns
custom/
local/

# Conflicts
*.tmp
!important.tmp
"#;

    println!("Original .gitignore content:");
    println!("{}", content);
    println!();

    // Parse the gitignore file
    let original_file = parse_gitignore(content)?;
    println!(
        "Parsed {} entries ({} patterns, {} comments, {} blank lines)",
        original_file.entries.len(),
        original_file.stats.pattern_lines,
        original_file.stats.comment_lines,
        original_file.stats.blank_lines
    );
    println!();

    // 1. Pattern Analysis
    println!("=== 1. Pattern Analysis ===");
    let analyzer = PatternAnalyzer::default();
    let pattern_strings: Vec<String> = original_file
        .entries
        .iter()
        .filter_map(|entry| {
            if let gix::models::EntryType::Pattern(pattern) = &entry.entry_type {
                Some(pattern.clone())
            } else {
                None
            }
        })
        .collect();

    for pattern in &pattern_strings {
        let analysis = analyzer.analyze_pattern(pattern);
        println!("Pattern: '{}'", pattern);
        println!("  - Type: {:?}", analysis.pattern_type);
        println!("  - Negation: {}", analysis.is_negation);
        println!("  - Absolute: {}", analysis.is_absolute);
        println!("  - Has wildcards: {}", analysis.has_wildcards);
        println!("  - Has globstar: {}", analysis.has_globstar);
        println!("  - Matches files: {}", analysis.matches_files);
        println!("  - Matches directories: {}", analysis.matches_directories);
        println!();
    }

    // 2. Pattern Categorization
    println!("=== 2. Pattern Categorization ===");
    let categorizer = PatternCategorizer::new();
    let categorized = categorizer.categorize_patterns(&pattern_strings);

    for (category, patterns) in &categorized {
        println!("{}: {} patterns", category.display_name(), patterns.len());
        for pattern in patterns {
            println!("  - {}", pattern);
        }
        println!();
    }

    // 3. Conflict Detection
    println!("=== 3. Conflict Detection ===");
    let conflicts = analyzer.find_conflicts(&pattern_strings);
    if conflicts.is_empty() {
        println!("No conflicts detected.");
    } else {
        println!("Found {} conflicts:", conflicts.len());
        for (pattern1, pattern2) in &conflicts {
            println!("  - '{}' conflicts with '{}'", pattern1, pattern2);
        }
    }
    println!();

    // 4. Comment Generation
    println!("=== 4. Comment Generation ===");
    let comment_generator = CommentGenerator::new();

    for pattern in &pattern_strings {
        let analysis = analyzer.analyze_pattern(pattern);
        let category = categorizer.categorize_pattern(pattern);

        if let Some(comment) = comment_generator.generate_pattern_comment(pattern, &analysis) {
            println!("{} -> {}", pattern, comment);
        } else {
            let detailed_comment =
                comment_generator.generate_detailed_comment(pattern, &analysis, &category);
            println!("{} -> {}", pattern, detailed_comment);
        }
    }
    println!();

    // 5. Advanced Analysis
    println!("=== 5. Advanced Analysis ===");
    let analysis = analyze_gitignore(&original_file)?;
    println!("Pattern Analysis Summary:");
    println!("  - Total patterns: {}", analysis.total_patterns);
    println!("  - File patterns: {}", analysis.file_patterns);
    println!("  - Directory patterns: {}", analysis.directory_patterns);
    println!("  - Both patterns: {}", analysis.both_patterns);
    println!("  - Negation patterns: {}", analysis.negation_patterns);
    println!("  - Absolute patterns: {}", analysis.absolute_patterns);
    println!("  - Wildcard patterns: {}", analysis.wildcard_patterns);
    println!("  - Globstar patterns: {}", analysis.globstar_patterns);
    println!("  - Case sensitive: {}", analysis.case_sensitive_patterns);
    println!(
        "  - Case insensitive: {}",
        analysis.case_insensitive_patterns
    );
    println!("  - Conflicts: {}", analysis.conflict_count());
    println!();

    // 6. Optimization with Advanced Features
    println!("=== 6. Optimization Results ===");
    let optimized_file = optimize_gitignore(&original_file)?;
    println!(
        "Optimized {} entries ({} patterns, {} comments, {} blank lines)",
        optimized_file.entries.len(),
        optimized_file.stats.pattern_lines,
        optimized_file.stats.comment_lines,
        optimized_file.stats.blank_lines
    );

    let reduction = original_file.entries.len() - optimized_file.entries.len();
    let reduction_percent = (reduction as f64 / original_file.entries.len() as f64) * 100.0;
    println!(
        "Reduced by {} entries ({:.1}%)",
        reduction, reduction_percent
    );
    println!();

    // 7. Generate Organized Output
    println!("=== 7. Organized Output ===");
    let organized_content =
        generate_organized_gitignore(&original_file, &categorizer, &comment_generator)?;
    println!("Organized .gitignore:");
    println!("{}", organized_content);

    Ok(())
}

fn generate_organized_gitignore(
    file: &GitignoreFile,
    categorizer: &PatternCategorizer,
    comment_generator: &CommentGenerator,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut organized = String::new();

    // Extract patterns
    let pattern_strings: Vec<String> = file
        .entries
        .iter()
        .filter_map(|entry| {
            if let gix::models::EntryType::Pattern(pattern) = &entry.entry_type {
                Some(pattern.clone())
            } else {
                None
            }
        })
        .collect();

    // Categorize patterns
    let categorized = categorizer.categorize_patterns(&pattern_strings);

    // Generate organized content
    for (category, patterns) in &categorized {
        // Add section header
        organized.push_str(&comment_generator.generate_section_header(category));
        organized.push('\n');

        // Add category description if available
        if let Some(description) = comment_generator.generate_category_comment(category) {
            organized.push_str(&format!("# {}\n", description));
        }

        // Add patterns
        for pattern in patterns {
            organized.push_str(pattern);
            organized.push('\n');
        }

        organized.push('\n');
    }

    // Add uncategorized patterns at the end
    if let Some(uncategorized) = categorized.get(&PatternCategory::Uncategorized) {
        if !uncategorized.is_empty() {
            organized.push_str("# Other\n");
            for pattern in uncategorized {
                organized.push_str(pattern);
                organized.push('\n');
            }
        }
    }

    Ok(organized)
}
