# TODO — awald-core

Full development roadmap for the `awald-core` Cargo workspace.
Follows [SemVer 2.0.0](https://semver.org) and standard OSS SDLC.

Status: `[ ]` open · `[x]` done · `[~]` in progress · `[-]` deferred · `[!]` blocking

Pre-release (`0.x.y`): breaking changes allowed between minors.
`1.0.0`: public API frozen, published to crates.io, consumed by `awald` v1.0.0.

---

## Current: Pre-Alpha (`0.0.x`) — Fix compile errors before any public push

These are `[!]` blocking — the workspace does not compile as-is.

### P0 — Compile errors (must fix first)

#### `awald-data`
- [ ] **Write `crates/awald-data/src/summary.rs`**
  - [ ] Define `VarSummary` struct
    - [ ] Fields: `name: String`, `dtype: String`, `nrows: usize`, `null_count: usize`
    - [ ] Numeric variant: `min`, `max`, `mean`, `std`, `p25`, `p50`, `p75` as `Option<f64>`
    - [ ] Categorical variant: `top_values: Vec<(String, usize)>` (top 10 value counts)
  - [ ] Derive `Debug`, `Clone`, `Serialize`, `Deserialize`
  - [ ] Export from `crates/awald-data/src/lib.rs` — already referenced, file missing
- [ ] **Fix `tempfile` in `crates/awald-data/Cargo.toml`**
  - [ ] Move `tempfile = "3"` from direct dep to `{ workspace = true }` pattern
  - [ ] Add `tempfile = "3"` to `[workspace.dependencies]` in root `Cargo.toml`

#### `awald-engine`
- [ ] **Fix `pyo3::ffi::c_str!` with runtime `String`**
  - [ ] `c_str!` macro only accepts string literals — using it with a `String` variable is a compile error
  - [ ] Replace with `std::ffi::CString::new(script.as_str()).map_err(|_| ...)?`
  - [ ] Or use `py.run_bound()` / `PyModule::from_code_bound()` from pyo3 0.24 API
  - [ ] Add `CString` test: verify null bytes in script produce clean error, not panic
- [ ] **Fix `MutexGuard` sent across `spawn_blocking`**
  - [ ] `parking_lot::MutexGuard<Py<PyDict>>` is not `Send`
  - [ ] `tokio::task::spawn_blocking` closure requires `Send + 'static`
  - [ ] Fix: clone the `Py<PyDict>` before entering `spawn_blocking`, drop guard before move
  - [ ] Pattern: `let globals_clone = self.globals.lock().clone();` then move `globals_clone`
  - [ ] Test: verify two concurrent `run()` calls do not deadlock
- [ ] **Remove unused `Duration` import in `executor.rs`**
  - [ ] `use std::time::{Duration, Instant}` — `Duration` is never used
  - [ ] `cargo clippy -- -D warnings` treats unused imports as errors

#### `awald-session` / tests
- [ ] **Wire integration tests to Cargo**
  - [ ] Create `tests/integration/main.rs` as test harness entry point
  - [ ] Add `[[test]]` entry in root `Cargo.toml`:
    ```toml
    [[test]]
    name    = "integration"
    path    = "tests/integration/main.rs"
    ```
  - [ ] Or move session tests to `crates/awald-session/tests/session_tests.rs` (preferred)
  - [ ] Verify `cargo test --workspace` discovers and runs them

### P0 verification gate

```bash
cargo build --workspace                    # zero errors
cargo test --workspace --all-targets       # all pass
cargo clippy --workspace -- -D warnings    # zero warnings
```

All three must pass before any commit to `main`.

---

## v0.1.0 — First Functional Release

**Goal:** `Session::load()` + `Session::execute()` work end-to-end.
A Python script can read the loaded DataFrame and run a regression.
**Distribution:** Used by `awald` desktop app v0.1.0. Not yet on crates.io.

### 0.1.0 — `awald-data`

- [ ] **Implement `VarSummary` computation in `summary.rs`**
  - [ ] `compute_summary(frame: &DataFrame, col_name: &str) -> Result<VarSummary>`
  - [ ] Numeric: use Polars `Series::mean()`, `Series::std()`, `Series::quantile()`
  - [ ] Categorical/String: use `Series::value_counts()`, take top 10
  - [ ] Null count: `Series::null_count()`
  - [ ] Test: numeric column → all stats present, no panics
  - [ ] Test: string column → top_values populated, numeric fields None
  - [ ] Test: all-null column → null_count == nrows, no divide-by-zero
- [ ] **Add `DataStore::summarize()` method**
  - [ ] `pub async fn summarize(&self, col: &str) -> Result<VarSummary>`
  - [ ] Acquires read lock, delegates to `summary::compute_summary`
- [ ] **Snapshot tests for `RowData` serialisation**
  - [ ] Use `insta` for golden snapshots of CSV → slice → JSON
  - [ ] Cover: integer, float, string, bool, null cells
  - [ ] Cover: empty slice (start == end)
  - [ ] Cover: slice beyond end of frame (clamped, no panic)
- [ ] **Unit tests for format dispatch**
  - [ ] `read_csv`: valid CSV round-trips correctly
  - [ ] `read_csv`: malformed CSV returns `PolarsError`, not panic
  - [ ] `read_parquet`: write then read round-trip
  - [ ] `read_stata`: test with a known `.dta` fixture file in `tests/fixtures/`
  - [ ] `read_excel`: returns `UnsupportedFormat` error cleanly
  - [ ] Unknown extension: returns `UnsupportedFormat`

### 0.1.0 — `awald-engine`

- [ ] **DataFrame → Python namespace injection**
  - [ ] This is the most critical missing piece in the entire codebase
  - [ ] `Executor` needs a method: `inject_dataframe(&self, name: &str, frame: Arc<RwLock<DataFrame>>) -> Result<()>`
  - [ ] Strategy: use `pyo3-polars` to convert Polars `DataFrame` → Python `polars.DataFrame` object
  - [ ] Add `pyo3-polars` to `awald-engine/Cargo.toml` dependencies
  - [ ] Add `polars` (Python) to `pyproject.toml` test env (already present)
  - [ ] Test: inject a frame as `df`, execute `print(df.shape)`, verify output matches
  - [ ] Test: inject frame, execute Polars filter expression, verify no error
- [ ] **Block-level script splitting**
  - [ ] `pub fn split_blocks(script: &str) -> Vec<String>`
  - [ ] Splits on double newline (`\n\n`) — blank line separator
  - [ ] Preserves indentation and multi-line constructs (functions, classes)
  - [ ] Does not split inside `"""` docstrings or `'''` strings
  - [ ] Test: single block returns one element
  - [ ] Test: two blocks separated by blank line → two elements
  - [ ] Test: function definition across multiple lines stays together
  - [ ] Test: empty script → empty vec
- [ ] **`execute_blocks()` on `Executor`**
  - [ ] `pub async fn run_blocks(&self, script: &str) -> Vec<ExecutionResult>`
  - [ ] Calls `split_blocks`, runs each sequentially
  - [ ] Halts on first block with `error.is_some()` — does not run subsequent blocks
  - [ ] Returns partial results up to the error block
  - [ ] Test: three blocks, second errors → results has two entries, third not run
- [ ] **Timeout support**
  - [ ] Add `timeout_ms: Option<u64>` to `run()` signature (default `None`)
  - [ ] Use `tokio::time::timeout` wrapping `spawn_blocking`
  - [ ] On timeout: kill Python thread (best-effort), return timeout error
  - [ ] Test: infinite loop `while True: pass` with 100ms timeout → returns error
- [ ] **Unit tests for `capture.rs`**
  - [ ] `install` + `drain` + `restore` round-trip
  - [ ] `print("hello")` → stdout captured as `"hello\n"`
  - [ ] `import sys; sys.stderr.write("err")` → stderr captured
  - [ ] After `restore`, `sys.stdout` is the real stdout again

### 0.1.0 — `awald-session`

- [ ] **Wire `load()` to Python namespace**
  - [ ] After `DataStore::load()`, call `executor.inject_dataframe("df", store.frame_handle())`
  - [ ] The injected name is always `df` for the primary dataset
  - [ ] Test: load CSV, execute `print(df.shape)`, verify output
- [ ] **`Session::execute_blocks()`**
  - [ ] Delegates to `executor.run_blocks()`
  - [ ] Returns `Vec<ExecutionResult>`
- [ ] **`Session::get_summary()`**
  - [ ] `pub async fn get_summary(&self, col: &str) -> Result<VarSummary>`
  - [ ] Delegates to `store.summarize(col)`, errors if no dataset loaded
- [ ] **Integration tests (all passing)**
  - [ ] `test_load_csv_returns_meta` — already written, must pass
  - [ ] `test_execute_simple_python` — already written, must pass
  - [ ] `test_execute_python_error_captured` — already written, must pass
  - [ ] `test_session_reset_clears_namespace` — already written, must pass
  - [ ] `test_load_and_use_dataframe` — new: load CSV, use `df` in Python
  - [ ] `test_execute_pyfixest_regression` — new: full regression via pyfixest
  - [ ] `test_block_halt_on_error` — new: second block errors, third not run
  - [ ] `test_get_summary_numeric` — new: VarSummary for numeric column
  - [ ] `test_get_summary_categorical` — new: VarSummary for string column

### 0.1.0 — Workspace quality

- [ ] **`cargo test --doc` passes** — all doc examples in `lib.rs` files compile and run
  - [ ] Fix `awald-data/src/lib.rs` example — uses `tokio_test::block_on` (add `tokio-test` to dev-deps)
  - [ ] Fix `awald-engine/src/lib.rs` example — same
  - [ ] Fix `awald-session/src/lib.rs` example — same
- [ ] **`cargo clippy --workspace -- -D warnings` zero warnings**
- [ ] **`cargo fmt --all -- --check` passes**
- [ ] **`cargo audit` zero known vulnerabilities**
- [ ] **CI green on all three platforms** (Linux, macOS, Windows)

---

## v0.2.0 — Production-quality `awald-data`

**Goal:** Data layer is robust, fast, and fully tested.

### 0.2.0 — Format support
- [ ] **Excel (`.xlsx`) support**
  - [ ] Add `polars` `calamine` feature to `awald-data/Cargo.toml`
  - [ ] Implement `read_excel()` using `CsvReader::from_path()` with calamine
  - [ ] Test: write known XLSX, read back, verify column names and types
  - [ ] Test: multi-sheet XLSX — reads first sheet by default
  - [ ] Test: XLSX with merged cells — handles gracefully, does not panic
- [ ] **Stata `.dta` value labels**
  - [ ] Verify Polars `StataReader` preserves value labels as Categorical dtype
  - [ ] Test: known `.dta` fixture with labelled variables → Categorical in schema
  - [ ] Test: `.dta` with date columns → correct date dtype, not integer
- [ ] **Compressed CSV** (`.csv.gz`, `.csv.zst`)
  - [ ] Polars supports these natively — add extensions to format dispatch
  - [ ] Test: gzip-compressed CSV round-trip
- [ ] **Format detection from content** (magic bytes fallback)
  - [ ] If extension is unknown/absent, probe first 8 bytes for Parquet magic, etc.
  - [ ] Test: file named `data` (no extension) that is actually a valid Parquet

### 0.2.0 — Performance
- [ ] **Benchmark: slice 50 rows from 1M-row dataset**
  - [ ] Target: < 1ms per call
  - [ ] Add to `benches/slice_bench.rs` — 1M row fixture
- [ ] **Benchmark: load 1M-row CSV**
  - [ ] Target: < 2s on standard laptop
  - [ ] Add to `benches/load_bench.rs`
- [ ] **Profile: verify zero full-DataFrame copy during slice**
  - [ ] Use `valgrind` or `heaptrack` to confirm no allocation spike
- [ ] **Streaming large files**
  - [ ] For files > 500MB, use Polars `LazyFrame` + `collect()` on slice only
  - [ ] Test: 2GB CSV loads without OOM on 4GB RAM machine

### 0.2.0 — API additions
- [ ] **`DataStore::schema()`** — returns `Vec<SchemaField>` without full meta clone
- [ ] **`DataStore::column_names()`** — returns `Vec<String>`
- [ ] **`DataStore::filter()`** — apply a Polars `LazyFrame` filter expression
  - [ ] `pub async fn filter(&self, expr: Expr) -> Result<DataStore>`
  - [ ] Returns a new `DataStore` with filtered frame — original unchanged
- [ ] **`DataStore::reload()`** — re-read from original path
  - [ ] `pub async fn reload(&mut self) -> Result<DatasetMeta>`

---

## v0.3.0 — Production-quality `awald-engine`

**Goal:** Executor is safe, observable, and handles adversarial scripts.

### 0.3.0 — Safety
- [ ] **Script sanitisation**
  - [ ] Reject scripts containing `sys.exit()`, `os._exit()`, `os.kill()`
  - [ ] Option to sandbox: `ExecutorConfig { allow_filesystem: bool, allow_network: bool }`
  - [ ] Test: `sys.exit(0)` in script → returns error, does not kill process
- [ ] **Memory limit**
  - [ ] `ExecutorConfig { max_memory_mb: Option<u64> }`
  - [ ] Poll `tracemalloc` or `resource.getrlimit` during execution
  - [ ] Test: script that allocates 1GB → returns error before OOM
- [ ] **Interrupt handling**
  - [ ] `Executor::interrupt()` — sends `KeyboardInterrupt` to running Python thread
  - [ ] Test: long-running script interrupted mid-execution

### 0.3.0 — Observability
- [ ] **Structured execution log**
  - [ ] Each `run()` emits a `tracing::span` with script hash, duration, error status
  - [ ] Log level: `DEBUG` for successful runs, `WARN` for errors
- [ ] **Execution history**
  - [ ] `Executor::history() -> Vec<ExecutionRecord>`
  - [ ] `ExecutionRecord`: timestamp, script_hash, duration_ms, error: Option<String>
  - [ ] Max history: 1000 entries (ring buffer)
- [ ] **Python version detection**
  - [ ] `Executor::python_version() -> String`
  - [ ] Surfaced in `awald` status bar

### 0.3.0 — API additions
- [ ] **`Executor::eval()`** — evaluate a single expression, return JSON value
  - [ ] `pub async fn eval(&self, expr: &str) -> Result<serde_json::Value>`
  - [ ] Used for variable inspection in the variable browser
  - [ ] Test: `eval("1 + 1")` → `Value::Number(2)`
  - [ ] Test: `eval("df.shape")` → `Value::Array([100, 4])`
- [ ] **`Executor::get_namespace()`** — list variable names and types in current scope
  - [ ] `pub async fn get_namespace(&self) -> Result<Vec<NamespaceEntry>>`
  - [ ] `NamespaceEntry { name: String, type_name: String, shape: Option<String> }`
  - [ ] Filters out builtins and dunder names
  - [ ] Test: after `x = 42`, namespace includes `{ name: "x", type_name: "int" }`

---

## v0.4.0 — Production-quality `awald-session`

**Goal:** Session is the complete interface for `awald` to use. No direct crate access needed.

### 0.4.0 — Multi-dataset support
- [ ] **Load multiple datasets**
  - [ ] `session.load_as("df2", "other_data.csv")` — load with custom variable name
  - [ ] Both `df` and `df2` available in Python namespace
  - [ ] `session.frames()` → `HashMap<String, DatasetMeta>` of all loaded frames
- [ ] **Dataset registry**
  - [ ] Implement `crates/awald-session/src/registry.rs` (currently missing)
  - [ ] `Registry { frames: HashMap<String, DataStore> }`
  - [ ] Methods: `insert`, `get`, `remove`, `names`
  - [ ] Test: register three datasets, retrieve by name, remove one

### 0.4.0 — Execution history
- [ ] **Implement `crates/awald-session/src/history.rs`** (currently missing)
  - [ ] `ExecutionRecord { block_index: usize, script: String, result: ExecutionResult, timestamp: u64 }`
  - [ ] `History { records: VecDeque<ExecutionRecord> }` — max 500 entries
  - [ ] Methods: `push`, `iter`, `last`, `clear`
  - [ ] Test: execute 10 blocks, history has 10 entries in order
- [ ] **`Session::history()`** — returns `Vec<ExecutionRecord>`
- [ ] **`Session::export_script()`** — concatenates all successful blocks into a `.py` string
  - [ ] Test: load + 3 execute calls → export produces valid, runnable Python

### 0.4.0 — Session persistence
- [ ] **Save/restore session state**
  - [ ] `session.save("~/.awald/sessions/latest.json")` — serialises metadata + history
  - [ ] `session.restore("~/.awald/sessions/latest.json")` — rebuilds state
  - [ ] Does NOT serialise DataFrames (too large) — only paths + history
  - [ ] On restore: re-loads datasets from original paths
  - [ ] Test: save, restore, verify namespace matches

---

## v0.5.0 — API stabilisation pre-release

**Goal:** API is reviewed, documented, and locked for v1.0.0.

- [ ] **API review** — mark all intentionally public items with `#[doc]`
  - [ ] Every `pub` struct, enum, function has a doc comment
  - [ ] Every doc comment has at least one example
  - [ ] `cargo doc --no-deps --document-private-items` — zero warnings
- [ ] **Deprecation pass** — identify any v0.x API that should change before v1.0
  - [ ] Tag with `#[deprecated]` + migration notes
- [ ] **Breaking change review**
  - [ ] Any changes needed to `Session`, `DataStore`, `Executor` public API?
  - [ ] Document decisions in `CHANGELOG.md` under `[0.5.0]`
- [ ] **Semver compatibility check**
  - [ ] Add `cargo-semver-checks` to CI
  - [ ] Run: `cargo semver-checks check-release`
- [ ] **Security audit**
  - [ ] `cargo audit` — zero known CVEs
  - [ ] Manual review: pyo3 boundary, arbitrary code execution surface
  - [ ] Document security model in `SECURITY.md`

---

## v0.6.0 — Crate publishing preparation

**Goal:** Each crate publishable to crates.io independently.

### 0.6.0 — crates.io readiness checklist (per crate)
- [ ] `Cargo.toml` has all required fields: `description`, `keywords`, `categories`, `license`, `repository`, `homepage`
- [ ] `README.md` at crate root (not just workspace root) — crates.io renders crate-level README
  - [ ] Add `crates/awald-data/README.md`
  - [ ] Add `crates/awald-engine/README.md`
  - [ ] Add `crates/awald-session/README.md`
- [ ] `license-file` or `license` field present
- [ ] `exclude` field in `Cargo.toml` — exclude test fixtures, benches from published crate
  ```toml
  exclude = ["tests/fixtures/*", "benches/*"]
  ```
- [ ] `cargo package --list` — verify only intended files are included
- [ ] `cargo publish --dry-run` — succeeds for all three crates
- [ ] Published in dependency order: `awald-data` → `awald-engine` → `awald-session`

### 0.6.0 — Documentation site
- [ ] `docs.rs` configuration in each `Cargo.toml`
  ```toml
  [package.metadata.docs.rs]
  all-features = true
  rustdoc-args  = ["--cfg", "docsrs"]
  ```
- [ ] All feature flags documented with `#[cfg_attr(docsrs, doc(cfg(feature = "...")))]`
- [ ] `cargo doc --open` locally — no broken links, no missing items

### 0.6.0 — CI release pipeline
- [ ] Tag-triggered publish workflow: `.github/workflows/release.yml`
  - [ ] Triggered on `v*` tags
  - [ ] Publishes `awald-data`, then `awald-engine`, then `awald-session` in order
  - [ ] Uses `CARGO_REGISTRY_TOKEN` secret
  - [ ] Creates GitHub Release with changelog excerpt
- [ ] Verify `publish-dry-run` job in `ci.yml` passes on tag push

---

## v1.0.0 — First Stable Release

**Criteria — all must be met before tagging `v1.0.0`:**

- [ ] All `v0.1.0` through `v0.6.0` milestones complete
- [ ] `cargo test --workspace` — 0 failures, 0 ignored
- [ ] `cargo clippy --workspace -- -D warnings` — 0 warnings
- [ ] `cargo test --doc` — all doc examples pass
- [ ] `cargo audit` — 0 known vulnerabilities
- [ ] `cargo semver-checks` — no unintentional breaking changes
- [ ] All three crates published to crates.io
- [ ] `docs.rs` builds successfully for all three crates
- [ ] CI green on Linux, macOS, Windows
- [ ] `CHANGELOG.md` entry for `[1.0.0]` written with full list of changes
- [ ] `README.md` reflects stable API — no "WIP" or "TODO" in public-facing docs
- [ ] Consumed by `awald` desktop app v1.0.0 (the real integration test)

---

## Post-1.0 Backlog

### Performance
- [ ] GPU-accelerated DataFrame operations via CUDA Polars feature
- [ ] Parallel block execution for independent script blocks (opt-in)
- [ ] Zero-copy IPC between Rust and Python via Arrow flight

### API extensions
- [ ] `awald-engine`: WASM target for browser-based execution (Pyodide backend)
- [ ] `awald-data`: streaming write support (append rows to loaded frame)
- [ ] `awald-data`: DuckDB integration for out-of-core queries

### Ecosystem
- [ ] Publish `awald-session` as the primary integration point for third-party apps
  - [ ] Any statistical desktop app can use `awald-session` as a drop-in Python executor + data layer
- [ ] Python bindings: `awald-py` — expose `Session`, `DataStore` to Python directly

---

## Deferred (explicitly out of scope for v1.0.0)

- [-] WASM/browser target — deferred, requires Pyodide instead of pyo3
- [-] Windows ARM native — deferred, pyo3 cross-compilation is complex
- [-] GPU memory management — deferred to post-1.0
- [-] Multi-process execution (separate Python process per session) — deferred
