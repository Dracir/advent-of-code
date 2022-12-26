use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day15rs::sensor::*;
use day15rs::solving::*;

#[allow(dead_code)]
fn test_base(sensors: &Vec<Sensor>, rows: usize) {
    for x in 0..rows {
        for y in 0..=4000000 {
            if !is_covered(sensors, x as i64, y) {
                return;
            }
        }
    }
}

#[allow(dead_code)]
fn test_cached(sensors: &Vec<Sensor>, rows: usize) {
    for x in 0..rows {
        for y in 0..=4000000 {
            if !is_covered_with_cached(sensors, x as i64, y) {
                return;
            }
        }
    }
}

#[allow(dead_code)]
fn test_jumpy(sensors: &Vec<Sensor>, rows: usize) {
    for x in 0..rows {
        find_empty_jumpy_at_x(sensors, rows, x as i64);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let input = std::fs::read_to_string("../inputs/day15/me.txt").unwrap();
    let sensors: Vec<Sensor> = input.lines().map(|line| parse_sensor(line)).collect();

    // let rows = 5;
    // let mut part1 = c.benchmark_group(format!("{} lines", rows));
    // part1.bench_function("base", |b| b.iter(|| test_base(black_box(&sensors), rows)));
    // part1.bench_function("cached", |b| b.iter(|| test_cached(black_box(&sensors), rows)));
    // part1.bench_function("jumpy", |b| b.iter(|| test_jumpy(black_box(&sensors), rows)));
    // part1.finish();

    let rows = 20000;
    let cols = 1000;
    let mut group_sensors = c.benchmark_group("Regrouping Sensors");
    // group_sensors.bench_function("base jumpy", |b| {
    //     b.iter(|| part2_jumpy(black_box(&sensors), black_box(rows), black_box(cols)))
    // });
    group_sensors.bench_function("with grouping", |b| {
        b.iter(|| {
            part2_jumpy_grouped(
                black_box(&sensors),
                black_box(rows),
                black_box(cols),
                black_box(rows / 100),
                false,
            )
        })
    });
    group_sensors.bench_function("with grouping check more!", |b| {
        b.iter(|| {
            part2_jumpy_grouped_check_more(
                black_box(&sensors),
                black_box(rows),
                black_box(cols),
                black_box(rows / 100),
                false,
            )
        })
    });
    group_sensors.bench_function("with grouping check more with THREADS!", |b| {
        b.iter(|| {
            part2_jumpy_grouped_check_more(
                black_box(&sensors),
                black_box(rows),
                black_box(cols),
                black_box(rows / 100),
                false,
            )
        })
    });
    group_sensors.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
