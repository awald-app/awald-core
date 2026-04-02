//! # awald-session
//!
//! Orchestrates `awald-data` and `awald-engine` into a unified session.
//!
//! ## Example
//!
//! ```rust,no_run
//! use awald_session::Session;
//!
//! # tokio_test::block_on(async {
//! let mut session = Session::new()?;
//!
//! let meta = session.load("wage_data.csv").await?;
//! println!("Loaded: {} rows", meta.nrows);
//!
//! let result = session.execute(r#"
//! import pyfixest as pf
//! fit = pf.feols("wage ~ hours + tenure | industry", data=df)
//! print(fit.summary())
//! "#).await?;
//!
//! println!("{}", result.stdout);
//! # Ok::<(), awald_session::Error>(())
//! # });
//! ```

pub mod error;
pub mod session;

pub use error::Error;
pub use session::Session;

pub type Result<T> = std::result::Result<T, Error>;
