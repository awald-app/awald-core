use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Polars error: {0}")]
    Polars(#[from] polars::error::PolarsError),

    #[error("Unsupported file format: {0}")]
    UnsupportedFormat(String),

    #[error("Slice out of bounds: requested {start}..{end}, frame has {nrows} rows")]
    SliceOutOfBounds { start: usize, end: usize, nrows: usize },

    #[error("Computation error: {0}")]
    Compute(String),
}
