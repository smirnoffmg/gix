# gix

> A stealthy `.gitignore` optimizer for hackers who like clean repos.

`gix` parses your `.gitignore`, removes duplicate entries, preserves comments and structure, and writes back a minimal, tidy version. Fast, safe, and written in pure Rust.

## Features

- Fast and memory-safe Rust implementation
- Deduplicates ignore rules
- Preserves comments and blank lines
- Testable with in-memory streams (via `Cursor`)
- CLI-ready for scripting or Git hooks

## Usage

```
gix optimize                # Optimizes .gitignore in-place
gix optimize --dry-run     # Shows what would change
gix optimize --backup      # Makes a .bak before overwriting
```

## Install

```
cargo install gix
```

Or clone and build manually:

```
git clone https://github.com/yourname/gix
cd gix
cargo build --release
```

## Testing

```
cargo test
```

## Why `gix`?

- Your `.gitignore` is a mess. We both know it.
- Stop duplicating `*.log` 6 times.
- Let your brain focus on code, not clutter.

---

Made with 🦀 by hackers who care about clean noise.
