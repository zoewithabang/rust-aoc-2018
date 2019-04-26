use criterion::*;
use aoc_2018::days::day1::Day1;
use aoc_2018::days::day2::Day2;
use aoc_2018::days::Day;

fn criterion_benchmark(c: &mut Criterion) {
    let day1 = Day1::new();
    let day2 = Day2::new();

    c.bench_function("day1", move |b| b.iter(|| day1.run()));
    c.bench_function("day2", move |b| b.iter(|| day2.run()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
