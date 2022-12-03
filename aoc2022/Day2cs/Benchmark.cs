using System.Text;
using BenchmarkDotNet.Attributes;

[MemoryDiagnoser]
public class Benchmark
{
	[Params("/home/richard/Workspace/advent-of-code/aoc2022/inputs/day2-input.txt")]
	public string InputFilePath = "";

	private string _input = "";

	[GlobalSetup]
	public void Setup()
	{
		_input = System.IO.File.ReadAllText(InputFilePath);
	}


	[Benchmark]
	public int Day2Match_Part1()
	{
		return Day2Match.Solve_Part1(_input);
	}

	[Benchmark]
	public int Day2Map_Part1()
	{
		return Day2Map.Solve_Part1(_input);
	}

	[Benchmark]
	public int Day2HandShapes_Part1()
	{
		return Day2HandShapes.Solve_Part1(_input);
	}


	[Benchmark]
	public int Day2Match_Part2()
	{
		return Day2Match.Solve_Part2(_input);
	}
	[Benchmark]
	public int Day2Map_Part2()
	{
		return Day2Map.Solve_Part2(_input);
	}
	[Benchmark]
	public int Day2HandShapes_Part2()
	{
		return Day2HandShapes.Solve_Part2(_input);
	}
}