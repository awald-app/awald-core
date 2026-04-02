use std::path::Path;

use polars::prelude::*;

use crate::{Error, Result};

/// Read any supported format, inferred from extension.
pub async fn read(path: &Path) -> Result<DataFrame> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    tokio::task::spawn_blocking({
        let path = path.to_path_buf();
        move || match ext.as_str() {
            "csv" | "tsv" | "txt" => read_csv(&path),
            "parquet"             => read_parquet(&path),
            "dta"                 => read_stata(&path),
            "xlsx" | "xls"        => read_excel(&path),
            "ipc" | "arrow"       => read_ipc(&path),
            other => Err(Error::UnsupportedFormat(other.to_string())),
        }
    })
    .await
    .map_err(|e| Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?
}

fn read_csv(path: &Path) -> Result<DataFrame> {
    Ok(CsvReadOptions::default()
        .with_infer_schema_length(Some(100))
        .with_has_header(true)
        .try_into_reader_with_file_path(Some(path.to_path_buf()))?
        .finish()?)
}

fn read_parquet(path: &Path) -> Result<DataFrame> {
    let file = std::fs::File::open(path)?;
    Ok(ParquetReader::new(file).finish()?)
}

fn read_stata(path: &Path) -> Result<DataFrame> {
    let file = std::fs::File::open(path)?;
    // polars >= 1.0 supports Stata .dta natively
    Ok(StataReader::new(file).finish()?)
}

fn read_excel(path: &Path) -> Result<DataFrame> {
    // Requires polars "xlsx2csv" feature or calamine
    // Placeholder — implementation tracked in TODO
    Err(Error::UnsupportedFormat(
        format!("Excel support pending: {}", path.display())
    ))
}

fn read_ipc(path: &Path) -> Result<DataFrame> {
    let file = std::fs::File::open(path)?;
    Ok(IpcReader::new(file).finish()?)
}
