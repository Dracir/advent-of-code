#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

fn main() {
    println!("Hello, world!");

    // let path = "../examples/day9/example1.txt";
    // let path = "../examples/day9/example2.txt";
    let path = "../inputs/day9-input.txt";

    let input = std::fs::read_to_string(path).unwrap();

    // part1(&input);
    // part2(&input, 2);
    part2(&input, 10);
}

fn move_direction(direction: &str, position: &Position) -> Position {
    match direction {
        "R" => {
            return Position {
                x: position.x + 1,
                y: position.y,
            };
        }
        "L" => {
            return Position {
                x: position.x - 1,
                y: position.y,
            };
        }
        "U" => {
            return Position {
                x: position.x,
                y: position.y + 1,
            };
        }
        "D" => {
            return Position {
                x: position.x,
                y: position.y - 1,
            };
        }
        _ => panic!("Unknown direction"),
    }
}

fn tail_follow(head: &Position, tail: &Position) -> Position {
    let x_diff = head.x - tail.x;
    let y_diff = head.y - tail.y;

    if x_diff.abs() <= 1 && y_diff.abs() <= 1 {
        return tail.clone();
    }
    return match (x_diff, y_diff) {
        //Simple Horizontal
        (2, _) => Position {
            x: tail.x + 1,
            y: head.y,
        },
        (-2, _) => Position {
            x: tail.x - 1,
            y: head.y,
        },
        //Simple Vertical
        (_, 2) => Position {
            x: head.x,
            y: tail.y + 1,
        },
        (_, -2) => Position {
            x: head.x,
            y: tail.y - 1,
        },
        // //Far ?
        // (3, _) => Position {
        //     x: tail.x + 1,
        //     y: head.y,
        // },
        // (-3, _) => Position {
        //     x: tail.x - 1,
        //     y: head.y,
        // },
        // //Simple Vertical
        // (_, 3) => Position {
        //     x: head.x,
        //     y: tail.y + 1,
        // },
        // (_, -3) => Position {
        //     x: head.x,
        //     y: tail.y - 1,
        // },
        _ => {
            panic!("Unknown case : {:?}", (x_diff, y_diff))
        }
    };
}

fn print_rope(rope: &Vec<Position>) {
    let most_left = rope.iter().min_by_key(|p| p.x).unwrap().x;
    let most_bottom = rope.iter().min_by_key(|p| p.y).unwrap().y;
    let most_right = rope.iter().max_by_key(|p| p.x).unwrap().x;
    let most_top = rope.iter().max_by_key(|p| p.y).unwrap().y;

    let mut grid = vec![
        vec![' '; (most_right - most_left + 1) as usize];
        (most_top - most_bottom + 1) as usize
    ];

    for position in rope {
        grid[(position.y - most_bottom) as usize][(position.x - most_left) as usize] = 'X';
    }
}

fn part2(input: &String, rope_length: usize) {
    let mut rope: Vec<Position> = Vec::new();
    for _ in 0..rope_length {
        rope.push(Position { x: 0, y: 0 });
    }

    let mut position_visited_by_last_tail: std::collections::HashMap<Position, i32> =
        std::collections::HashMap::new();

    for line in input.lines() {
        let splited = line.split(" ").collect::<Vec<&str>>();
        let direction = splited[0];
        let distance = splited[1].parse::<i32>().unwrap();

        println!("Direction : {:?} Distance : {:?}", direction, distance);
        for _ in 0..distance {
            rope[0] = move_direction(direction, &rope[0]);
            for i in 1..rope.len() {
                rope[i] = tail_follow(&rope[i - 1], &rope[i]);
            }

            // if position_visite contains tailPosition
            if !position_visited_by_last_tail.contains_key(&rope[rope_length - 1]) {
                position_visited_by_last_tail.insert(rope[rope_length - 1], 1);
            } else {
                let value = position_visited_by_last_tail
                    .get(&rope[rope_length - 1])
                    .unwrap();
                position_visited_by_last_tail.insert(rope[rope_length - 1], value + 1);
            }
        }
    }
    println!("UniqueVisited : {:?}", position_visited_by_last_tail.len());
}

fn part1(input: &String) {
    let mut head_position = Position { x: 0, y: 0 };
    let mut tail_position = Position { x: 0, y: 0 };
    let mut position_visites: std::collections::HashMap<Position, i32> =
        std::collections::HashMap::new();
    position_visites.insert(Position { x: 0, y: 0 }, 1);

    for line in input.lines() {
        let splited = line.split(" ").collect::<Vec<&str>>();
        let direction = splited[0];
        let distance = splited[1].parse::<i32>().unwrap();

        for _ in 0..distance {
            head_position = move_direction(direction, &head_position);
            tail_position = tail_follow(&head_position, &tail_position);

            // if position_visite contains tailPosition
            if !position_visites.contains_key(&tail_position) {
                position_visites.insert(tail_position, 1);
            } else {
                let value = position_visites.get(&tail_position).unwrap();
                position_visites.insert(tail_position, value + 1);
            }
        }
    }
    println!("UniqueVisited : {:?}", position_visites.len());
}
