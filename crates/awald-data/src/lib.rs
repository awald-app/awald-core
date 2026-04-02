//! # awald-data
//!
//! Polars-backed dataset store for Awald.
//!
//! ## Design
//!
//! - One `DataStore` per loaded dataset, held behind `Arc<RwLock<>>`.
//! - The full `DataFrame` never leaves Rust memory.
//! - Callers request viewport slices via `DataStore::slice()`.
//! - Serialisation to JSON happens at the slice boundary only.
//!
//! ## Example
//!
//! ```rust,no_run
//! use awald_data::{DataStore, SliceRequest};
//!
//! # tokio_test::block_on(async {
//! let store = DataStore::from_csv("data.csv").await?;
//! let meta  = store.meta();
//! println!("{} rows × {} cols", meta.nrows, meta.ncols);
//!
//! let rows = store.slice(SliceRequest { start: 0, end: 50 }).await?;
//! # Ok::<(), awald_data::Error>(())
//! # });
//! ```

pub mod error;
pub mod formats;
pub mod slice;
pub mod store;
pub mod summary;

pub use error::Error;
pub use slice::{RowData, SliceRequest};
pub use store::{DataStore, DatasetMeta, SchemaField};
pub use summary::VarSummary;

pub type Result<T> = std::result::Result<T, Error>;
