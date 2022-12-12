use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    row: usize,
    col: usize,
}

fn main() {
    println!("Hello, world!");
    // let path = "../inputs/day12/example1.txt";
    let path = "../inputs/day12/me.txt";

    let file_content =
        std::fs::read_to_string(path).expect("Something went wrong reading the file");
    // map file_content to a grid 2d of chars
    let grid: Vec<Vec<char>> = file_content
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    part1(&grid);
    part2(&grid);
}

fn part2(grid: &Vec<Vec<char>>) {
    // Find all positions that are S or a
    let end_position = find_position(&grid, 'E');

    let mut min = usize::MAX;
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == 'a' || grid[row][col] == 'S' {
                let start_position = Position { row, col };
                let shortest_path = shortest_path(&grid, start_position, end_position);
                let shortest_path = shortest_path
                    .iter()
                    .rev()
                    .cloned()
                    .collect::<Vec<Position>>();
                // print_shortest_path(&grid, &shortest_path);
                // println!("shortest_path: {:?}", shortest_path);
                // println!(
                //     "shortest_path.len({},{}): {:?}",
                //     row,
                //     col,
                //     shortest_path.len()
                // );
                if shortest_path.len() != 0 && shortest_path.len() - 1 < min {
                    min = shortest_path.len() - 1;
                }
            }
        }
    }
    println!("min: {:?}", min);
}

fn part1(grid: &Vec<Vec<char>>) {
    let start_position = find_position(&grid, 'S');
    let end_position = find_position(&grid, 'E');
    println!("start_position: {:?}", start_position);
    println!("end_position: {:?}", end_position);
    let shortest_path = shortest_path(&grid, start_position, end_position);
    let shortest_path = shortest_path
        .iter()
        .rev()
        .cloned()
        .collect::<Vec<Position>>();
    print_shortest_path(&grid, &shortest_path);
    println!("shortest_path: {:?}", shortest_path);
    println!("shortest_path.len(): {:?}", shortest_path.len() - 1);
}

fn print_shortest_path(grid: &Vec<Vec<char>>, shortest_path: &[Position]) {
    let mut rendering = grid.clone();
    // replace all points with .
    for row in 0..rendering.len() {
        for col in 0..rendering[row].len() {
            rendering[row][col] = '.';
        }
    }
    for i in 0..shortest_path.len() - 1 {
        let position = shortest_path[i];
        let next_position = shortest_path[i + 1];
        if next_position.row > position.row && next_position.col == position.col {
            rendering[position.row][position.col] = 'v';
        } else if next_position.row < position.row && next_position.col == position.col {
            rendering[position.row][position.col] = '^';
        } else if next_position.row == position.row && next_position.col > position.col {
            rendering[position.row][position.col] = '>';
        } else if next_position.row == position.row && next_position.col < position.col {
            rendering[position.row][position.col] = '<';
        }
    }
    for row in 0..rendering.len() {
        for col in 0..rendering[row].len() {
            print!("{}", rendering[row][col]);
        }
        println!();
    }
}

fn heuristic(a: Position, b: Position) -> i32 {
    //return the distance between a and b
    let dx = (a.col as i32 - b.col as i32).abs();
    let dy = (a.row as i32 - b.row as i32).abs();
    dx + dy
}
fn shortest_path(
    grid: &Vec<Vec<char>>,
    start_position: Position,
    end_position: Position,
) -> Vec<Position> {
    let mut open_set = vec![start_position];

    let mut came_from: HashMap<Position, Position> = HashMap::new();

    let mut g_score: HashMap<Position, i32> = HashMap::new();
    //add all points to g_score with a score of i32.max
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            g_score.insert(Position { row, col }, i32::max_value());
        }
    }
    *g_score.get_mut(&start_position).unwrap() = 0;

    let mut f_score: HashMap<Position, i32> = HashMap::new();
    f_score.insert(start_position, heuristic(start_position, end_position));

    while !open_set.is_empty() {
        let current_open_set = open_set.clone();
        let current = current_open_set.iter().min_by_key(|x| f_score[x]).unwrap();
        // let current = open_set.iter().min_by_key(|x| f_score[x]).unwrap().to_owned();
        // println!("current: {:?}", current);
        if *current == end_position {
            return reconstruct_path(&came_from, *current);
        }

        open_set.retain(|&x| x != *current);

        let neighbors = get_neighbors(grid, *current);
        // println!("neighbors: {:?}", neighbors);
        for neighbor in neighbors {
            // println!("neighbor: {:?}", neighbor);
            let tentative_g_score = g_score[current] + 1;
            if tentative_g_score < g_score[&neighbor] {
                came_from.insert(neighbor, *current);
                *g_score.get_mut(&neighbor).unwrap() = tentative_g_score;
                f_score.insert(
                    neighbor,
                    tentative_g_score + heuristic(neighbor, end_position),
                );
                if !open_set.contains(&neighbor) {
                    open_set.push(neighbor);
                }
            }
        }
    }
    // println!("no path found");
    vec![]
}

fn is_at_most_one_heigher(from: char, to: char) -> bool {
    //return true if the value is at most one letter higher
    // for exemple 'c' can go to 'a' or 'b' or 'c' or 'd' but not 'e'
    if to == 'E' {
        if from == 'y' || from == 'z' {
            return true;
        }
    } else if from == 'S' {
        if to == 'a' || to == 'b' {
            return true;
        }
    } else if (to as i32) - (from as i32) <= 1 {
        return true;
    }
    false
}

fn get_neighbors(grid: &Vec<Vec<char>>, current: Position) -> Vec<Position> {
    //return char from north, east, ouest, south
    //only if the character is 1 letter higher at max
    // for exemple 'c' can go to 'a' or 'b' or 'c' or 'd' but not 'e'

    let mut neighbors = vec![];
    let current_value = grid[current.row][current.col];

    if current.row >= 1 {
        let north = Position {
            row: current.row - 1,
            col: current.col,
        };
        if is_at_most_one_heigher(current_value, grid[north.row][north.col]) {
            neighbors.push(north);
        }
    }

    if current.row < grid.len() - 1 {
        let south = Position {
            row: current.row + 1,
            col: current.col,
        };
        if is_at_most_one_heigher(current_value, grid[south.row][south.col]) {
            neighbors.push(south);
        }
    }

    if current.col >= 1 {
        let west = Position {
            row: current.row,
            col: current.col - 1,
        };
        if is_at_most_one_heigher(current_value, grid[west.row][west.col]) {
            neighbors.push(west);
        }
    }

    if current.col < grid[current.row].len() - 1 {
        let east = Position {
            row: current.row,
            col: current.col + 1,
        };
        if is_at_most_one_heigher(current_value, grid[east.row][east.col]) {
            neighbors.push(east);
        }
    }

    neighbors
}

fn reconstruct_path(came_from: &HashMap<Position, Position>, current: Position) -> Vec<Position> {
    let mut total_path = vec![current];
    let mut current = current;
    while came_from.contains_key(&current) {
        current = came_from[&current];
        total_path.push(current);
    }
    total_path
}

fn find_position(grid: &Vec<Vec<char>>, arg: char) -> Position {
    let row = grid.iter().position(|row| row.contains(&arg)).unwrap();
    let col = grid[row].iter().position(|&c| c == arg).unwrap();
    return Position { row, col };
}
