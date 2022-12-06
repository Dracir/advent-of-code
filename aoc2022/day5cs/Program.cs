using System.Text;

Console.WriteLine("Hello, World!");

// var InputFilePath = "/home/richard/Workspaces/advent-of-code/aoc2022/examples/day5/example0.txt";
var InputFilePath = "/home/richard/Workspaces/advent-of-code/aoc2022/inputs/day5-input.txt";
var input = System.IO.File.ReadAllText(InputFilePath);

Solver.Part1(input);
Solver.Part2(input);