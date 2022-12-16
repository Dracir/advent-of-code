#![allow(dead_code)]
#![allow(unused)]

mod valve;
use valve::*;

mod parse;
use parse::*;

fn main() {
    println!("Hello, world!");
    let input_example1 = std::fs::read_to_string("../inputs/day16/example1.txt").unwrap();
    let input_me = std::fs::read_to_string("../inputs/day16/me.txt").unwrap();

    let valves = parse_valves(input_example1);
    part1(valves);
}


fn part1(input_example1: Vec<Valve>) {}
