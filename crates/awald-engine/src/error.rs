use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Python error: {0}")]
    Python(#[from] pyo3::PyErr),

    #[error("Executor task panicked: {0}")]
    TaskPanic(String),

    #[error("Invalid script: {0}")]
    InvalidScript(String),
}
