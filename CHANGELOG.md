# Changelog

Format: [Keep a Changelog 1.1.0](https://keepachangelog.com).
Versioning: [SemVer 2.0.0](https://semver.org).
All three crates (`awald-data`, `awald-engine`, `awald-session`) are
versioned together — a single release bumps all three.

---

## [Unreleased]

### Added
- `awald-data`: `DataStore`, `DatasetMeta`, `SliceRequest`, `RowData`
- `awald-data`: format dispatch for CSV, Parquet, Stata DTA, IPC
- `awald-engine`: `Executor` with serialised GIL (`parking_lot::Mutex`)
- `awald-engine`: stdout/stderr capture via `sys` redirection
- `awald-session`: `Session` orchestrating data + engine

### Known gaps (targeted for v0.1.0)
- Excel (`.xlsx`) reader pending — `polars` calamine feature
- DataFrame → Python namespace injection via pyo3 (`session.load`)
- Integration tests require live Python + pyfixest install

---

## [0.1.0] — TBD

First usable release. Consumed by `awald` desktop app v0.1.0.
