use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;

use file_walkers::nonparallel;
use file_walkers::parallel;

fn nonparallel_case() {
    let mut current_dir = std::env::current_dir().unwrap();
    current_dir.push("src/bench/benchmark");

    if let Some(dir) = current_dir.to_str() {
        nonparallel::get_directories(dir);
    } else {
        println!("Error!");
    }
}

fn parallel_case() {
    let mut current_dir = std::env::current_dir().unwrap();
    current_dir.push("src/bench/benchmark");

    if let Some(dir) = current_dir.to_str() {
        parallel::get_directories(dir, false);
    } else {
        println!("Error!");
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("nonparallel", |b| b.iter(|| nonparallel_case()));
    c.bench_function("parallel", |b| b.iter(|| parallel_case()));
}

criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = criterion_benchmark
}
criterion_main!(benches);
