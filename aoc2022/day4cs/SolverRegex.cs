
using System.Text.RegularExpressions;

public static class SolverRegex
{
	private static (int a1, int a2, int b1, int b2) ParseToIntsWithRegex(string group)
	{
		var match = Regex.Match(group, @"(\d+)-(\d+),(\d+)-(\d+)");
		return (int.Parse(match.Groups[1].Value), int.Parse(match.Groups[2].Value), int.Parse(match.Groups[3].Value), int.Parse(match.Groups[4].Value));
	}

	public static bool OneFullyOverlapOther(int a1, int a2, int b1, int b2) => a1 <= b1 && a2 >= b2 || b1 <= a1 && b2 >= a2;
	public static bool OnePartiallyOverlapOther(int a1, int a2, int b1, int b2) => a1 <= b1 && a2 >= b1 || b1 <= a1 && b2 >= a1;

	public static int Solve_Part1(string input) => input
		.Split("\n")
		.Select(ParseToIntsWithRegex)
		.Count(ranges => OneFullyOverlapOther(ranges.a1, ranges.a2, ranges.b1, ranges.b2));

	public static int Solve_Part2(string input) => input
		.Split("\n")
		.Select(ParseToIntsWithRegex)
		.Count(ranges => OnePartiallyOverlapOther(ranges.a1, ranges.a2, ranges.b1, ranges.b2));

}