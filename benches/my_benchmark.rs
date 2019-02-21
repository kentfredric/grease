extern crate criterion;
extern crate grease;

use criterion::{BatchSize, Criterion};
use grease::{repository::Repository, version};
use std::{path::Path, time::Duration};

fn criterion_benchmark(c: &mut Criterion) {
    let p = Path::new("/usr/local/gentoo");

    c.bench_function("repo.categories", move |b| {
        b.iter_batched(|| Repository::new(&p), |x| x.categories(), BatchSize::NumIterations(100))
    });

    c.bench_function("repo.categories.count", move |b| {
        b.iter_batched(|| Repository::new(&p).categories().unwrap(), |x| x.count(), BatchSize::NumIterations(100))
    });

    c.bench_function("repo.packages", move |b| {
        b.iter_batched(|| Repository::new(&p), |x| x.packages(), BatchSize::NumIterations(100))
    });

    c.bench_function("repo.get_category(dev-perl)", move |b| {
        b.iter_batched(|| Repository::new(&p), |x| x.get_category("dev-perl"), BatchSize::NumIterations(100))
    });

    c.bench_function("repo.get_category(dev-perl).packages.count", move |b| {
        b.iter_batched(
            || Repository::new(&p).get_category("dev-perl").unwrap(),
            |x| x.packages().unwrap().count(),
            BatchSize::NumIterations(100),
        )
    });

    c.bench_function_over_inputs(
        "version::parse",
        |b, &inp| {
            b.iter_batched(|| inp, |data| version::parse(data), BatchSize::NumIterations(10_000));
        },
        &["1", "1-r1", "10", "10-r1", "1234", "12345-r1"],
    );
    c.bench_function("repo.new", move |b| b.iter(|| Repository::new(&p)));
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
