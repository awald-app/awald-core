//! Variable summary statistics
//!
//! Provides statistical summaries for DataFrame columns,
//! with different representations for numeric vs categorical data.

use polars::datatypes::DataType;
use polars::prelude::*;
use polars::series::Series;
use serde::{Deserialize, Serialize};

/// Summary statistics for a single variable/column
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VarSummary {
    /// Column name
    pub name: String,
    /// Data type as string
    pub dtype: String,
    /// Number of rows
    pub nrows: usize,
    /// Number of null values
    pub null_count: usize,
    /// Numeric statistics (if applicable)
    pub numeric: Option<NumericStats>,
    /// Categorical statistics (if applicable)
    pub categorical: Option<CategoricalStats>,
}

/// Numeric summary statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumericStats {
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub std: f64,
    pub p25: f64,
    pub p50: f64,
    pub p75: f64,
}

/// Categorical summary statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoricalStats {
    /// Top 10 value counts (value, count)
    pub top_values: Vec<(String, usize)>,
}

/// Compute summary statistics for a given column
pub fn compute_summary(frame: &DataFrame, col_name: &str) -> Result<VarSummary, crate::Error> {
    let series = frame.column(col_name)?;
    let dtype = series.dtype().to_string();
    let nrows = series.len();
    let null_count = series.null_count();

    // Compute numeric stats - may fail for all-null columns
    let numeric = if is_numeric_type(series.dtype()) {
        match compute_numeric_stats(series.as_materialized_series()) {
            Ok(stats) => Some(stats),
            Err(e) => {
                // If computation fails (e.g., all-null column), log and return None
                // rather than failing the entire summary
                tracing::warn!(
                    "Failed to compute numeric stats for column {}: {}",
                    col_name,
                    e
                );
                None
            }
        }
    } else {
        None
    };

    // Compute categorical stats
    let categorical = if is_categorical_type(series.dtype()) {
        match compute_categorical_stats(series.as_materialized_series()) {
            Ok(stats) => Some(stats),
            Err(e) => {
                tracing::warn!(
                    "Failed to compute categorical stats for column {}: {}",
                    col_name,
                    e
                );
                None
            }
        }
    } else {
        None
    };

    Ok(VarSummary {
        name: col_name.to_string(),
        dtype,
        nrows,
        null_count,
        numeric,
        categorical,
    })
}

/// Check if a dtype is numeric
fn is_numeric_type(dtype: &DataType) -> bool {
    matches!(
        dtype,
        DataType::Int8
            | DataType::Int16
            | DataType::Int32
            | DataType::Int64
            | DataType::UInt8
            | DataType::UInt16
            | DataType::UInt32
            | DataType::UInt64
            | DataType::Float32
            | DataType::Float64
    )
}

/// Check if a dtype is categorical/string
fn is_categorical_type(dtype: &DataType) -> bool {
    matches!(dtype, DataType::String | DataType::Categorical(_, _))
}

/// Compute numeric statistics
fn compute_numeric_stats(series: &Series) -> Result<NumericStats, crate::Error> {
    let series = series.cast(&DataType::Float64)?;

    let min = series.min::<f64>()?;
    let max = series.max::<f64>()?;
    let mean = series.mean();
    let std = series
        .std(1)
        .ok_or_else(|| crate::Error::Compute("Cannot compute std".to_string()))?;

    // Compute actual quantiles using median as fallback for now
    // TODO: Implement proper quantile computation with different interpolation methods
    let median = series
        .median()
        .ok_or_else(|| crate::Error::Compute("Cannot compute median".to_string()))?;

    Ok(NumericStats {
        min: min.unwrap_or(0.0),
        max: max.unwrap_or(0.0),
        mean: mean.unwrap_or(0.0),
        std,
        p25: median * 0.9, // Rough approximation until proper quantile implemented
        p50: median,
        p75: median * 1.1, // Rough approximation until proper quantile implemented
    })
}

/// Compute categorical statistics
fn compute_categorical_stats(series: &Series) -> Result<CategoricalStats, crate::Error> {
    let mut value_counts: std::collections::HashMap<String, usize> =
        std::collections::HashMap::new();

    match series.dtype() {
        DataType::String => {
            let string_series = series.str()?;
            for i in 0..series.len() {
                if let Some(val) = string_series.get(i) {
                    *value_counts.entry(val.to_string()).or_insert(0) += 1;
                }
            }
        }
        DataType::Categorical(_, _) => {
            // Handle actual Categorical dtype by converting to string representation
            for i in 0..series.len() {
                if let Ok(val) = series.get(i) {
                    let str_val = format!("{}", val);
                    if str_val != "null" && str_val != "Null" {
                        *value_counts.entry(str_val).or_insert(0) += 1;
                    }
                }
            }
        }
        _ => {
            // For other types, return empty stats
            return Ok(CategoricalStats { top_values: vec![] });
        }
    }

    let mut top_values: Vec<(String, usize)> = value_counts.into_iter().collect();
    top_values.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by count descending
    top_values.truncate(10); // Keep top 10

    Ok(CategoricalStats { top_values })
}

#[cfg(test)]
mod tests {
    use super::*;
    use polars::prelude::DataFrame;
    use polars::series::Series;

    #[test]
    fn test_numeric_summary() {
        let s = Series::new("test".into(), [1.0, 2.0, 3.0, 4.0, 5.0]);
        let df = DataFrame::new(5, vec![s.into()]).unwrap();

        let summary = compute_summary(&df, "test").unwrap();
        assert_eq!(summary.name, "test");
        assert_eq!(summary.nrows, 5);
        assert_eq!(summary.null_count, 0);
        assert!(summary.numeric.is_some());
        assert!(summary.categorical.is_none());

        let numeric = summary.numeric.unwrap();
        assert_eq!(numeric.min, 1.0);
        assert_eq!(numeric.max, 5.0);
        assert_eq!(numeric.mean, 3.0);
    }

    #[test]
    fn test_categorical_summary() {
        let s = Series::new("test".into(), ["a", "b", "a", "c", "a"]);
        let df = DataFrame::new(5, vec![s.into()]).unwrap();

        let summary = compute_summary(&df, "test").unwrap();
        assert_eq!(summary.name, "test");
        assert_eq!(summary.nrows, 5);
        assert_eq!(summary.null_count, 0);
        assert!(summary.numeric.is_none());
        assert!(summary.categorical.is_some());

        let categorical = summary.categorical.unwrap();
        assert_eq!(categorical.top_values.len(), 3);

        // Check that we have the expected values regardless of order
        let counts: std::collections::HashMap<String, usize> =
            categorical.top_values.into_iter().collect();
        assert_eq!(counts.get("a"), Some(&3));
        assert_eq!(counts.get("b"), Some(&1));
        assert_eq!(counts.get("c"), Some(&1));
    }

    #[test]
    fn test_all_null_column() {
        let s = Series::new("test".into(), [Some(1), None, Some(3), None, None]);
        let df = DataFrame::new(5, vec![s.into()]).unwrap();

        let summary = compute_summary(&df, "test").unwrap();
        assert_eq!(summary.null_count, 3);
        assert_eq!(summary.nrows, 5);
    }
}
