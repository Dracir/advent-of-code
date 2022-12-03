using System.Text;
using BenchmarkDotNet.Attributes;

[MemoryDiagnoser]
public class Day3Benchmark
{
	[Params("/home/richard/Workspace/advent-of-code/aoc2022/inputs/day3-input.txt")]
	public string InputFilePath = "";

	private string[] _input = new string[0];

	[GlobalSetup]
	public void Setup()
	{
		_input = System.IO.File.ReadAllLines(InputFilePath);
	}

	[Benchmark]
	public int Solve_Part2_Bitmap()
	{
		return Day3_Bitmap.Solve_Part2(_input);
	}

	[Benchmark]
	public int Solve_Part2_Char()
	{
		return Day3_Char.Solve_Part2(_input);
	}
}