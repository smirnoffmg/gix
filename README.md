# GIX - Advanced Gitignore Optimizer

A powerful command-line Rust tool that optimizes `.gitignore` files by detecting and removing duplicate patterns, normalizing whitespace, and preserving comments and blank lines while maintaining the file's functionality.

## Features

### Core Optimization
- **Duplicate Detection**: Removes exact and functionally equivalent duplicate patterns
- **Pattern Normalization**: Normalizes patterns by removing trailing spaces and handling separators
- **Comment Preservation**: Maintains all comments and blank lines for readability
- **Multiple Optimization Modes**: Standard, Aggressive, Conservative, and Advanced modes

### Advanced Pattern Analysis
- **Pattern Type Detection**: Identifies whether patterns match files, directories, or both
- **Negation Pattern Support**: Properly handles negation patterns (starting with `!`)
- **Wildcard Analysis**: Detects patterns with wildcards (`*`, `?`, `[]`) and globstar (`**`)
- **Path Analysis**: Distinguishes between absolute and relative paths
- **Case Sensitivity**: Analyzes case sensitivity of patterns

### Pattern Categorization
- **Language-Specific Patterns**: Automatically categorizes patterns by programming language (Python, Node.js, Java, Rust, Go)
- **Framework Patterns**: Identifies framework-specific patterns (React, Django, Spring)
- **Tool Patterns**: Categorizes IDE and tool patterns (VSCode, IntelliJ, Vim, Emacs)
- **OS Patterns**: Detects operating system-specific patterns (macOS, Windows, Linux)
- **Custom Patterns**: Identifies project-specific and custom patterns

### Conflict Detection
- **Pattern Conflicts**: Detects contradictory patterns (e.g., `*.log` and `!*.log`)
- **Functional Equivalence**: Identifies patterns that are functionally equivalent
- **Conflict Reporting**: Provides detailed reports of detected conflicts

### Comment Generation
- **Automatic Comments**: Generates descriptive comments for common patterns
- **Category Headers**: Creates organized section headers for different pattern categories
- **Pattern Descriptions**: Provides human-readable descriptions of pattern purposes
- **Custom Comments**: Supports custom comment generation for project-specific patterns

### Advanced Output
- **Organized Structure**: Groups patterns by category with clear section headers
- **Statistics**: Provides detailed statistics about pattern types and optimization results
- **Conflict Warnings**: Highlights potential issues in the gitignore file
- **Optimization Reports**: Shows reduction in file size and duplicate removal

## Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/gix.git
cd gix

# Build the project
cargo build --release

# Install globally (optional)
cargo install --path .
```

## Usage

### Basic Usage

```bash
# Optimize the default .gitignore file
gix

# Optimize a specific file
gix path/to/.gitignore

# Create a backup before modifying
gix --backup

# Dry run to see what would be changed
gix --dry-run

# Show detailed statistics
gix --stats

# Verbose output
gix --verbose
```

### Advanced Features

```bash
# Use advanced optimization with pattern analysis
gix --mode advanced

# Analyze patterns and show categorization
gix --analyze

# Detect and report pattern conflicts
gix --detect-conflicts

# Generate comments for patterns
gix --generate-comments

# Show pattern categories
gix --show-categories

# Combine multiple features
gix --analyze --detect-conflicts --generate-comments --stats
```

### Optimization Modes

- **Standard** (default): Remove duplicate patterns, preserve comments and blank lines
- **Aggressive**: Also remove duplicate comments and limit consecutive blank lines
- **Conservative**: Only remove exact duplicates
- **Advanced**: Use pattern analysis for better deduplication and organization

## Examples

### Basic Optimization

```bash
$ gix --dry-run
DRY RUN - No changes will be made
✅ Removed 3 duplicate line(s)

Original file: 25 lines
Optimized file: 22 lines
```

### Advanced Analysis

```bash
$ gix --analyze --stats
📊 Statistics:
  Original file:
    Total lines: 25
    Pattern lines: 15
    Comment lines: 5
    Blank lines: 5
  
  Optimized file:
    Total lines: 22
    Pattern lines: 12
    Comment lines: 5
    Blank lines: 5
  
  Optimization:
    Lines removed: 3
    Size reduction: 12.0%

Pattern Analysis Summary:
  - Total patterns: 15
  - File patterns: 2
  - Directory patterns: 8
  - Both patterns: 5
  - Negation patterns: 1
  - Wildcard patterns: 4
  - Conflicts: 0
```

### Pattern Categorization

```bash
$ gix --show-categories
Language: Python: 3 patterns
  - *.pyc
  - __pycache__/
  - venv/

Language: Node.js: 2 patterns
  - node_modules/
  - npm-debug.log*

Tool: VSCode: 1 patterns
  - .vscode/

OS: macOS: 1 patterns
  - .DS_Store

Custom: Project-specific: 2 patterns
  - custom/
  - local/
```

### Conflict Detection

```bash
$ gix --detect-conflicts
Found 1 conflicts:
  - '*.log' conflicts with '!debug.log'
```

## Advanced Example

Run the comprehensive example to see all features in action:

```bash
cargo run --example advanced_usage
```

This demonstrates:
- Pattern analysis and categorization
- Conflict detection
- Comment generation
- Organized output generation
- Statistics and optimization results

## Pattern Analysis

GIX provides detailed analysis of gitignore patterns:

### Pattern Types
- **File**: Patterns that match only files (e.g., `*.log`)
- **Directory**: Patterns that match only directories (e.g., `build/`)
- **Both**: Patterns that match both files and directories (e.g., `*.tmp`)

### Pattern Properties
- **Negation**: Patterns starting with `!` that explicitly include files
- **Absolute**: Patterns starting with `/` that match from the repository root
- **Wildcards**: Patterns containing `*`, `?`, or `[]` for pattern matching
- **Globstar**: Patterns containing `**` for recursive matching

### Functional Equivalence
GIX can detect when patterns are functionally equivalent:
- `build` and `build/` (directory patterns)
- `/build` and `build` (absolute vs relative)
- `*.log` and `*.log` (exact duplicates)

## Categorization System

GIX automatically categorizes patterns into:

### Languages
- **Python**: `*.pyc`, `__pycache__/`, `venv/`, etc.
- **Node.js**: `node_modules/`, `npm-debug.log*`, etc.
- **Java**: `*.class`, `*.jar`, `target/`, etc.
- **Rust**: `Cargo.lock`, `target/`, `*.pdb`, etc.
- **Go**: `*.exe`, `vendor/`, etc.

### Frameworks
- **React**: `.next/`, `out/`, etc.
- **Django**: `db.sqlite3`, `media/`, etc.
- **Spring**: `*.war`, `.gradle/`, etc.

### Tools
- **VSCode**: `.vscode/`, `*.code-workspace`, etc.
- **IntelliJ**: `.idea/`, `*.iml`, etc.
- **Vim**: `*.swp`, `*.swo`, etc.
- **Emacs**: `*~`, `*.elc`, etc.

### Operating Systems
- **macOS**: `.DS_Store`, `.AppleDouble`, etc.
- **Windows**: `Thumbs.db`, `Desktop.ini`, etc.
- **Linux**: `*~`, `*.swp`, etc.

## Configuration

GIX uses sensible defaults but can be customized:

### Pattern Analyzer
- **Normalization**: Automatically normalizes patterns (removes trailing spaces, handles separators)
- **Case Sensitivity**: Configurable case sensitivity for pattern matching
- **Equivalence Detection**: Advanced detection of functionally equivalent patterns

### Categorizer
- **Language Patterns**: Comprehensive database of language-specific patterns
- **Framework Patterns**: Recognition of popular framework patterns
- **Tool Patterns**: Support for common development tools and IDEs
- **OS Patterns**: Operating system-specific file patterns

### Comment Generator
- **Predefined Comments**: Built-in comments for common patterns
- **Category Descriptions**: Automatic descriptions for pattern categories
- **Custom Comments**: Support for project-specific comment generation

## Development

### Project Structure
```
gix/
├── src/
│   ├── core/
│   │   ├── pattern_analyzer.rs    # Pattern analysis and normalization
│   │   ├── categorizer.rs         # Pattern categorization
│   │   ├── comment_generator.rs   # Automatic comment generation
│   │   ├── optimizer.rs           # Core optimization logic
│   │   ├── parser.rs              # Gitignore file parsing
│   │   └── validator.rs           # Pattern validation
│   ├── models/
│   │   ├── gitignore.rs           # Data structures
│   │   └── errors.rs              # Error handling
│   ├── cli/
│   │   ├── args.rs                # Command-line argument parsing
│   │   └── output.rs              # Output formatting
│   └── utils/
│       ├── file.rs                # File I/O operations
│       └── patterns.rs            # Pattern utilities
├── tests/
│   ├── unit_tests.rs              # Unit tests
│   └── integration_tests.rs       # Integration tests
└── examples/
    ├── basic_usage.rs             # Basic usage example
    └── advanced_usage.rs          # Advanced features example
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test categories
cargo test --test unit_tests
cargo test --test integration_tests

# Run with output
cargo test -- --nocapture
```

### Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Run the test suite
6. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by the need for better gitignore management
- Built with Rust for performance and reliability
- Uses TDD principles for robust development
- Comprehensive test coverage ensures reliability
