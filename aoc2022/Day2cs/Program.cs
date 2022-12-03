// Console.WriteLine("Hello, World!");


// var InputFilePath = "/home/richard/Workspace/advent-of-code/aoc2022/inputs/day2-input.txt";
// var input = System.IO.File.ReadAllText(InputFilePath);

// var p1 = Day2Match.Solve_Part1(input);
// var p2 = Day2Match.Solve_Part2(input);

// Console.WriteLine($"Part 1: {p1}");
// Console.WriteLine($"Part 2: {p2}");

// var p1_map = Day2Map.Solve_Part1(input);
// var p2_map = Day2Map.Solve_Part2(input);

// Console.WriteLine($"Part 1: {p1_map}");
// Console.WriteLine($"Part 2: {p2_map}");

// var p1_handshapes = Day2HandShapes.Solve_Part1(input);
// var p2_handshapes = Day2HandShapes.Solve_Part2(input);

// Console.WriteLine($"Part 1: {p1_handshapes}");
// Console.WriteLine($"Part 2: {p2_handshapes}");


var summary = BenchmarkDotNet.Running.BenchmarkRunner.Run<Benchmark>();