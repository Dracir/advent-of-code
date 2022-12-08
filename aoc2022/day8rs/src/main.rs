fn main() {
    println!("Hello, world!");
    // let path = "../examples/day8/example1.txt";
    let path = "../inputs/day8-input.txt";

    let input = std::fs::read_to_string(path).unwrap();

    // parse to grid of int
    let grid: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|cell| cell.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();

    println!("Test1 {:?} == false", is_visible(&grid, 1, 1));
    println!("Test1 {:?} == true", is_visible(&grid, 2, 1));
    println!("Test1 {:?} == false", is_visible(&grid, 3, 1));
    println!("Test1 {:?} == true", is_visible(&grid, 1, 2));

    let mut visible = grid.len() * 2 + grid[0].len() * 2 - 4;
    for i in 1..grid.len() - 1 {
        for j in 1..grid[i].len() - 1 {
            if is_visible(&grid, i, j) {
                visible += 1;
            }
        }
    }

    println!("Part 1 Visible: {}", visible);

    println!("Test2 {:?} == 4", scenic_score(&grid, 2, 1));
    println!("Test2 {:?} == 8", scenic_score(&grid, 2, 3));

    let heightest_scenic_score = grid
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, _)| scenic_score(&grid, i, j))
                .max()
                .unwrap()
        })
        .max()
        .unwrap();
    println!("Part 2 Scenic score: {}", heightest_scenic_score);
}

fn scenic_score(grid: &Vec<Vec<i32>>, row: usize, col: usize) -> i32 {
    let height = grid[row][col];

    let mut top = 0;
    for i in (0..row).rev() {
        top += 1;
        if grid[i][col] >= height {
            break;
        }
    }

    let mut bottom = 0;
    for i in row + 1..grid.len() {
        bottom += 1;
        if grid[i][col] >= height {
            break;
        }
    }

    let mut left = 0;
    for i in (0..col).rev() {
        left += 1;
        if grid[row][i] >= height {
            break;
        }
    }

    let mut right = 0;
    for i in col + 1..grid[row].len() {
        right += 1;
        if grid[row][i] >= height {
            break;
        }
    }

    return top * bottom * left * right;
}

fn is_visible(grid: &Vec<Vec<i32>>, row: usize, col: usize) -> bool {
    // for loop exmple from 0 to i
    let height = grid[row][col];

    let mut blocked_from_top = false;
    for i in 0..row {
        if grid[i][col] >= height {
            blocked_from_top = true;
            break;
        }
    }
    let mut blocked_from_bottom = false;
    for i in row + 1..grid.len() {
        if grid[i][col] >= height {
            blocked_from_bottom = true;
            break;
        }
    }
    let mut blocked_from_left = false;
    for i in 0..col {
        if grid[row][i] >= height {
            blocked_from_left = true;
            break;
        }
    }

    let mut blocked_from_right = false;
    for i in col + 1..grid[row].len() {
        if grid[row][i] >= height {
            blocked_from_right = true;
            break;
        }
    }

    return !blocked_from_top || !blocked_from_bottom || !blocked_from_left || !blocked_from_right;
}
