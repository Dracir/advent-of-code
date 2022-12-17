use std::collections::HashMap;
use std::collections::HashSet;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day16rs::parse::*;
use day16rs::solver::*;
use day16rs::valve::*;

fn criterion_benchmark(c: &mut Criterion) {
    let input_example1 = std::fs::read_to_string("../inputs/day16/example1.txt").unwrap();

    let mut valves = HashMap::<String, Valve>::new();
    parse_valves(input_example1, &mut valves);

    let itt = 6;
    let mut part1_tests = c.benchmark_group("1 line");
    part1_tests.bench_function("base", |b| b.iter(|| part1(black_box(&valves), black_box(itt))));
    part1_tests.bench_function("bit", |b| b.iter(|| part1_mask(black_box(&valves), black_box(itt))));
    part1_tests.measurement_time(std::time::Duration::from_secs(25));
    part1_tests.finish();

    let time_remaining = 7;
    let base_open_valves = HashSet::<String>::new();
    let base_current_valve = "AA";

    let bit_open_valves = 0 as u64;
    let mut bit_valves = HashMap::<usize, ValveBit>::new();
    let mut bit_current_valve = 0 as usize;
    from_hash_string_to_hash_bit(&valves, &mut bit_valves, &mut bit_current_valve);

    let mut part1_after_parse_tests = c.benchmark_group("After Parse");
    part1_after_parse_tests.bench_function("base", |b| {
        b.iter(|| {
            most_pressure(
                black_box(&valves),
                black_box(&base_open_valves),
                black_box(time_remaining),
                black_box(base_current_valve),
            )
        })
    });
    part1_after_parse_tests.bench_function("bit", |b| {
        b.iter(|| {
            most_pressure_mask(
                black_box(&bit_valves),
                black_box(bit_open_valves),
                black_box(time_remaining),
                black_box(bit_current_valve),
                black_box(0),
            )
        })
    });
    part1_after_parse_tests.measurement_time(std::time::Duration::from_secs(120));
    part1_after_parse_tests.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
