use std::time::{Duration, Instant};

use parking_lot::Mutex;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use serde::{Deserialize, Serialize};

use crate::{Error, Result, capture};

/// Output from a single execution block.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    /// Captured stdout from the Python block.
    pub stdout:      String,
    /// Captured stderr (warnings, tracebacks).
    pub stderr:      String,
    /// Python exception message if execution failed, None on success.
    pub error:       Option<String>,
    /// Wall-clock duration of the Python call.
    pub duration_ms: u64,
}

/// Single-session Python executor.
///
/// Holds a persistent `PyDict` globals namespace so variables defined in
/// one `run()` call are available in subsequent calls — identical to how
/// a Stata `.do` file maintains state across commands.
pub struct Executor {
    /// Serialises all Python calls. One execution at a time — no exceptions.
    _lock:    Mutex<()>,
    /// Persistent Python globals. Wrapped in Mutex so `run` takes &self.
    globals:  Mutex<Py<PyDict>>,
}

impl Executor {
    /// Create a new executor with a fresh Python namespace.
    pub fn new() -> Result<Self> {
        let globals = Python::with_gil(|py| {
            let d = PyDict::new(py);
            // Pre-import builtins so `print`, `len`, etc. work out of the box
            d.set_item("__builtins__", py.import("builtins")?)?;
            Ok::<_, PyErr>(d.unbind())
        })?;

        Ok(Self {
            _lock:   Mutex::new(()),
            globals: Mutex::new(globals),
        })
    }

    /// Execute a Python script string. Captures stdout/stderr.
    /// Returns `ExecutionResult` whether or not the script raised an exception.
    pub async fn run(&self, script: &str) -> Result<ExecutionResult> {
        let script = script.to_owned();
        let globals_ref = self.globals.lock();

        tokio::task::spawn_blocking(move || {
            Python::with_gil(|py| {
                // Re-bind globals into this GIL context
                let globals = globals_ref.bind(py);

                // Install stdout/stderr capture
                let (stdout_cap, stderr_cap) = capture::install(py)?;

                let start = Instant::now();
                let error = py
                    .run(pyo3::ffi::c_str!(script), Some(globals), None)
                    .err()
                    .map(|e| e.to_string());
                let duration_ms = start.elapsed().as_millis() as u64;

                let stdout = capture::drain(&stdout_cap, py)?;
                let stderr = capture::drain(&stderr_cap, py)?;
                capture::restore(py)?;

                Ok(ExecutionResult { stdout, stderr, error, duration_ms })
            })
        })
        .await
        .map_err(|e| Error::TaskPanic(e.to_string()))?
    }

    /// Reset the namespace — equivalent to restarting the session.
    pub fn reset(&self) -> Result<()> {
        let mut globals = self.globals.lock();
        Python::with_gil(|py| {
            let d = PyDict::new(py);
            d.set_item("__builtins__", py.import("builtins")?)?;
            *globals = d.unbind();
            Ok(())
        })?;
        Ok(())
    }
}
