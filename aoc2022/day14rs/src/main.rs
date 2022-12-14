struct Cave {
    tiles: Vec<Vec<char>>,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

impl Cave {
    fn set_tile(&mut self, x: usize, y: usize, c: char) {
        if self.tiles.len() <= x {
            self.tiles.resize(x + 1, Vec::new());
        }
        if self.tiles[x].len() <= y {
            self.tiles[x].resize(y + 1, '.');
        }
        self.tiles[x][y] = c;
    }
    fn get_tile(&self, x: usize, y: usize) -> char {
        if self.tiles.len() <= x {
            return '.';
        }
        if self.tiles[x].len() <= y {
            return '.';
        }
        self.tiles[x][y]
    }

    fn print_partial(&self, x: usize, y: usize, width: usize, height: usize) {
        for y in y..y + height {
            for x in x..x + width {
                print!("{}", self.get_tile(x, y));
            }
            println!();
        }
    }

    pub(crate) fn get_lowest_wall_y(&self) -> usize {
        let mut lowest_y = 0;
        for x in 0..self.tiles.len() {
            for y in 0..self.tiles[x].len() {
                if self.tiles[x][y] == '#' {
                    if y > lowest_y {
                        lowest_y = y;
                    }
                }
            }
        }
        lowest_y
    }
}

fn main() {
    println!("Hello, world!");
    // let input = std::fs::read_to_string("../inputs/day14/example1.txt").unwrap();
    let input = std::fs::read_to_string("../inputs/day14/me.txt").unwrap();

    // part1(input);
    part2(input);
}

fn part1(input: String) {
    let mut cave = Cave { tiles: Vec::new() };
    for _ in 0..1000 {
        cave.tiles.push(Vec::new());
    }
    for line in input.lines() {
        add_wall(line, &mut cave);
    }
    let lowest_wall_y = cave.get_lowest_wall_y();
    println!("Lowest wall y: {}", lowest_wall_y);
    cave.set_tile(500, 0, '+');
    println!("Cave:");
    // cave.print_partial(460, 0, 80, 30);
    cave.print_partial(485, 0, 30, 20);
    for i in 0..1000 {
        let res = spawn_a_sand(500, 0, &cave, lowest_wall_y);
        match res {
            Ok(pos) => {
                println!("Sand stopped at {}, {}", pos.x, pos.y);
                cave.set_tile(pos.x, pos.y, 'o');
            }
            Err(_) => {
                println!("Sand {} stopped", i);
                break;
            }
        }
        cave.print_partial(485, 0, 30, 20);
    }
}

fn part2(input: String) {
    let mut cave = Cave { tiles: Vec::new() };
    for _ in 0..1000 {
        cave.tiles.push(Vec::new());
    }
    for line in input.lines() {
        add_wall(line, &mut cave);
    }
    let lowest_wall_y = cave.get_lowest_wall_y() + 2;
    println!("Lowest wall y: {}", lowest_wall_y);
    for x in 0..1000 {
        cave.set_tile(x, lowest_wall_y, '#');
    }

    cave.set_tile(500, 0, '+');
    println!("Cave:");
    // cave.print_partial(460, 0, 80, 30);
    cave.print_partial(485, 0, 30, 20);
    for i in 0..100000 {
        let res = spawn_a_sand(500, 0, &cave, lowest_wall_y);
        match res {
            Ok(pos) => {
                // println!("Sand stopped at {}, {}", pos.x, pos.y);
                if pos.x == 500 && pos.y == 0 {
                    println!("Sand {} stopped", i);
                    break;
                }
                cave.set_tile(pos.x, pos.y, 'o');
            }
            Err(_) => {
                println!("No change at {}", i);
                break;
            }
        }
        // cave.print_partial(485, 0, 30, 20);
    }
    // cave.print_partial(400, 0, 200, lowest_wall_y);
}

fn spawn_a_sand(
    spawn_x: usize,
    spawn_y: usize,
    cave: &Cave,
    lowest_wall_y: usize,
) -> Result<Position, ()> {
    let mut x = spawn_x;
    let mut y = spawn_y;
    loop {
        y += 1;
        if y > lowest_wall_y {
            return Err(());
        }
        if cave.get_tile(x, y) == '.' {
            continue;
        } else {
            //Blocked, sand will move diagonal
            if cave.get_tile(x - 1, y) == '.' {
                x -= 1;
                continue;
            } else if cave.get_tile(x + 1, y) == '.' {
                x += 1;
                continue;
            }
        }
        break;
    }
    y -= 1;
    return Ok(Position { x, y });
}

fn str_to_position(line: &str) -> (usize, usize) {
    let mut iter = line.split(",");
    let x = iter.next().unwrap().parse::<usize>().unwrap();
    let y = iter.next().unwrap().parse::<usize>().unwrap();
    (x, y)
}

fn add_wall(line: &str, cave: &mut Cave) {
    let points = line.split("->").map(|s| s.trim()).collect::<Vec<_>>();
    let mut from = str_to_position(points[0]);
    for i in 1..points.len() {
        let to = str_to_position(points[i]);
        add_wall_segment(from, to, cave);
        // cave.print_partial(490, 0, 20, 12);
        from = to;
    }
}

fn add_wall_segment(from: (usize, usize), to: (usize, usize), cave: &mut Cave) {
    // println!("Adding wall from {:?} to {:?}", from, to);
    if from.1 == to.1 {
        if from.0 > to.0 {
            for x in to.0..from.0 + 1 {
                cave.set_tile(x, from.1, '#');
            }
        } else {
            for x in from.0..to.0 + 1 {
                cave.set_tile(x, from.1, '#');
            }
        }
    } else {
        if from.1 > to.1 {
            for y in to.1..from.1 + 1 {
                cave.set_tile(from.0, y, '#');
            }
        } else {
            for y in from.1..to.1 + 1 {
                cave.set_tile(from.0, y, '#');
            }
        }
    }
}
