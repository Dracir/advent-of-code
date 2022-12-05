fn main() {
    println!("Hello, world!");
    // let path = "../examples/day4/example0.txt";
    let path = "../inputs/day4-input.txt";

    let input = std::fs::read_to_string(path).unwrap();
    let mut input_values: Vec<i32> = Vec::new();

    for line in input.lines() {
        for elve in line.split(",") {
            for value in elve.split("-") {
                input_values.push(value.parse::<i32>().unwrap());
            }
        }
    }

    let mut overlapping_full = 0;
    let mut overlapping_at_all = 0;

    for group in input_values.chunks(4) {
        let a1 = group[0];
        let a2 = group[1];
        let b1 = group[2];
        let b2 = group[3];

        if a1 >= b1 && a2 <= b2 {
            overlapping_full += 1;
        } else if b1 >= a1 && b2 <= a2 {
            overlapping_full += 1;
        }

        if (a2 >= b1 && a2 <= b2) || (a1 >= b1 && a1 <= b2) {
            overlapping_at_all += 1;
        } else if (b2 >= a1 && b2 <= a2) || (b1 >= a1 && b1 <= a2) {
            overlapping_at_all += 1;
        }
    }

    println!("Overlapping full: {}", overlapping_full);
    println!("Overlapping at all: {}", overlapping_at_all);
}
