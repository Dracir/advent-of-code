use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day15rs::sensor::*;
use day15rs::solving::*;

#[allow(dead_code)]
fn test_base(sensors: &Vec<Sensor>) {
    for x in 0..=4000000 {
        if !is_covered(sensors, x as i64, 0) {
            return;
        }
    }
}

#[allow(dead_code)]
fn test_cached(sensors: &Vec<Sensor>) {
    for x in 0..=4000000 {
        if !is_covered_with_cached(sensors, x as i64, 0) {
            return;
        }
    }
}

#[allow(dead_code)]
fn test_jumpy(sensors: &Vec<Sensor>) {
    let x = 0;
    let mut y = 0;
    loop {
        if y >= 4000000 {
            break;
        }
        let covered_option = is_covered_by(sensors, x as i64, y as i64);
        // println!("x: {}, y: {}, covered_option: {:?}", x, y, covered_option);
        match covered_option {
            Some(sensor) => {
                let y_distance = sensor.s_y - y as i64;
                if y_distance > 0 {
                    // println!("y += {}", y_distance);
                    y += 2 * y_distance as usize;
                } else {
                    // println!("y++");
                    y += 1;
                }
            }
            _ => {
                return;
            }
        }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let input = std::fs::read_to_string("../inputs/day15/me.txt").unwrap();
    let sensors: Vec<Sensor> = input.lines().map(|line| parse_sensor(line)).collect();

    let mut part1 = c.benchmark_group("1 line");
    part1.bench_function("base", |b| b.iter(|| test_base(black_box(&sensors))));
    part1.bench_function("cached", |b| b.iter(|| test_cached(black_box(&sensors))));
    part1.bench_function("jumpy", |b| b.iter(|| test_jumpy(black_box(&sensors))));
    part1.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
