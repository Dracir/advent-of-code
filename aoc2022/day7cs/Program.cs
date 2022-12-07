using System.Text;

Console.WriteLine("Hello, World!");

var example0 = System.IO.File.ReadAllText("/home/richard/Workspace/advent-of-code/aoc2022/examples/day7/example1.txt");
var input = System.IO.File.ReadAllText("/home/richard/Workspace/advent-of-code/aoc2022/inputs/day7-input.txt");

// var e0_part1 = Solver.Part1(example0);
// Console.WriteLine($"Example 0: {e0_part1}");

// var part1_input = Solver.Part1(input);
// Console.WriteLine($"Part 1: {part1_input}");

var e0_part2 = Solver.Part2(example0);
Console.WriteLine($"Example 0: {e0_part2}");

var part2_input = Solver.Part2(input);
Console.WriteLine($"Part 2: {part2_input}");


// Solver.Part2(input);