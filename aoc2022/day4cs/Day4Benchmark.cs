using System.Text;
using BenchmarkDotNet.Attributes;
using static Solvers;

[MemoryDiagnoser]
public class Day4Benchmark
{
	// [Params("/home/richard/Workspace/advent-of-code/aoc2022/inputs/day4-input.txt")]
	public string InputFilePath = "/home/richard/Workspace/advent-of-code/aoc2022/inputs/day4-input.txt";

	public IEnumerable<object[]> FirstBenchmarkArguments()
	{
		foreach (var arg0 in new[] { DataType.INT_TUPPLE, DataType.RANGE })
		{
			foreach (var arg1 in new[] { Technic.SPLIT, Technic.REGEX, Technic.SPAN })
			{
				yield return new object[] { arg0, arg1 };
			}
		}
	}

	private string _input = "";

	[GlobalSetup]
	public void Setup()
	{
		_input = System.IO.File.ReadAllText(InputFilePath);
	}

	[Benchmark]
	[ArgumentsSource(nameof(FirstBenchmarkArguments))]
	public int Part1(DataType dataTypeUsed, Technic technicUsed)
	{
		return Solvers.Solve_Part1(_input, dataTypeUsed, technicUsed);
	}

	[Benchmark]
	public int UltimateSolution_Part1()
	{
		return Solvers.UltimateSolution_Part1(_input);
	}

	[Benchmark]
	public int Dyke10_Part1()
	{
		return Solvers.Dyke10_Part1(_input);
	}




}