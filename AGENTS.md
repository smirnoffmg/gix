# GIX - Gitignore Optimizer

## Project Overview

`gix` is a command-line Rust tool that optimizes `.gitignore` files by detecting and removing duplicate patterns, normalizing whitespace, and preserving comments and blank lines while maintaining the file's functionality.

## Architecture Principles

### Core Principles
- **TDD (Test-Driven Development)**: Write tests first, then implement functionality
- **SOLID**: Single Responsibility, Open/Closed, Liskov Substitution, Interface Segregation, Dependency Inversion
- **KISS (Keep It Simple, Stupid)**: Prefer simple, readable solutions over complex ones
- **YAGNI (You Aren't Gonna Need It)**: Don't implement features until they're actually needed
- **DRY (Don't Repeat Yourself)**: Eliminate code duplication

### Design Patterns
- **Command Pattern**: For CLI argument handling
- **Strategy Pattern**: For different pattern matching algorithms
- **Builder Pattern**: For constructing optimized gitignore content
- **Error Handling**: Use `Result<T, E>` and custom error types

## Project Structure

```
gix/
├── Cargo.toml
├── Cargo.lock
├── src/
│   ├── main.rs              # CLI entry point
│   ├── lib.rs               # Library root
│   ├── cli/                 # Command-line interface
│   │   ├── mod.rs
│   │   ├── args.rs          # Argument parsing
│   │   └── output.rs        # Output formatting
│   ├── core/                # Core business logic
│   │   ├── mod.rs
│   │   ├── parser.rs        # .gitignore file parsing
│   │   ├── optimizer.rs     # Pattern optimization logic
│   │   ├── normalizer.rs    # Pattern normalization
│   │   └── validator.rs     # Pattern validation
│   ├── models/              # Data structures
│   │   ├── mod.rs
│   │   ├── gitignore.rs     # GitignoreEntry and related types
│   │   └── errors.rs        # Custom error types
│   └── utils/               # Utility functions
│       ├── mod.rs
│       ├── file.rs          # File I/O operations
│       └── patterns.rs      # Pattern matching utilities
├── tests/                   # Integration tests
│   ├── common/              # Test utilities
│   ├── test_files/          # Sample .gitignore files for testing
│   └── integration_tests.rs
└── examples/                # Usage examples
    └── basic_usage.rs
```

## Agent Responsibilities

### 1. Core Logic Agent
**Responsibilities:**
- Implement `.gitignore` parsing logic
- Pattern normalization and deduplication
- Pattern validation and optimization
- Core business rules implementation

**Key Modules:**
- `src/core/parser.rs`
- `src/core/optimizer.rs`
- `src/core/normalizer.rs`
- `src/core/validator.rs`

**Testing Focus:**
- Unit tests for each parsing function
- Edge case handling (empty files, malformed patterns)
- Performance benchmarks for large files

### 2. CLI Interface Agent
**Responsibilities:**
- Command-line argument parsing
- User interaction and output formatting
- Help text and documentation
- Exit codes and error reporting

**Key Modules:**
- `src/cli/args.rs`
- `src/cli/output.rs`
- `src/main.rs`

**Testing Focus:**
- CLI argument validation
- Output formatting consistency
- Error message clarity

### 3. Data Models Agent
**Responsibilities:**
- Define data structures for gitignore entries
- Error type definitions
- Serialization/deserialization if needed
- Type safety and validation

**Key Modules:**
- `src/models/gitignore.rs`
- `src/models/errors.rs`

**Testing Focus:**
- Data structure invariants
- Error type coverage
- Memory efficiency

### 4. File Operations Agent
**Responsibilities:**
- Safe file I/O operations
- Backup creation before modifications
- Atomic file writes
- Cross-platform compatibility

**Key Modules:**
- `src/utils/file.rs`

**Testing Focus:**
- File permission handling
- Backup and restore functionality
- Atomic write operations
- Error recovery

### 5. Testing Agent
**Responsibilities:**
- Integration test suite
- Test data generation
- Performance testing
- Coverage analysis

**Key Modules:**
- `tests/integration_tests.rs`
- `tests/common/`
- `tests/test_files/`

**Testing Focus:**
- End-to-end functionality
- Real-world .gitignore files
- Performance benchmarks
- Error scenario coverage

## Implementation Guidelines

### Phase 1: Foundation (TDD Approach)
1. **Start with tests**: Create test cases for core functionality
2. **Minimal implementation**: Implement only what's needed to pass tests
3. **Refactor**: Clean up code while maintaining test coverage

### Phase 2: Core Features
1. **File parsing**: Read and parse .gitignore files
2. **Pattern normalization**: Handle whitespace and case differences
3. **Duplicate detection**: Identify and remove redundant patterns
4. **Output generation**: Create optimized .gitignore content

### Phase 3: CLI Interface
1. **Argument parsing**: Handle command-line options
2. **User feedback**: Progress indicators and error messages
3. **File operations**: Safe reading and writing

### Phase 4: Optimization
1. **Performance tuning**: Optimize for large files
2. **Memory efficiency**: Minimize allocations
3. **Error handling**: Comprehensive error recovery

## Testing Strategy

### Unit Tests
- Each function should have corresponding unit tests
- Test edge cases and error conditions
- Mock external dependencies where appropriate

### Integration Tests
- Test complete workflows with real .gitignore files
- Verify file I/O operations
- Test CLI interface end-to-end

### Performance Tests
- Benchmark with large .gitignore files
- Measure memory usage
- Test scalability

## Error Handling

### Custom Error Types
```rust
#[derive(Debug, thiserror::Error)]
pub enum GixError {
    #[error("File not found: {0}")]
    FileNotFound(String),
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    #[error("Invalid pattern: {0}")]
    InvalidPattern(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
```

### Error Recovery
- Graceful degradation when possible
- Clear error messages for users
- Proper exit codes for automation

## Performance Requirements

### Targets
- Process 10,000+ line .gitignore files in < 1 second
- Memory usage < 50MB for typical files
- Startup time < 100ms

### Optimization Strategies
- Use efficient string operations
- Minimize allocations
- Leverage Rust's zero-cost abstractions
- Profile and optimize hot paths

## Dependencies

### Minimal Dependencies
- `clap`: Command-line argument parsing
- `thiserror`: Error handling
- `anyhow`: Error propagation (optional)

### No External Dependencies For
- File I/O (use std::fs)
- String processing (use std::string)
- Pattern matching (implement custom logic)
- Collections (use std::collections)

## Collaboration Guidelines

### Code Review Checklist
- [ ] Tests pass and coverage is maintained
- [ ] Code follows Rust idioms and conventions
- [ ] Error handling is comprehensive
- [ ] Performance impact is considered
- [ ] Documentation is updated

### Commit Messages
- Use conventional commit format
- Reference issue numbers when applicable
- Include brief description of changes

### Branch Strategy
- `main`: Stable, tested code
- `feature/*`: New features
- `fix/*`: Bug fixes
- `test/*`: Test improvements

## Success Criteria

### Functional Requirements
- [ ] Successfully parse .gitignore files
- [ ] Detect and remove duplicate patterns
- [ ] Preserve comments and blank lines
- [ ] Generate optimized output
- [ ] Handle errors gracefully

### Quality Requirements
- [ ] 90%+ test coverage
- [ ] All tests pass
- [ ] No clippy warnings
- [ ] Documentation complete
- [ ] Performance targets met

### User Experience
- [ ] Clear error messages
- [ ] Helpful usage information
- [ ] Fast execution
- [ ] Safe file operations

## Future Enhancements (YAGNI - Not for Initial Implementation)

### Potential Features
- Interactive mode for pattern selection
- Backup file management
- Pattern grouping and organization
- Integration with git hooks
- Support for multiple .gitignore files

### Performance Optimizations
- Parallel processing for large files
- Memory-mapped file I/O
- Pattern compilation caching
- Incremental processing

## Getting Started

### Development Setup
```bash
# Clone and setup
git clone <repository>
cd gix

# Run tests
cargo test

# Run with sample file
cargo run -- --help

# Build release version
cargo build --release
```

### Testing
```bash
# Run all tests
cargo test

# Run with coverage
cargo tarpaulin

# Run benchmarks
cargo bench

# Run clippy
cargo clippy
```

This AGENTS.md provides a comprehensive guide for AI agents to collaborate effectively on the `gix` project while adhering to the specified principles and maintaining high code quality.
