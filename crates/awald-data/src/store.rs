use std::path::Path;
use std::sync::Arc;

use polars::prelude::*;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::{
    formats,
    slice::{RowData, SliceRequest},
    Result,
};

/// Metadata returned to the frontend after a dataset is loaded.
/// Never contains raw data — only schema and shape information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetMeta {
    pub name: String,
    pub path: String,
    pub nrows: usize,
    pub ncols: usize,
    pub schema: Vec<SchemaField>,
}

/// A single column's name and Polars dtype as a display string.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaField {
    pub name: String,
    pub dtype: String,
}

/// Thread-safe handle to a loaded Polars `DataFrame`.
///
/// Holds the frame behind `Arc<RwLock<>>` so multiple async tasks can
/// read viewport slices concurrently while a single writer can reload.
pub struct DataStore {
    frame: Arc<RwLock<DataFrame>>,
    meta: DatasetMeta,
}

impl DataStore {
    /// Load a dataset from any supported format, inferred from file extension.
    pub async fn load(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let frame = formats::read(path).await?;
        let meta = DatasetMeta {
            name: path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned(),
            path: path.to_string_lossy().into_owned(),
            nrows: frame.height(),
            ncols: frame.width(),
            schema: frame
                .schema()
                .iter()
                .map(|(name, dtype)| SchemaField {
                    name: name.to_string(),
                    dtype: format!("{dtype}"),
                })
                .collect(),
        };
        Ok(Self {
            frame: Arc::new(RwLock::new(frame)),
            meta,
        })
    }

    /// Convenience constructor for CSV files.
    pub async fn from_csv(path: impl AsRef<Path>) -> Result<Self> {
        Self::load(path).await
    }

    /// Returns a clone of the dataset metadata (cheap — no DataFrame copy).
    pub fn meta(&self) -> DatasetMeta {
        self.meta.clone()
    }

    /// Returns a viewport slice as serialisable row data.
    ///
    /// Only the requested rows are materialised. The full DataFrame stays
    /// in Rust memory behind the `RwLock`.
    pub async fn slice(&self, req: SliceRequest) -> Result<Vec<RowData>> {
        let frame = self.frame.read().await;
        crate::slice::slice_frame(&frame, req)
    }

    /// Returns a shared handle to the inner frame.
    /// Used by `awald-session` to expose frames to the Python namespace.
    pub fn frame_handle(&self) -> Arc<RwLock<DataFrame>> {
        Arc::clone(&self.frame)
    }
}
