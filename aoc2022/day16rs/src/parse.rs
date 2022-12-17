use std::{collections::HashMap, str::FromStr};

use crate::valve::*;

pub fn parse_valves(input_example1: String, valves: &mut HashMap<String, Valve>) {
    for line in input_example1.lines() {
        // line example
        // Valve GT has flow rate=0; tunnels lead to valves FJ, AW

        // let mut valve = Valve::new();
        let mut tokens = line.split_whitespace();
        tokens.next(); // valves
        let name = tokens.next().unwrap().to_string();
        tokens.next(); // has
        tokens.next(); // flow
        let mut flow_tokens = tokens.next().unwrap().split("="); // rate=0;
        let flow_rate = flow_tokens
            .nth(1)
            .unwrap()
            .trim_end_matches(";")
            .parse::<i64>()
            .unwrap();
        tokens.next(); // ;
        tokens.next(); // tunnels
        tokens.next(); // lead
        tokens.next(); // to
        tokens.next(); // valves
        let mut tunnels = Vec::<String>::new();
        for token in tokens {
            tunnels.push(token.trim_end_matches(",").to_string());
        }
        let tunnels = tunnels;

        let valve = Valve {
            name,
            flow_rate,
            tunnels,
        };
        // println!("name : {:?}", valve.name.to_owned());
        valves.insert(valve.name.to_owned(), valve);
    }
}

pub fn string_to_bit_index(valves_string: &HashMap<String, Valve>, string: &str) -> usize {
    let mut names = valves_string.keys().collect::<Vec<&String>>();
    names.sort();
    let index = names.iter().position(|&r| r == &string).unwrap();
    return index as usize;
}

pub fn from_hash_string_to_hash_bit(
    valves_string: &HashMap<String, Valve>,
    valves_bit: &mut HashMap<usize, ValveBit>,
    aa_index: &mut usize,
) {
    let mut names = valves_string.keys().collect::<Vec<&String>>();
    names.sort();

    for name in names.iter() {
        let mut tunnels_bit = Vec::<usize>::new();
        for tunnel in valves_string.to_owned().get(*name).unwrap().tunnels.iter() {
            // println!(
            //     "tunnel : {:?} is bit : {}",
            //     tunnel,
            //     string_to_bit_index(valves_string, tunnel)
            // );
            tunnels_bit.push(string_to_bit_index(valves_string, tunnel));
        }
        let bit = string_to_bit_index(valves_string, *name);
        // println!("for {} , bit : {:?}", name, bit);

        let valve_bit = ValveBit {
            name: String::from_str(*name).unwrap(),
            id: bit,
            flow_rate: valves_string.get(*name).unwrap().flow_rate,
            tunnels: tunnels_bit,
        };
        valves_bit.insert(bit, valve_bit);
    }
    *aa_index = string_to_bit_index(valves_string, "AA");
}
