#![allow(dead_code)]
#![allow(unused)]

use std::{
    collections::{hash_map, HashMap, HashSet},
    hash::Hash,
};

mod valve;
use valve::*;

mod parse;
use parse::*;

mod solver;
use solver::*;

fn main() {
    println!("Hello, world!");
    let input_example1 = std::fs::read_to_string("../inputs/day16/example1.txt").unwrap();
    // let input_me = std::fs::read_to_string("../inputs/day16/me.txt").unwrap();

    let mut valves = hash_map::HashMap::<String, Valve>::new();
    parse_valves(input_example1, &mut valves);

    // let res = part1(&valves, 30);
    // println!("res base: {:?}", res);

    let res = part1_mask(&valves, 30);
    println!("res mask: {:?}", res);
}
