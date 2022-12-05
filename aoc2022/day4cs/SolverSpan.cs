
public static class SolverSpan
{
	private static (int a1, int a2, int b1, int b2) ParseToIntsWithSpan(string group)
	{
		ReadOnlySpan<char> asSpan = group;
		var firstDash = asSpan.IndexOf('-');
		var firstComma = asSpan.IndexOf(',');
		var secondDash = asSpan.LastIndexOf('-');

		return (
			int.Parse(asSpan.Slice(0, firstDash)),
			int.Parse(asSpan.Slice(firstDash + 1, firstComma - firstDash - 1)),
			int.Parse(asSpan.Slice(firstComma + 1, secondDash - firstComma - 1)),
			int.Parse(asSpan.Slice(secondDash + 1))
		);
	}

	public static bool OneFullyOverlapOther(int a1, int a2, int b1, int b2) => a1 <= b1 && a2 >= b2 || b1 <= a1 && b2 >= a2;
	public static bool OnePartiallyOverlapOther(int a1, int a2, int b1, int b2) => a1 <= b1 && a2 >= b1 || b1 <= a1 && b2 >= a1;

	public static int Solve_Part1(string input) => input
		.Split("\n")
		.Select(ParseToIntsWithSpan)
		.Count(ranges => OneFullyOverlapOther(ranges.a1, ranges.a2, ranges.b1, ranges.b2));

	public static int Solve_Part2(string input) => input
		.Split("\n")
		.Select(ParseToIntsWithSpan)
		.Count(ranges => OnePartiallyOverlapOther(ranges.a1, ranges.a2, ranges.b1, ranges.b2));

}