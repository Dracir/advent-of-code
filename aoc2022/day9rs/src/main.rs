use core::time;
use std::thread;

use termion::{color, terminal_size};

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
        (2, 2) => Position {
            x: tail.x + 1,
            y: tail.y + 1,
        },
        (2, -2) => Position {
            x: tail.x + 1,
            y: tail.y - 1,
        },
        (-2, 2) => Position {
            x: tail.x - 1,
            y: tail.y + 1,
        },
        (-2, -2) => Position {
            x: tail.x - 1,
            y: tail.y - 1,
        },
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
        _ => {
            panic!("Unknown case : {:?}", (x_diff, y_diff))
        }
    };
}

fn print_grid(screen_size: (usize, usize), viewport_offset: (usize, usize)) {
    print!("{}", color::Fg(color::Rgb(55, 55, 55)));
    for y in 0..screen_size.1 {
        println!(
            "{}{}",
            termion::cursor::Goto(1, (y + 1).try_into().unwrap()),
            ".".repeat(screen_size.0 as usize)
        );
    }

    // println!("viewport_offset:{:?}", viewport_offset);
    let x_offset = 1 + ((10 - viewport_offset.0 % 10) % 10) as i32;
    let y_offset = 1 + ((10 - viewport_offset.1 % 10) % 10) as i32;
    // println!("x:{},y:{}", x_offset, y_offset);
    print!("{}", color::Fg(color::LightWhite));
    for x in (x_offset..screen_size.0 as i32).step_by(10) {
        for y in (y_offset..screen_size.1 as i32).step_by(10) {
            println!("{}{}", termion::cursor::Goto(x as u16, y as u16), "+");
        }
    }
}
#[allow(dead_code)]
fn print_rope(rope: &Vec<Position>, screen_size: (usize, usize), viewport_offset: (usize, usize)) {
    let height = screen_size.1 - 1;

    for i in 0..rope.len() {
        let beau_character = char::from_u32(48 + i as u32).unwrap();
        let x = (rope[i].x + viewport_offset.0 as i32) as u16;
        let y = (height as i32 - rope[i].y - viewport_offset.1 as i32) as u16;
        println!(
            "{}{}{}",
            termion::cursor::Goto(x, y),
            color::Fg(color::Rgb(255, (i * 25) as u8, 0)),
            beau_character
        );
    }
}

fn part2(input: &String, rope_length: usize) {
    print!("{}", termion::clear::All);
    print!("{}", termion::cursor::Hide);

    let mut rope: Vec<Position> = Vec::new();
    for _ in 0..rope_length {
        rope.push(Position { x: 0, y: 0 });
    }

    let mut position_visited_by_last_tail = std::collections::HashMap::new();
    let size = terminal_size().unwrap();
    let size = (size.0 as usize, (size.1 - 1) as usize);
    let viewport_offset = (size.0 / 2, size.1 / 2);

    for line in input.lines() {
        let splited = line.split(" ").collect::<Vec<&str>>();
        let direction = splited[0];
        let distance = splited[1].parse::<i32>().unwrap();

        // println!("Direction : {:?} Distance : {:?}", direction, distance);
        print_grid(size, viewport_offset);
        print_rope(&rope, size, viewport_offset);
        thread::sleep(time::Duration::from_millis(10));

        for _ in 0..distance {
            // println!("{} / {} ", i, distance);
            rope[0] = move_direction(direction, &rope[0]);
            print_rope(&rope, size, viewport_offset);
            thread::sleep(time::Duration::from_millis(1));

            // print_rope(&rope);
            // println!("lets go les ropes !");
            for i in 1..rope.len() {
                // println!("Rope {} ", i);
                rope[i] = tail_follow(&rope[i - 1], &rope[i]);
                print_rope(&rope, size, viewport_offset);
                thread::sleep(time::Duration::from_millis(1));
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
    print!("{}", termion::cursor::Show);
    println!("UniqueVisited : {:?}", position_visited_by_last_tail.len());
}
