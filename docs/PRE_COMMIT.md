# Pre-commit Configuration

This repository uses [pre-commit](https://pre-commit.com/) to ensure code quality and consistency across all commits. The pre-commit hooks are designed to mirror exactly what the CI workflow does in `.github/workflows/ci.yml`.

## Quick Start

### 1. Install uv
```bash
curl -LsSf https://astral.sh/uv/install.sh | sh
```

### 2. Install pre-commit
```bash
uv tool install pre-commit
```

### 3. Install pre-commit hooks
```bash
uv run pre-commit install
```

### 4. Run pre-commit on all files
```bash
uv run pre-commit run --all-files
```

## What Gets Checked

The pre-commit hooks mirror the CI workflow exactly:

### CI-Mirrored Checks

1. **cargo fmt check** - Mirrors CI `fmt check` step
   - Runs: `cargo fmt --all -- --check`
   - Ensures code formatting matches Rust standards

2. **cargo clippy** - Mirrors CI `clippy` step
   - Runs: `cargo clippy --workspace --all-targets -- -D warnings`
   - Lints Rust code for common mistakes and improvements

3. **cargo test** - Mirrors CI `test` step
   - Runs: `cargo test --workspace --all-targets`
   - Executes all tests in the workspace

4. **cargo audit** - Mirrors CI `cargo-audit` step
   - Runs: `cargo install cargo-audit --quiet && cargo audit`
   - Checks for known security vulnerabilities in dependencies

5. **cargo build benches** - Mirrors CI `bench-check` step
   - Runs: `cargo build --benches --workspace`
   - Ensures benchmarks compile successfully

6. **uv sync** - Mirrors CI Python dependency setup
   - Runs: `uv sync`
   - Ensures Python dependencies are properly installed

### Additional Quality Checks

- **File hygiene**: Trailing whitespace, end-of-file, YAML/JSON/TOML validation
- **Security**: Private key detection, merge conflict markers
- **Documentation**: Counts documented Rust files
- **TODO tracking**: Finds TODO/FIXME/XXX items for issue creation

## Local Development

### How it Works
When you run `git commit`, pre-commit will automatically run the same checks that CI runs. This means:

- **If pre-commit passes locally**: CI will almost certainly pass
- **If pre-commit fails locally**: Fix the issues before pushing
- **No surprises**: What works locally works in CI

### Skip Pre-commit (Not Recommended)
```bash
git commit --no-verify -m "commit message"
```

### Run on Specific Files
```bash
uv run pre-commit run --files src/main.rs
```

### Run Specific Hook
```bash
uv run pre-commit run cargo-fmt
```

## Troubleshooting

### Common Issues

1. **Pre-commit hook not found**
   ```bash
   uv run pre-commit install --install-hooks
   ```

2. **Rust toolchain issues**
   ```bash
   rustup update stable
   rustup component add clippy rustfmt
   ```

3. **uv not found**
   ```bash
   curl -LsSf https://astral.sh/uv/install.sh | sh
   ```

4. **Pre-commit not installed**
   ```bash
   uv tool install pre-commit
   ```

5. **Python dependency issues**
   ```bash
   uv sync
   ```

### Performance Tips

- Use `uv run pre-commit run --files <files>` to run hooks only on changed files
- Use `uv run pre-commit clean` to clean cached environments
- Use `PRE_COMMIT_ALLOW_FAILURE=1` for non-blocking checks during development

## Why Mirror CI?

### Benefits
1. **Fast feedback**: Catch issues locally before pushing
2. **No CI surprises**: What passes locally passes in CI
3. **Consistent environment**: Same tools and versions locally and in CI
4. **Developer productivity**: Fix issues in your local environment

### What's Not Included
- **Multi-platform testing**: CI runs on Ubuntu, macOS, Windows
- **Publish dry-run**: Only runs on tagged releases
- **Cross-compilation**: Platform-specific CI checks

## Maintenance

- Update hook versions: `uv run pre-commit autoupdate`
- Review CI workflow changes and update pre-commit accordingly
- Keep uv and Rust tools updated

## Best Practices

1. **Always run pre-commit locally** before pushing
2. **Fix issues incrementally** rather than all at once
3. **Use meaningful commit messages** that describe the fixes
4. **Check CI logs** if pre-commit passes but CI fails (platform differences)

## Support

For pre-commit issues:
- [pre-commit documentation](https://pre-commit.com/)
- [GitHub discussions](https://github.com/pre-commit/pre-commit/discussions)
- Project issues for configuration-specific problems

For CI-related questions:
- Check `.github/workflows/ci.yml` for the exact CI steps
- Compare local vs CI environment differences
- Review CI logs for platform-specific issues

## See Also

- [CONTRIBUTING.md](CONTRIBUTING.md) - Full contribution guide
- [TODO.md](TODO.md) - Development roadmap
- [SECURITY.md](../SECURITY.md) - Security policy
- [README.md](../README.md) - Project overview
