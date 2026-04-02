# awald-core

> Rust data layer and Python executor for Awald.

`awald-core` is a Cargo workspace of three crates that power the
[Awald](https://github.com/awald-app/awald) desktop application.
Each crate is independently usable — they do not require the Tauri shell.

[![CI](https://github.com/awald-app/awald-core/actions/workflows/ci.yml/badge.svg)](https://github.com/awald-app/awald-core/actions)
[![License](https://img.shields.io/badge/license-MIT-blue)](LICENSE)
[![crates.io](https://img.shields.io/crates/v/awald-data)](https://crates.io/crates/awald-data)

---

## Crates

| Crate | Path | Description |
|---|---|---|
| [`awald-data`](crates/awald-data) | `crates/awald-data` | Polars-backed dataset store with zero-copy viewport slicing |
| [`awald-engine`](crates/awald-engine) | `crates/awald-engine` | pyo3 Python executor with serialised GIL and linear execution |
| [`awald-session`](crates/awald-session) | `crates/awald-session` | Session state: variable registry, execution history, output blocks |

---

## Design

The three crates reflect a strict separation of concerns:

```
awald-session          orchestrates
    ├── awald-data     owns DataFrame memory (Arc<RwLock<DataFrame>>)
    └── awald-engine   owns Python interpreter (Mutex over pyo3 GIL)
```

**awald-data** never calls Python.
**awald-engine** never holds a DataFrame.
**awald-session** is the only crate that touches both.

This makes each crate independently testable without a Python interpreter
or a live dataset.

---

## Workspace layout

```
awald-core/
├── Cargo.toml              # workspace manifest
├── crates/
│   ├── awald-data/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── store.rs    # DataStore, DatasetMeta
│   │       ├── slice.rs    # viewport slicing, RowData serialisation
│   │       └── formats.rs  # CSV, Parquet, DTA, XLSX readers
│   ├── awald-engine/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── executor.rs # Executor, ExecutionResult
│   │       ├── capture.rs  # stdout/stderr capture
│   │       └── error.rs    # PythonError, ExecutorError
│   └── awald-session/
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── session.rs  # Session, SessionState
│           ├── registry.rs # variable name → DataStore handle
│           └── history.rs  # ExecutionRecord log
├── tests/
│   ├── integration/
│   └── fixtures/           # sample CSV/Parquet for tests
└── benches/                # criterion benchmarks
```

---

## Quick start

```toml
# Cargo.toml
[dependencies]
awald-data    = "0.1"
awald-engine  = "0.1"
awald-session = "0.1"
```

```rust
use awald_session::Session;

let mut session = Session::new()?;

// Load a dataset — stays in Rust memory
let meta = session.load("data.csv").await?;
println!("{} rows × {} cols", meta.nrows, meta.ncols);

// Execute Python — linearly, with output captured
let result = session.execute(r#"
import pyfixest as pf
fit = pf.feols("wage ~ hours + tenure | industry", data=df)
"#).await?;

println!("{}", result.stdout);
```

---

## Development

```bash
git clone https://github.com/awald-app/awald-core
cd awald-core

# Build all crates
cargo build --workspace

# Run tests
cargo test --workspace

# Run benchmarks
cargo bench --workspace

# Lint
cargo clippy --workspace -- -D warnings

# Security audit
cargo audit
```

**Prerequisites:**
- Rust 1.82+ stable (`rustup update stable`)
- Python 3.12+ in `PATH` (required by `awald-engine` tests)
- uv (`curl -LsSf https://astral.sh/uv/install.sh | sh`)

---

## Versioning

Follows [SemVer 2.0.0](https://semver.org). All three crates are versioned
together — a single release bumps all three to the same version.

Current: **`0.0.1` — pre-alpha, unstable API.**

---

## License

MIT — see [LICENSE](LICENSE).
