use polars::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{Error, Result};

/// A request for a contiguous row range. `end` is exclusive.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SliceRequest {
    pub start: usize,
    pub end:   usize,
}

/// A single row as an ordered vec of JSON-safe cell values.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RowData {
    pub index:  usize,
    pub cells:  Vec<CellValue>,
}

/// A cell value, JSON-safe. Null maps to `CellValue::Null`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CellValue {
    Null,
    Int(i64),
    Float(f64),
    Text(String),
    Bool(bool),
}

pub fn slice_frame(frame: &DataFrame, req: SliceRequest) -> Result<Vec<RowData>> {
    let nrows = frame.height();
    let start = req.start.min(nrows);
    let end   = req.end.min(nrows);

    if start > end {
        return Err(Error::SliceOutOfBounds { start, end, nrows });
    }

    let len   = end - start;
    let slice = frame.slice(start as i64, len);
    let ncols = slice.width();
    let mut rows = Vec::with_capacity(len);

    for row_idx in 0..slice.height() {
        let mut cells = Vec::with_capacity(ncols);
        for col in slice.columns() {
            let val: AnyValue = col.get(row_idx).map_err(Error::Polars)?;
            cells.push(anyvalue_to_cell(val));
        }
        rows.push(RowData { index: start + row_idx, cells });
    }
    Ok(rows)
}

fn anyvalue_to_cell(v: AnyValue<'_>) -> CellValue {
    match v {
        AnyValue::Null                   => CellValue::Null,
        AnyValue::Boolean(b)             => CellValue::Bool(b),
        AnyValue::Int8(i)                => CellValue::Int(i as i64),
        AnyValue::Int16(i)               => CellValue::Int(i as i64),
        AnyValue::Int32(i)               => CellValue::Int(i as i64),
        AnyValue::Int64(i)               => CellValue::Int(i),
        AnyValue::UInt8(i)               => CellValue::Int(i as i64),
        AnyValue::UInt16(i)              => CellValue::Int(i as i64),
        AnyValue::UInt32(i)              => CellValue::Int(i as i64),
        AnyValue::UInt64(i)              => CellValue::Int(i as i64),
        AnyValue::Float32(f)             => CellValue::Float(f as f64),
        AnyValue::Float64(f)             => CellValue::Float(f),
        AnyValue::String(s)              => CellValue::Text(s.to_string()),
        AnyValue::StringOwned(s)         => CellValue::Text(s.to_string()),
        AnyValue::Categorical(idx, _rev) => {
            // Convert categorical to string representation
            CellValue::Text(format!("cat_{}", idx))
        },
        other                            => CellValue::Text(format!("{other}")),
    }
}
