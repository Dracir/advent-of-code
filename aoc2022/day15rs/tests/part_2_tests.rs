use day15rs::sensor::*;
use day15rs::solving::*;

#[test]
fn part2_jumpy_example1() {
    let input = std::fs::read_to_string("../inputs/day15/example1.txt").unwrap();
    let sensors: Vec<Sensor> = input.lines().map(|line| parse_sensor(line)).collect();
    let result = part2_jumpy(&sensors, 20, 20);
    assert_eq!(result, 56000011);
}

#[test]
fn group_sensors_in_zones_test() {
    let input = std::fs::read_to_string("../inputs/day15/example1.txt").unwrap();
    let sensors: Vec<Sensor> = input.lines().map(|line| parse_sensor(line)).collect();

    let mut groups = Vec::<Vec<Sensor>>::new();
    group_sensors_in_zones(&sensors, 2, 20, &mut groups);
    assert_eq!(groups[0].len(), 4);
}
#[test]
fn group_sensors_in_zones_contains_2_18_n2_15() {
    let input = std::fs::read_to_string("../inputs/day15/example1.txt").unwrap();
    let sensors: Vec<Sensor> = input.lines().map(|line| parse_sensor(line)).collect();

    let mut group = Vec::<Sensor>::new();
    group_sensors_for_zone(&sensors, 8, 2, &mut group);
    assert!(group.contains(&Sensor {
        s_x: 2,
        s_y: 18,
        b_x: -2,
        b_y: 15,
        radius: 7
    }));
}

#[test]
fn is_covered_with_cached_test() {
    let input = std::fs::read_to_string("../inputs/day15/example1.txt").unwrap();
    let sensors: Vec<Sensor> = input.lines().map(|line| parse_sensor(line)).collect();

    let mut groups = Vec::<Vec<Sensor>>::new();
    group_sensors_in_zones(&sensors, 2, 20, &mut groups);

    let covered_option = is_covered_by(&sensors, 8, 17);
    assert_eq!(
        covered_option,
        Some(Sensor {
            s_x: 2,
            s_y: 18,
            b_x: -2,
            b_y: 15,
            radius: 7
        })
    );
}

#[test]
fn part2_jumpy_grouped_example1() {
    let input = std::fs::read_to_string("../inputs/day15/example1.txt").unwrap();
    let sensors: Vec<Sensor> = input.lines().map(|line| parse_sensor(line)).collect();
    let result = part2_jumpy_grouped(&sensors, 20, 20, 2, true);
    assert_eq!(result, 56000011);
}

#[test]
fn part2_jumpy_grouped_check_more_threaded_example1() {
    let input = std::fs::read_to_string("../inputs/day15/example1.txt").unwrap();
    let sensors: Vec<Sensor> = input.lines().map(|line| parse_sensor(line)).collect();
    let result = part2_jumpy_grouped_check_more_threaded(&sensors, 20, 20, 2, true);
    assert_eq!(result, 56000011);
}
