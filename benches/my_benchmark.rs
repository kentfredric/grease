extern crate criterion;
extern crate grease;

use criterion::{Criterion, BatchSize};
use grease::version;
use std::ffi::OsString;
use std::time::Duration;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "new",
        |b, &inp| {
            b.iter_batched(
                || inp,
                |data| version::parse(data),
                BatchSize::NumIterations(10_000),
            );
        },
        &["1", "1-r1", "10", "10-r1", "1234", "12345-r1"],
    );
}


fn main() {
    let mut c = Criterion::default()
        .sample_size(500)
        .warm_up_time(Duration::from_millis(3000))
        .measurement_time(Duration::new(10, 0))
        .noise_threshold(0.1)
        .confidence_level(0.95)
        .configure_from_args();
    criterion_benchmark(&mut c);
    c.final_summary();
}
