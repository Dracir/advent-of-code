using System.Text;

Console.WriteLine("Hello, World!");

var example0 = System.IO.File.ReadAllText("/home/richard/Workspace/advent-of-code/aoc2022/examples/day6/example0.txt");
var example6 = System.IO.File.ReadAllText("/home/richard/Workspace/advent-of-code/aoc2022/examples/day6/example6.txt");
var example8 = System.IO.File.ReadAllText("/home/richard/Workspace/advent-of-code/aoc2022/examples/day6/example8.txt");
var example10 = System.IO.File.ReadAllText("/home/richard/Workspace/advent-of-code/aoc2022/examples/day6/example10.txt");
var example12 = System.IO.File.ReadAllText("/home/richard/Workspace/advent-of-code/aoc2022/examples/day6/example12.txt");
var input = System.IO.File.ReadAllText("/home/richard/Workspace/advent-of-code/aoc2022/inputs/day6-input.txt");

var e0_part1 = Solver.Part1(example0);
Console.WriteLine($"Example 0: {e0_part1}");

var e6_part1 = Solver.Part1(example6);
Console.WriteLine($"Example 1: {e6_part1}");

var e8_part1 = Solver.Part1(example8);
Console.WriteLine($"Example 2: {e8_part1}");

var e10_part1 = Solver.Part1(example10);
Console.WriteLine($"Example 3: {e10_part1}");

var e12_part1 = Solver.Part1(example12);
Console.WriteLine($"Example 4: {e12_part1}");

Console.WriteLine($"Input: {Solver.Part1(input)}");

var part2Exemple0Input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
Console.Write(part2Exemple0Input);
var part2Exemple0Value = Solver.Part2(part2Exemple0Input);
Console.WriteLine($" {part2Exemple0Value}");

var part2Exemple1Input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
Console.Write(part2Exemple1Input);
var part2Exemple1Value = Solver.Part2(part2Exemple1Input);
Console.WriteLine($": {part2Exemple1Value}");


Console.Write("Part 2 inputs");
var part2Input = Solver.Part2(input);
Console.WriteLine($": {part2Input}");



// Solver.Part2(input);