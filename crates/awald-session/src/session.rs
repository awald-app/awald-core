use std::collections::HashMap;
use std::path::Path;

use awald_data::{DataStore, DatasetMeta};
use awald_engine::{ExecutionResult, Executor};

use crate::{Error, Result};

/// A single user session: one loaded dataset + one Python executor.
pub struct Session {
    pub store:    Option<DataStore>,
    pub executor: Executor,
    /// Maps Python variable names → DataStore handles for frames
    /// created or loaded during this session.
    pub frames:   HashMap<String, DataStore>,
}

impl Session {
    pub fn new() -> Result<Self> {
        Ok(Self {
            store:    None,
            executor: Executor::new()?,
            frames:   HashMap::new(),
        })
    }

    /// Load a dataset and make it available as `df` in the Python namespace.
    pub async fn load(&mut self, path: impl AsRef<Path>) -> Result<DatasetMeta> {
        let store = DataStore::load(path).await?;
        let meta  = store.meta();
        self.store = Some(store);
        // TODO: inject polars DataFrame into Python namespace via pyo3
        Ok(meta)
    }

    /// Execute a Python script, capturing output.
    pub async fn execute(&self, script: &str) -> Result<ExecutionResult> {
        Ok(self.executor.run(script).await?)
    }

    /// Reset session: clear namespace and drop loaded data.
    pub fn reset(&mut self) -> Result<()> {
        self.store = None;
        self.frames.clear();
        self.executor.reset()?;
        Ok(())
    }
}
