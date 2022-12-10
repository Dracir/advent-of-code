struct Registers {
    x: i64,
    cycles: u64,
}

fn main() {
    println!("Hello, world!");

    // let path = "../examples/day10/example1.txt";
    // let path = "../examples/day10/example2.txt";
    let path = "../inputs/day10-input.txt";

    let input = std::fs::read_to_string(path).unwrap();

    part1(&input);
    // part2(&input, 2);
}

fn addx(registers: &Registers, amount: i64) -> Vec<Registers> {
    let mut result = Vec::<Registers>::new();
    result.push(Registers {
        x: registers.x,
        cycles: registers.cycles + 1,
    });
    result.push(Registers {
        x: registers.x + amount,
        cycles: registers.cycles + 2,
    });
    return result;
}

fn noop(registers: &Registers) -> Vec<Registers> {
    let mut result = Vec::<Registers>::new();
    result.push(Registers {
        x: registers.x,
        cycles: registers.cycles + 1,
    });
    return result;
}

fn exe_instruction(
    instruction: &str,
    params: Option<&str>,
    registers: &Registers,
) -> Vec<Registers> {
    return match instruction {
        "addx" => addx(registers, params.unwrap().parse::<i64>().unwrap()),
        "noop" => noop(registers),
        _ => panic!("Unknown instruction: {}", instruction),
    };
}

fn part1(input: &str) {
    let mut registers_cycles = Vec::<Registers>::new();
    registers_cycles.push(Registers { x: 1, cycles: 1 });
    registers_cycles.push(Registers { x: 1, cycles: 1 });

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let instruction = parts.next().unwrap();
        let params = parts.next();
        let next_steps = exe_instruction(
            instruction,
            params,
            &registers_cycles[registers_cycles.len() - 1],
        );
        for next_step in next_steps {
            registers_cycles.push(next_step);
        }
    }

    let signal_at_20 = 20 * registers_cycles[20].x;
    let signal_at_60 = 60 * registers_cycles[60].x;
    let signal_at_100 = 100 * registers_cycles[100].x;
    let signal_at_140 = 140 * registers_cycles[140].x;
    let signal_at_180 = 180 * registers_cycles[180].x;
    let signal_at_220 = 220 * registers_cycles[220].x;

    println!("Signal at 20: {}", signal_at_20);
    println!("Signal at 60: {}", signal_at_60);
    println!("Signal at 100: {}", signal_at_100);
    println!("Signal at 140: {}", signal_at_140);
    println!("Signal at 180: {}", signal_at_180);
    println!("Signal at 220: {}", signal_at_220);

    let signal_strenght =
        signal_at_20 + signal_at_60 + signal_at_100 + signal_at_140 + signal_at_180 + signal_at_220;
    println!("Part 1: {}", signal_strenght);

    let mut crt: Vec<char> = vec!['.'; 241];
    for i in 1..241 {
        let x = registers_cycles[i].x;
        let pos = ((i - 1) as i64) % 40;
        if x >= pos - 1 && x <= pos + 1 {
            crt[i - 1] = '#';
        }
    }
    let crt_lines = crt.chunks(40).collect::<Vec<&[char]>>();
    for line in crt_lines {
        println!("{}", line.iter().collect::<String>());
    }
}
