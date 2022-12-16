//import sensor from "./sensor.rs";

mod sensor;
use sensor::*;
mod solving;
use solving::*;

fn main() {
    println!("Hello, world!");
    let input = std::fs::read_to_string("../inputs/day15/example1.txt").unwrap();
    // let input = std::fs::read_to_string("../inputs/day15/me.txt").unwrap();
    let sensors: Vec<Sensor> = input.lines().map(|line| parse_sensor(line)).collect();

    // let res = part1(&sensors, 2000000);
    // println!("res: {}", res);

    let response_2 = part2_jumpy(&sensors, 20);
    // let response_2 = part2_jumpy(&sensors, 4000000);
    println!("response_2: {}", response_2);
    // 2708544677134 too low
}
