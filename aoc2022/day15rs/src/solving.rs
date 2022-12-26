use std::thread;

use crate::sensor::*;

pub fn manhattan_distance(x1: i64, y1: i64, x2: i64, y2: i64) -> i64 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

pub fn part1(sensors: &Vec<Sensor>, y: i64) -> i64 {
    // let min_x_at_y
    let (min_x, max_x, min_x_at_y, max_x_at_y) = get_real_range(sensors, y);

    // println!("min_x: {}", min_x);
    // println!("max_x: {}", max_x);

    // println!("min_x_at_y: {}", min_x_at_y);
    // println!("max_x_at_y: {}", max_x_at_y);
    let width_at_y = max_x_at_y - min_x_at_y + 1;
    // println!("width_at_y: {}", width_at_y);
    let mut position_covered = 0;

    for x in min_x_at_y..=max_x_at_y {
        for s in sensors.iter() {
            if s.b_x == x && s.b_y == y {
                // There is already a beacon at this position
                position_covered += 1;
                break;
            }
        }
    }
    // println!("position_covered: {}", position_covered);
    let position_free = width_at_y - position_covered;
    // println!("position_free: {}", position_free);
    position_free
}

pub fn part2(sensors: &Vec<Sensor>, search_range_x: usize, search_range_y: usize) -> i64 {
    for x in 0..=search_range_x {
        if x % 40000 == 0 {
            println!("x: {}", x);
        }
        for y in 0..=search_range_y {
            if !is_covered_with_cached(sensors, x as i64, y as i64) {
                return 4000000 * x as i64 + y as i64;
            }
        }
    }
    0
}

pub fn part2_jumpy(sensors: &Vec<Sensor>, search_range_x: usize, search_range_y: usize) -> i64 {
    for x in 0..=search_range_x {
        // println!("x: {}", x);
        let y = find_empty_jumpy_at_x(sensors, search_range_y, x as i64);
        if y > 0 {
            // println!("Found at x: {}, y: {}", x, y);
            return 4000000 * x as i64 + y as i64;
        }
    }
    0
}

pub fn part2_jumpy_grouped(
    sensors: &Vec<Sensor>,
    search_range_x: usize,
    search_range_y: usize,
    group_width: usize,
    verbose: bool,
) -> i64 {
    let mut groups = Vec::<Vec<Sensor>>::new();
    group_sensors_in_zones(sensors, group_width, search_range_x, &mut groups);

    for x in 0..=search_range_x {
        // Print every 1% done
        let group = x / group_width;
        if verbose && x % (group_width) == 0 {
            println!("group: {}", group);
        }
        // println!("x: {}, group: {}", x, group);
        // println!("x: {}", x);
        let y = find_empty_jumpy_at_x(&groups[group], search_range_y, x as i64);
        if y > 0 {
            // println!("Found at x: {}, y: {}", x, y);
            return 4000000 * x as i64 + y as i64;
        }
    }
    0
}
pub fn part2_jumpy_grouped_check_more(
    sensors: &Vec<Sensor>,
    search_range_x: usize,
    search_range_y: usize,
    group_width: usize,
    verbose: bool,
) -> i64 {
    let mut groups = Vec::<Vec<Sensor>>::new();
    group_sensors_in_zones(sensors, group_width, search_range_x, &mut groups);

    for x in 0..=search_range_x {
        let group = x / group_width;
        if verbose && x % (group_width) == 0 {
            println!("group: {}", group);
        }
        let y = find_empty_jumpy_at_x_more(&groups[group], search_range_y, x as i64);
        if y > 0 {
            // println!("Found at x: {}, y: {}", x, y);
            return 4000000 * x as i64 + y as i64;
        }
    }
    0
}
pub fn part2_jumpy_grouped_check_more_threaded(
    sensors: &Vec<Sensor>,
    search_range_x: usize,
    search_range_y: usize,
    group_width: usize,
    verbose: bool,
) -> i64 {
    let mut groups = Vec::<Vec<Sensor>>::new();
    group_sensors_in_zones(sensors, group_width, search_range_x, &mut groups);

    let mut threads = Vec::new();
    for i in 0..search_range_x / group_width {
        let fixed_groups = groups.clone();
        let thread = thread::spawn(move || {
            let start_x = i * group_width;
            for x in start_x..start_x + group_width {
                let group = x / group_width;
                // if verbose {
                //     println!("Checking at x: {}", x);
                // }
                let y = find_empty_jumpy_at_x_more(&fixed_groups[group], search_range_y, x as i64);
                if y > 0 {
                    // println!("Found at x: {}, y: {}", x, y);
                    return 4000000 * x as i64 + y as i64;
                }
            }
            0
        });
        threads.push(thread);
    }
    for h in threads {
        let res = h.join().unwrap();
        if res > 0 {
            return res;
        }
    }
    0
}
fn fun_name(
    x: usize,
    group_width: usize,
    verbose: bool,
    groups: &Vec<Vec<Sensor>>,
    search_range_y: usize,
) -> Option<i64> {
    let group = x / group_width;
    if verbose && x % (group_width) == 0 {
        println!("group: {}", group);
    }
    let y = find_empty_jumpy_at_x_more(&groups[group], search_range_y, x as i64);
    if y > 0 {
        // println!("Found at x: {}, y: {}", x, y);
        return Some(4000000 * x as i64 + y as i64);
    }
    None
}

pub fn group_sensors_for_zone(
    sensors: &Vec<Sensor>,
    start_x: usize,
    group_width: usize,
    sensors_groupes: &mut Vec<Sensor>,
) {
    let end = start_x + group_width;
    for sensor in sensors {
        if (start_x as i64) >= sensor.s_x - sensor.radius - 1 && (end as i64) <= sensor.s_x + sensor.radius + 1 {
            sensors_groupes.push(sensor.clone());
        }
    }
}

pub fn group_sensors_in_zones(
    sensors: &Vec<Sensor>,
    group_width: usize,
    total_width: usize,
    sensors_groupes: &mut Vec<Vec<Sensor>>,
) {
    for i in 0..=total_width / group_width {
        let mut sensors_group = Vec::<Sensor>::new();
        let start_x = i * group_width;
        group_sensors_for_zone(sensors, start_x, group_width, &mut sensors_group);
        sensors_groupes.push(sensors_group);
    }
}

pub fn find_empty_jumpy_at_x(sensors: &Vec<Sensor>, search_range: usize, x: i64) -> i64 {
    let mut y = 0;
    loop {
        if y >= search_range {
            return 0;
        }
        let covered_option = is_covered_by(sensors, x as i64, y as i64);
        // println!("x: {}, y: {}, covered_option: {:?}", x, y, covered_option);
        match covered_option {
            Some(sensor) => {
                let y_distance = sensor.s_y - y as i64;
                if y_distance > 0 {
                    // println!("y += {}", y_distance);
                    y += (2 * y_distance) as usize;
                } else {
                    // println!("y++");
                    y += 1;
                }
            }
            _ => {
                // println!("Found at x: {}, y: {}", x, y);
                return y as i64;
            }
        }
    }
}

pub fn find_empty_jumpy_at_x_more(sensors: &Vec<Sensor>, search_range: usize, x: i64) -> i64 {
    let mut y = 0;
    loop {
        if y >= search_range {
            return 0;
        }
        let covered_option = is_covered_by_list(sensors, x as i64, y as i64);
        match covered_option {
            Some(covering_sensors) => {
                let mut best_jump = 1;
                for sensor in covering_sensors {
                    let y_distance = sensor.s_y - y as i64;
                    if y_distance > 0 {
                        // println!("y += {}", y_distance);
                        best_jump = best_jump.max((2 * y_distance) as usize);
                    }
                }
                y += best_jump;
            }
            _ => {
                // println!("Found at x: {}, y: {}", x, y);
                return y as i64;
            }
        }
    }
}

pub fn is_covered(sensors: &Vec<Sensor>, x: i64, y: i64) -> bool {
    for s in sensors.iter() {
        let distance_from_senstor_to_beacon = s.distance_covered();
        let distance_to_sensor = manhattan_distance(x, y, s.s_x, s.s_y);
        if distance_to_sensor <= distance_from_senstor_to_beacon {
            return true;
        }
    }
    false
}

pub fn is_covered_with_cached(sensors: &Vec<Sensor>, x: i64, y: i64) -> bool {
    for s in sensors.iter() {
        let distance_from_senstor_to_beacon = s.radius;
        let distance_to_sensor = manhattan_distance(x, y, s.s_x, s.s_y);
        if distance_to_sensor <= distance_from_senstor_to_beacon {
            return true;
        }
    }
    false
}

pub fn is_covered_by(sensors: &Vec<Sensor>, x: i64, y: i64) -> Option<Sensor> {
    for s in sensors.iter() {
        let distance_from_senstor_to_beacon = s.radius;
        let distance_to_sensor = manhattan_distance(x, y, s.s_x, s.s_y);
        if distance_to_sensor <= distance_from_senstor_to_beacon {
            return Some(s.clone());
        }
    }
    None
}
pub fn is_covered_by_list(sensors: &Vec<Sensor>, x: i64, y: i64) -> Option<Vec<Sensor>> {
    let mut covered_sensors = Vec::<Sensor>::new();
    for s in sensors.iter() {
        let distance_from_senstor_to_beacon = s.radius;
        let distance_to_sensor = manhattan_distance(x, y, s.s_x, s.s_y);
        if distance_to_sensor <= distance_from_senstor_to_beacon {
            covered_sensors.push(s.clone());
        }
    }
    if covered_sensors.is_empty() {
        return None;
    } else {
        return Some(covered_sensors);
    }
}
pub fn get_real_range(sensors: &Vec<Sensor>, y: i64) -> (i64, i64, i64, i64) {
    let (min_x, max_x, min_y, max_y, width, height) = get_ranges(&sensors);
    let mut min_x_at_y = i64::MAX;
    let mut max_x_at_y = i64::MIN;
    for x in min_x..=max_x {
        if is_covered(sensors, x, y) {
            if x < min_x_at_y {
                min_x_at_y = x;
            }
            if x > max_x_at_y {
                max_x_at_y = x;
            }
            break;
        }
    }
    (min_x, max_x, min_x_at_y, max_x_at_y)
}

pub fn get_ranges(sensors: &Vec<Sensor>) -> (i64, i64, i64, i64, i64, i64) {
    let min_x = sensors.iter().map(|sensor| sensor.s_x - sensor.b_x).min().unwrap();
    let max_x = sensors.iter().map(|sensor| sensor.s_x + sensor.b_x).max().unwrap();
    let min_y = sensors.iter().map(|sensor| sensor.s_y - sensor.b_y).min().unwrap();
    let max_y = sensors.iter().map(|sensor| sensor.s_y + sensor.b_y).max().unwrap();
    let width = max_x - min_x;
    let height = max_y - min_y;
    (min_x, max_x, min_y, max_y, width, height)
}
