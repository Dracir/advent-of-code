
public static class SolverSubString
{
	private static (Range a, Range b) ParseToRange(string group)
	{
		var split = group.Split(',');
		var left = split[0].Split('-');
		var right = split[1].Split('-');
		return (new Range(int.Parse(left[0]), int.Parse(left[1])), new Range(int.Parse(right[0]), int.Parse(right[1])));
	}
	private static (int a1, int a2, int b1, int b2) ParseToInts(string group)
	{
		var split = group.Split(',');
		var left = split[0].Split('-');
		var right = split[1].Split('-');
		return (int.Parse(left[0]), int.Parse(left[1]), int.Parse(right[0]), int.Parse(right[1]));
	}

	public static bool OneFullyOverlapOther(Range a, Range b) => a.Start.Value <= b.Start.Value && a.End.Value >= b.End.Value || b.Start.Value <= a.Start.Value && b.End.Value >= a.End.Value;

	public static bool OneFullyOverlapOther(int a1, int a2, int b1, int b2) => a1 <= b1 && a2 >= b2 || b1 <= a1 && b2 >= a2;

	public static bool OnePartiallyOverlapOther(Range a, Range b) => a.Start.Value <= b.Start.Value && a.End.Value >= b.Start.Value || b.Start.Value <= a.Start.Value && b.End.Value >= a.Start.Value;

	public static bool OnePartiallyOverlapOther(int a1, int a2, int b1, int b2) => a1 <= b1 && a2 >= b1 || b1 <= a1 && b2 >= a1;

	public static int Solve_Part1Range(string input) => input
		.Split("\n")
		.Select(ParseToRange)
		.Count(ranges => OneFullyOverlapOther(ranges.a, ranges.b));

	public static int Solve_Part1(string input) => input
		.Split("\n")
		.Select(ParseToInts)
		.Count(ranges => OneFullyOverlapOther(ranges.a1, ranges.a2, ranges.b1, ranges.b2));

	public static int Solve_Part2Range(string input) => input
		.Split("\n")
		.Select(ParseToRange)
		.Count(ranges => OnePartiallyOverlapOther(ranges.a, ranges.b));

	public static int Solve_Part2(string input) => input
		.Split("\n")
		.Select(ParseToInts)
		.Count(ranges => OnePartiallyOverlapOther(ranges.a1, ranges.a2, ranges.b1, ranges.b2));

}