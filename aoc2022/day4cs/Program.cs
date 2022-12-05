using static Solvers;

Console.WriteLine("Hello, World!");

// Day3_Char.Solve();

var InputFilePath = "/home/richard/Workspace/advent-of-code/aoc2022/inputs/day4-input.txt";
var input = System.IO.File.ReadAllText(InputFilePath);

// var part1RegexIntTupple = Solvers.Solve_Part1(input, DataType.INT_TUPPLE, Technic.REGEX);
// var part1RegexRange = Solvers.Solve_Part1(input, DataType.RANGE, Technic.REGEX);
// var part1RegexStruct = Solvers.Solve_Part1(input, DataType.STRUCT, Technic.REGEX);

// var part1SplitIntTupple = Solvers.Solve_Part1(input, DataType.INT_TUPPLE, Technic.SPLIT);
// var part1SplitRange = Solvers.Solve_Part1(input, DataType.RANGE, Technic.SPLIT);
// var part1SplitStruct = Solvers.Solve_Part1(input, DataType.STRUCT, Technic.SPLIT);

// var part1SpanIntTupple = Solvers.Solve_Part1(input, DataType.INT_TUPPLE, Technic.SPAN);
// var part1SpanRange = Solvers.Solve_Part1(input, DataType.RANGE, Technic.SPAN);
// var part1SpanStruct = Solvers.Solve_Part1(input, DataType.STRUCT, Technic.SPAN);

var part1UltimateIntTupple = Solvers.UltimateSolution_Part1(input);
var part1Dyke10 = Solvers.Dyke10_Part1(input);

// Console.WriteLine($"Part1 Regex IntTupple: {part1RegexIntTupple}");
// Console.WriteLine($"Part1 Regex Range: {part1RegexRange}");
// Console.WriteLine($"Part1 Regex Struct: {part1RegexStruct}");
// Console.WriteLine($"Part1 Split IntTupple: {part1SplitIntTupple}");
// Console.WriteLine($"Part1 Split Range: {part1SplitRange}");
// Console.WriteLine($"Part1 Split Struct: {part1SplitStruct}");
// Console.WriteLine($"Part1 Span IntTupple: {part1SpanIntTupple}");
// Console.WriteLine($"Part1 Span Range: {part1SpanRange}");
// Console.WriteLine($"Part1 Span Struct: {part1SpanStruct}");

// Console.WriteLine($"Part1 Ultimate IntTupple: {part1UltimateIntTupple}");
// Console.WriteLine($"Part1 Dyke10: {part1Dyke10}");


var summary = BenchmarkDotNet.Running.BenchmarkRunner.Run<Day4Benchmark>();