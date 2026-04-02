use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn bench_slice(c: &mut Criterion) {
    use awald_data::{DataStore, SliceRequest};

    let rt = tokio::runtime::Runtime::new().unwrap();

    // Build an in-memory CSV for benchmarking
    let mut csv_content = String::from("a,b,c\n");
    for i in 0..100_000 {
        csv_content.push_str(&format!("{},{},{}\n", i, i as f64 * 1.5, i % 10));
    }
    let mut tmp = tempfile::NamedTempFile::with_suffix(".csv").unwrap();
    std::io::Write::write_all(&mut tmp, csv_content.as_bytes()).unwrap();

    let store = rt
        .block_on(DataStore::from_csv(tmp.path()))
        .expect("load");

    let mut group = c.benchmark_group("slice");
    for size in [50usize, 500, 5_000] {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            b.iter(|| {
                rt.block_on(store.slice(SliceRequest { start: 0, end: size }))
                    .unwrap()
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_slice);
criterion_main!(benches);
