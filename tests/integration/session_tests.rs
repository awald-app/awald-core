/// Integration test: load a CSV, slice rows, execute Python.
/// Requires Python 3.12+ with pyfixest and polars installed.
#[cfg(test)]
mod tests {
    use std::io::Write;

    use awald_session::Session;
    use tempfile::NamedTempFile;

    fn sample_csv() -> NamedTempFile {
        let mut f = NamedTempFile::with_suffix(".csv").unwrap();
        writeln!(f, "wage,hours,tenure,industry").unwrap();
        for i in 0..100 {
            writeln!(
                f,
                "{},{},{},{}",
                10.0 + i as f64 * 0.5,
                40 + (i % 8),
                i % 10,
                i % 4
            )
            .unwrap();
        }
        f
    }

    #[tokio::test]
    async fn test_load_csv_returns_meta() {
        let csv  = sample_csv();
        let mut session = Session::new().expect("session init");
        let meta = session.load(csv.path()).await.expect("load");
        assert_eq!(meta.nrows, 100);
        assert_eq!(meta.ncols, 4);
        assert!(meta.schema.iter().any(|f| f.name == "wage"));
    }

    #[tokio::test]
    async fn test_execute_simple_python() {
        let session = Session::new().expect("session init");
        let result  = session.execute("print('awald-engine ok')").await.expect("exec");
        assert_eq!(result.error, None);
        assert!(result.stdout.contains("awald-engine ok"));
    }

    #[tokio::test]
    async fn test_execute_python_error_captured() {
        let session = Session::new().expect("session init");
        let result  = session.execute("raise ValueError('test error')").await.expect("exec");
        assert!(result.error.is_some());
        let err = result.error.unwrap();
        assert!(err.contains("ValueError") || err.contains("test error"));
    }

    #[tokio::test]
    async fn test_session_reset_clears_namespace() {
        let mut session = Session::new().expect("session init");
        session.execute("x = 42").await.expect("exec");
        session.reset().expect("reset");
        let result = session.execute("print(x)").await.expect("exec");
        assert!(result.error.is_some()); // x should be undefined
    }
}
