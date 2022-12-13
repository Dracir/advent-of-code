#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: fn(u64, u64) -> u64,
    second_param: u64,
    test_divisible: u64,
    if_true_throw_to: usize,
    if_false_throw_to: usize,
    inspections: u64,
}

fn parse_operation_line(line: &str) -> (fn(u64, u64) -> u64, u64) {
    let operations = line.split("= ").nth(1).unwrap();
    let operations = operations.split(" ").collect::<Vec<&str>>();
    let operation = operations[1];

    match operation {
        "*" => {
            let second_param = operations[2];
            if second_param == "old" {
                return (|x, _| x * x, 0);
            } else {
                let multiplier = operations[2].parse::<u64>().unwrap();
                return (|x, y| x * y, multiplier);
            }
        }
        "+" => {
            let addition = operations[2].parse::<u64>().unwrap();
            return (|x, y| x + y, addition);
        }
        _ => panic!("Unknown operation: {}", operation),
    };
}

fn get_line_last_number(line: &str) -> u64 {
    line.split(" ")
        .last()
        .unwrap()
        .parse::<u64>()
        .expect("Unable to parse number")
}

fn parse_to_monkeys(path: &str, monkeys: &mut Vec<Monkey>) {
    let input = std::fs::read_to_string(path).expect("Unable to read file");
    let monkeys_split = input.split("\n\n").collect::<Vec<&str>>();
    for monkey in monkeys_split {
        let monkey = parse_monkey(monkey);
        monkeys.push(monkey);
    }
}

fn parse_monkey(monkey: &str) -> Monkey {
    let mut monkey_lines = monkey.split("\n");
    monkey_lines.next();
    // 	Monkey 0:
    //   Starting items: 79, 98
    //   Operation: new = old * 19
    //   Test: divisible by 23
    //     If true: throw to monkey 2
    //     If false: throw to monkey 3
    let items = monkey_lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split(",")
        .map(|x| x.trim().parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let operation = parse_operation_line(monkey_lines.next().unwrap());
    let test_divisible = get_line_last_number(monkey_lines.next().unwrap());
    let if_true_throw_to = get_line_last_number(monkey_lines.next().unwrap()) as usize;
    let if_false_throw_to = get_line_last_number(monkey_lines.next().unwrap()) as usize;
    let monkey = Monkey {
        items,
        operation: operation.0,
        second_param: operation.1,
        test_divisible,
        if_true_throw_to,
        if_false_throw_to,
        inspections: 0,
    };
    monkey
}

fn print_monkey_items(monkeys: &Vec<Monkey>) {
    for (i, monkey) in monkeys.iter().enumerate() {
        print!("Monkey {}: ", i);
        for item in &monkey.items {
            print!("{} ", item);
        }
        println!();
    }
}

fn play_round(monkeys: &mut Vec<Monkey>, worry_level_division: u64) {
    for i in 0..monkeys.len() {
        for j in 0..monkeys[i].items.len() {
            let worry = (monkeys[i].operation)(monkeys[i].items[j], monkeys[i].second_param)
                / worry_level_division;
            if worry % monkeys[i].test_divisible == 0 {
                let target = monkeys[i].if_true_throw_to;
                monkeys[target].items.push(worry);
            } else {
                let target = monkeys[i].if_false_throw_to;
                monkeys[target].items.push(worry);
            }
        }
        monkeys[i].inspections += monkeys[i].items.len() as u64;
        monkeys[i].items.clear();
    }
}
fn main() {
    println!("Hello Day 11 - Monkey in the Middle!");
    let path = "../inputs/day11/example1.txt";
    // let path = "../inputs/day11/me.txt";

    part1(path);
    part2(path);
}

fn part1(path: &str) {
    let mut monkeys = Vec::<Monkey>::new();
    parse_to_monkeys(path, &mut monkeys);
    for round in 0..20 {
        println!("Round {}", round);
        play_round(&mut monkeys, 3);
        print_monkey_items(&monkeys);
        println!("---");
    }
    for i in 0..monkeys.len() {
        println!(
            "Monkey {} inspected items {} times.",
            i, monkeys[i].inspections
        );
    }
    monkeys.sort_by(|a, b| b.inspections.cmp(&a.inspections));
    let monkey_business = monkeys[0].inspections * monkeys[1].inspections;
    println!("Monkey business part 1: {}", monkey_business);
}

fn part2(path: &str) {
    let mut monkeys = Vec::<Monkey>::new();
    parse_to_monkeys(path, &mut monkeys);
    for round in 0..20 {
        println!("Round {}", round);
        play_round(&mut monkeys, 1);
        print_monkey_items(&monkeys);
        println!("---");
    }
    for i in 0..monkeys.len() {
        println!(
            "Monkey {} inspected items {} times.",
            i, monkeys[i].inspections
        );
    }
    monkeys.sort_by(|a, b| b.inspections.cmp(&a.inspections));
    let monkey_business = monkeys[0].inspections * monkeys[1].inspections;
    println!("Monkey business part 2: {}", monkey_business);
}
