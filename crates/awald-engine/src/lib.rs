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
