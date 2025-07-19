# Contributing to Temporal Fast Downward Planner

Thank you for your interest in contributing to the Temporal Fast Downward Planner! This document provides guidelines and information for contributors.

## 🚀 Quick Start

### Development Setup

1. **Clone the repository**
   ```bash
   git clone https://github.com/caelumspace/temporal-planner.git
   cd temporal-planner
   ```

2. **Install Rust** (if not already installed)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

3. **Build the project**
   ```bash
   cargo build
   ```

4. **Run tests**
   ```bash
   cargo test
   cargo run --bin comprehensive_tests
   ```

## 🧪 Testing

### Before Submitting Changes

Always run the full test suite:

```bash
# Run all standard tests
cargo test

# Run end-to-end tests
cargo run --bin e2e_tests

# Run comprehensive test suite
cargo run --bin comprehensive_tests

# Run performance benchmarks
cargo run --bin benchmark

# Check code formatting
cargo fmt --check

# Run linter
cargo clippy -- -D warnings
```

### Adding New Tests

When adding new functionality:

1. **Add unit tests** in the same file as your implementation
2. **Add integration tests** in `tests/integration_tests.rs` if applicable
3. **Add PDDL test domains** in `tests/fixtures/` for new features
4. **Update benchmarks** if performance is affected

## 📝 Code Style

### Rust Guidelines

- Follow standard Rust formatting: `cargo fmt`
- Address all Clippy warnings: `cargo clippy`
- Use meaningful variable and function names
- Add documentation comments for public APIs
- Prefer `Result<T, E>` over panicking for error handling

### Code Organization

```rust
// File structure example
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Public struct with documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalAction {
    pub name: String,
    // ... fields
}

impl TemporalAction {
    /// Create a new temporal action
    pub fn new(name: String) -> Self {
        // ... implementation
    }
}
```

## 🏗️ Architecture Guidelines

### Adding New Components

1. **PDDL Features**: Add parsing logic in `temporal_task.rs`
2. **Search Algorithms**: Implement in `search.rs` with the `TemporalSearchEngine` trait
3. **Heuristics**: Add to `heuristics.rs` with the `TemporalHeuristic` trait
4. **Temporal Reasoning**: Extend `scheduler.rs` for STN functionality

### Performance Considerations

- Profile code with `cargo bench` when adding performance-critical features
- Avoid unnecessary allocations in hot paths
- Use `&str` instead of `String` for temporary references
- Consider `Box<dyn Trait>` for dynamic dispatch when needed

## 📋 Submission Process

### Pull Request Guidelines

1. **Create a feature branch**
   ```bash
   git checkout -b feature/description-of-change
   ```

2. **Make focused commits** with clear messages
   ```bash
   git commit -m "Add: temporal heuristic implementation"
   ```

3. **Update documentation** for new features
4. **Add or update tests** for your changes
5. **Run the full test suite** before submitting

### Commit Message Format

Use conventional commits:

```
type(scope): description

body (optional)

footer (optional)
```

Types:
- `feat`: New features
- `fix`: Bug fixes
- `docs`: Documentation changes
- `test`: Test additions or modifications
- `refactor`: Code refactoring
- `perf`: Performance improvements

Examples:
```
feat(parser): add support for numeric fluents
fix(search): correct temporal constraint handling
docs(readme): update installation instructions
test(e2e): add factory automation test case
```

## 🐛 Bug Reports

### Creating Issues

When reporting bugs, include:

1. **Description** of the problem
2. **Steps to reproduce** the issue
3. **Expected behavior** vs actual behavior
4. **Environment information** (OS, Rust version)
5. **PDDL files** that cause the issue (if applicable)
6. **Error messages** or logs

### Bug Report Template

```markdown
**Bug Description**
A clear description of what the bug is.

**To Reproduce**
Steps to reproduce the behavior:
1. Run command '...'
2. With PDDL file '...'
3. See error

**Expected Behavior**
What you expected to happen.

**Environment**
- OS: [e.g. Windows 10, Ubuntu 20.04]
- Rust version: [e.g. 1.70.0]
- Project version: [e.g. 0.1.0]

**Additional Context**
Any other context about the problem.
```

## 💡 Feature Requests

### Proposing New Features

1. **Search existing issues** to avoid duplicates
2. **Create a detailed proposal** with:
   - Problem description
   - Proposed solution
   - Alternatives considered
   - Implementation complexity
3. **Discuss the feature** before implementing large changes

## 🔧 Development Tips

### Useful Commands

```bash
# Quick development cycle
cargo check          # Fast compilation check
cargo test --lib     # Run only library tests
cargo test integration_tests::test_name  # Run specific test

# Documentation
cargo doc --open     # Generate and open documentation
cargo doc --no-deps  # Generate docs without dependencies

# Profiling
cargo build --release
perf record --call-graph=dwarf target/release/benchmark
```

### IDE Setup

Recommended VS Code extensions:
- `rust-analyzer`: Rust language support
- `CodeLLDB`: Debugging support
- `Better TOML`: Cargo.toml editing
- `Error Lens`: Inline error display

## 📚 Resources

### Learning Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [PDDL Reference](https://planning.wiki/ref/pddl)
- [Temporal Planning](https://www.aaai.org/Papers/JAIR/Vol20/JAIR-2003.pdf)

### Project-Specific Documentation

- `README.md`: Main project documentation
- `tests/README.md`: Test suite documentation
- `E2E_TEST_SUMMARY.md`: Testing accomplishments
- `CHANGELOG.md`: Version history

## ❓ Getting Help

### Communication Channels

- **GitHub Issues**: Bug reports and feature requests
- **GitHub Discussions**: General questions and ideas
- **Code Comments**: Implementation-specific questions

### Response Times

- **Bug reports**: Usually within 1-2 days
- **Feature requests**: Review within a week
- **Pull requests**: Review within 3-5 days

## 🙏 Recognition

Contributors will be:
- Listed in the project contributors
- Mentioned in release notes for significant contributions
- Credited in academic publications if applicable

Thank you for contributing to temporal planning research and development! 🚀
