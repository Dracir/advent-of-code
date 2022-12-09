pub fn solve_part1(grid: &Vec<Vec<i32>>) -> usize {
    let mut visible = grid.len() * 2 + grid[0].len() * 2 - 4;
    for i in 1..grid.len() - 1 {
        for j in 1..grid[i].len() - 1 {
            if is_visible(&grid, i, j) {
                visible += 1;
            }
        }
    }
    return visible;
}

pub fn solve_part2(grid: &Vec<Vec<i32>>) -> i32 {
    return grid
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
