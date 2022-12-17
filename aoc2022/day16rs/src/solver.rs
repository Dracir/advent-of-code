use std::collections::HashMap;
use std::collections::HashSet;

use crate::parse::*;
use crate::valve::*;

pub fn part1(input_example1: &HashMap<String, Valve>, time_remaining: i64) -> (i64, u64) {
    let open_valves = HashSet::<String>::new();
    let current_valve = "AA";
    let max_flow = most_pressure(input_example1, &open_valves, time_remaining, current_valve);
    return max_flow;
}

pub fn most_pressure(
    valves: &HashMap<String, Valve>,
    open_valves: &HashSet<String>,
    time_remaining: i64,
    current_valve_id: &str,
) -> (i64, u64) {
    // if time_remaining >= 15 {
    //     println!("time_remaining >= 15 : {} {} ", current_valve, time_remaining);
    // }
    if time_remaining == 1 {
        if !open_valves.contains(current_valve_id) {
            return (valves.get(current_valve_id).unwrap().flow_rate, 1);
        }
        return (0, 0);
    } else if time_remaining <= 0 {
        // println!("time_remaining <= 0");
        return (0, 0);
    }
    if open_valves.len() == valves.len() {
        // println!("open_valves.len() == valves.len()");
        return (0, 0);
    }
    // println!("Visiting {} at {} ", current_valve_id, time_remaining);

    let mut max_flow = 0 as i64;
    let mut ticks = 0 as u64;
    //try with current valve
    let current_valve = valves.get(current_valve_id).unwrap();
    let flow_from_current = current_valve.flow_rate * time_remaining;
    for tunnel in current_valve.tunnels.iter() {
        if open_valves.contains(tunnel) {
            continue;
        } else {
            let mut new_valves = open_valves.clone();
            let pressure = most_pressure(valves, &new_valves, time_remaining - 1, tunnel);
            max_flow = max_flow.max(pressure.0);
            ticks += pressure.1;

            // try with current valve
            new_valves.insert(current_valve_id.to_owned());
            let pressure = most_pressure(valves, &new_valves, time_remaining - 2, tunnel);
            max_flow = max_flow.max(flow_from_current + pressure.0);
            ticks += pressure.1;
        }
    }

    (max_flow, ticks)
}

pub fn part1_mask(valves_strings: &HashMap<String, Valve>, time_remaining: i64) -> (i64, u64) {
    let mut current_valve = 0 as usize;
    let open_valves = 0 as u64;
    let mut valves = HashMap::<usize, ValveBit>::new();
    from_hash_string_to_hash_bit(valves_strings, &mut valves, &mut current_valve);

    let max_flow = most_pressure_mask(&valves, open_valves, time_remaining, current_valve, 0);
    max_flow
}
pub fn most_pressure_mask(
    valves: &HashMap<usize, ValveBit>,
    open_valves: u64,
    time_remaining: i64,
    current_valve_id: usize,
    opened_valves: usize,
) -> (i64, u64) {
    let bit = 1 << current_valve_id;
    if time_remaining == 1 {
        if open_valves & bit == 0 {
            // println!("time 1 {} {} ", current_valve_id, time_remaining);
            return (valves.get(&current_valve_id).unwrap().flow_rate, 1);
        }
        return (0, 0);
    } else if time_remaining <= 0 {
        // println!("time_remaining <= 0");
        return (0, 0);
    }
    if opened_valves == valves.len() {
        // println!("open_valves.len() == valves.len()");
        return (0, 0);
    }
    // println!("Visiting {} at {} ", current_valve_id, time_remaining);
    // println!("open_valves: {:b}", open_valves);

    let mut ticks = 0;
    let mut max_flow = 0 as i64;
    //try with current valve
    let current_valve = valves.get(&current_valve_id).unwrap();
    let flow_from_current = current_valve.flow_rate * (time_remaining - 1);
    for tunnel in current_valve.tunnels.iter() {
        // println!("\topen_valves: {} ", open_valves);
        // println!("\tbit: {:b}", bit);
        if open_valves & bit > 0 {
            continue;
        } else {
            let mut new_valves = open_valves.clone();
            let res = most_pressure_mask(valves, new_valves, time_remaining - 1, *tunnel, opened_valves);
            max_flow = max_flow.max(res.0);
            ticks += res.1;

            // try with current valve
            new_valves |= bit;
            let res = most_pressure_mask(valves, new_valves, time_remaining - 2, *tunnel, opened_valves + 1);
            max_flow = max_flow.max(flow_from_current + res.0);
            ticks += res.1;
        }
    }
    println!("end : {} {} {} {} ", current_valve_id, time_remaining, max_flow, ticks);
    (max_flow, ticks)
}
