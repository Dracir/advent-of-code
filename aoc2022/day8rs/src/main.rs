mod solver_itterative;
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

    let visible = solver_itterative::solve_part1(&grid);
    println!("Part 1 Visible: {}", visible);

    let heighest_scenic_score = solver_itterative::solve_part2(&grid);
    println!("Part 2 Scenic score: {}", heighest_scenic_score);
}
