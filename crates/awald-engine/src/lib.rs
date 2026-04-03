//! # awald-engine
//!
//! pyo3-based Python executor for Awald.
//!
//! ## Execution model
//!
//! - One `Executor` per session, holding a persistent Python namespace.
//! - A `parking_lot::Mutex` serialises all Python calls — one at a time.
//! - Execution is always linear: blocks run in submission order, halt on error.
//! - stdout / stderr are captured per-block and returned as structured output.
//!
//! ## Example
//!
//! ```rust,no_run
//! use awald_engine::Executor;
//!
//! # tokio_test::block_on(async {
//! let mut exec = Executor::new()?;
//!
//! let result = exec.run("import pyfixest as pf\nprint('ok')").await?;
//! assert_eq!(result.stdout.trim(), "ok");
//! # Ok::<(), awald_engine::Error>(())
//! # });
//! ```

pub mod capture;
pub mod error;
pub mod executor;

pub use error::Error;
pub use executor::{ExecutionResult, Executor};

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_null_bytes_in_script_produce_error() {
        let exec = Executor::new().unwrap();

        // Script with embedded null byte should return InvalidScript error
        let script = "print('hello\x00world')";
        let result = exec.run(script).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            Error::InvalidScript(msg) => {
                assert!(msg.contains("null bytes"));
            }
            other => panic!("Expected InvalidScript error, got {:?}", other),
        }
    }

    #[tokio::test]
    async fn test_valid_script_runs_successfully() {
        let exec = Executor::new().unwrap();

        let result = exec.run("print('hello')").await.unwrap();
        assert!(result.error.is_none());
        assert!(result.stdout.contains("hello"));
    }
}
