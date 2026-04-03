# Contributing to awald-core

Thank you for your interest in contributing to awald-core! This guide will help you get started with contributing to this Rust workspace for data analysis with Python integration.

## Table of Contents

- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Code Quality](#code-quality)
- [Testing](#testing)
- [Documentation](#documentation)
- [Submitting Changes](#submitting-changes)
- [Community](#community)

## Getting Started

### Prerequisites

- **Rust**: Latest stable version
- **Python**: 3.11+ 
- **uv**: Modern Python package manager
- **Git**: For version control

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/awald-app/awald-core.git
   cd awald-core
   ```

2. **Install Rust**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup component add clippy rustfmt
   ```

3. **Install uv**
   ```bash
   curl -LsSf https://astral.sh/uv/install.sh | sh
   ```

4. **Set up Python dependencies**
   ```bash
   uv sync
   ```

5. **Install pre-commit hooks**
   ```bash
   uv tool install pre-commit
   uv run pre-commit install
   ```

### Verify Setup

```bash
# Check Rust build
cargo build --workspace

# Check tests
cargo test --workspace --all-targets

# Check formatting
cargo fmt --all -- --check

# Check linting
cargo clippy --workspace --all-targets -- -D warnings
```

## Development Workflow

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/issue-description
```

### 2. Make Changes

- Follow the existing code style
- Write tests for new functionality
- Update documentation as needed

### 3. Run Quality Checks

```bash
# Run all pre-commit checks (mirrors CI)
uv run pre-commit run --all-files

# Or run specific checks
uv run pre-commit run cargo-fmt
uv run pre-commit run cargo-clippy
uv run pre-commit run cargo-test
```

### 4. Test Your Changes

```bash
# Run all tests
cargo test --workspace --all-targets

# Run specific test
cargo test --package awald-data --lib summary::tests::test_numeric_summary

# Run benchmarks (if applicable)
cargo bench --package awald-data
```

## Code Quality

### Rust Code Style

- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Follow Rust naming conventions
- Add `#[inline]` for small, performance-critical functions
- Use `Result<T, Error>` for error handling

### Python Integration

- Use `uv` for Python dependency management
- Follow Python PEP 8 style (via `black`)
- Add type hints where appropriate
- Document Python interfaces in Rust code

### Documentation

- Use `///` for public API documentation
- Use `//!` for module-level documentation
- Include examples in documentation
- Update [`README.md`](../README.md) for user-facing changes

## Testing

### Test Organization

```
crates/
├── awald-data/src/
│   ├── lib.rs
│   ├── summary.rs
│   └── tests/          # Integration tests
├── awald-engine/src/
│   ├── lib.rs
│   ├── executor.rs
│   └── tests/          # Integration tests
└── awald-session/src/
    ├── lib.rs
    └── tests/          # Integration tests
```

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_functionality() {
        // Arrange
        let input = create_test_input();
        
        // Act
        let result = function_under_test(input);
        
        // Assert
        assert_eq!(result.expected_value, actual_value);
    }

    #[test]
    fn test_error_handling() {
        let input = create_invalid_input();
        let result = function_under_test(input);
        assert!(result.is_err());
    }
}
```

### Running Tests

```bash
# All tests
cargo test --workspace

# Specific package
cargo test --package awald-data

# Specific test
cargo test test_numeric_summary

# Tests with output
cargo test -- --nocapture

# Release mode tests
cargo test --release
```

## Documentation

### Types of Documentation

1. **API Documentation** (`///` comments)
2. **Module Documentation** (`//!` comments)
3. **Examples** in documentation
4. **User Documentation** (`README.md`, guides)
5. **Developer Documentation** (this file, [`TODO.md`](TODO.md))

### Documentation Standards

- All public functions must have documentation
- Include examples where helpful
- Use proper markdown formatting
- Cross-reference related items

```rust
/// Computes summary statistics for a DataFrame column.
///
/// # Examples
///
/// ```
/// use awald_data::summary::compute_summary;
/// use polars::prelude::*;
///
/// let df = df!("A" => [1, 2, 3])?;
/// let summary = compute_summary(&df, "A")?;
/// assert_eq!(summary.name, "A");
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Arguments
///
/// * `frame` - The DataFrame to analyze
/// * `col_name` - Name of the column to summarize
///
/// # Returns
///
/// `VarSummary` containing the computed statistics
///
/// # Errors
///
/// Returns an error if the column doesn't exist or computation fails.
pub fn compute_summary(frame: &DataFrame, col_name: &str) -> Result<VarSummary> {
    // implementation
}
```

## Submitting Changes

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
feat: add new summary statistics feature
fix: resolve memory leak in data processing
docs: update API documentation
test: add integration tests for executor
refactor: improve error handling consistency
```

### Pull Request Process

1. **Update Documentation**
   - Update [`README.md`](../README.md) for user-facing changes
   - Update [`TODO.md`](TODO.md) for development roadmap changes
   - Add examples to API documentation

2. **Create Pull Request**
   - Use descriptive title
   - Fill out the PR template
   - Link relevant issues

3. **Code Review**
   - Address reviewer feedback
   - Ensure all checks pass
   - Update documentation as needed

### Pull Request Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] All tests pass
- [ ] New tests added
- [ ] Manual testing completed

## Documentation
- [ ] API documentation updated
- [ ] README updated
- [ ] Examples added

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Pre-commit hooks pass
- [ ] CI checks pass
```

## Project Structure

```
awald-core/
├── crates/                    # Rust crates
│   ├── awald-data/           # Data processing
│   ├── awald-engine/         # Python execution
│   └── awald-session/        # Session management
├── docs/                     # Documentation
│   ├── CONTRIBUTING.md      # This file
│   ├── PRE_COMMIT.md         # Pre-commit setup
│   └── TODO.md               # Development roadmap
├── .github/                  # GitHub configuration
│   ├── workflows/           # CI/CD workflows
│   └── dependabot.yml       # Dependency updates
├── Cargo.toml               # Workspace configuration
├── README.md                # Project overview
└── SECURITY.md              # Security policy
```

### Crate Responsibilities

- **awald-data**: Data structures, statistical summaries, file I/O
- **awald-engine**: Python interpreter integration, script execution
- **awald-session**: User session management, data store coordination

## Development Guidelines

### Performance

- Profile performance-critical code
- Use benchmarks for optimization decisions
- Consider memory usage in data processing
- Leverage Polars for efficient data operations

### Security

- Follow [security policy](../SECURITY.md)
- Validate all inputs from Python scripts
- Use safe Rust practices
- Review dependencies for vulnerabilities

### Python Integration

- Use `pyo3` for Python-Rust interoperability
- Handle Python GIL correctly
- Provide clear error messages for Python users
- Document Python-facing APIs

### Error Handling

- Use `thiserror` for error types
- Provide context in error messages
- Handle Python errors gracefully
- Log errors appropriately

## Getting Help

### Resources

- [README.md](../README.md) - Project overview
- [TODO.md](TODO.md) - Development roadmap
- [PRE_COMMIT.md](PRE_COMMIT.md) - Code quality setup
- [SECURITY.md](../SECURITY.md) - Security policy

### Community

- **Issues**: Report bugs or request features
- **Discussions**: Ask questions or share ideas
- **Pull Requests**: Contribute code or documentation

### Development Support

- Check existing issues before creating new ones
- Search discussions for similar questions
- Follow the contribution guidelines
- Be patient and respectful in all interactions

## Release Process

Releases follow [Semantic Versioning](https://semver.org/):

- **Major (X.0.0)**: Breaking changes
- **Minor (X.Y.0)**: New features, backward compatible
- **Patch (X.Y.Z)**: Bug fixes, backward compatible

### Release Checklist

- [ ] All tests pass
- [ ] Documentation updated
- [ ] Version numbers updated
- [ ] CHANGELOG updated
- [ ] Security review completed
- [ ] Performance benchmarks run

## License

By contributing to awald-core, you agree that your contributions will be licensed under the same license as the project.

---

Thank you for contributing to awald-core! Your contributions help make data analysis with Rust and Python better for everyone.
