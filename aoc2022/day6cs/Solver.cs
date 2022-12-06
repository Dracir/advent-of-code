public static class Solver
{
	private static int SolveFor(string input, int markerLength)
	{
		var marker = new List<char>(input.Substring(0, markerLength - 1));

		for (int i = markerLength - 1; i < input.Length; i++)
		{
			marker.Add(input[i]);
			if (marker.Distinct().Count() == markerLength)
				return i + 1;
			marker.RemoveAt(0);
		}
		return 0;
	}

	public static int Part1(string input) => SolveFor(input, 4);
	public static int Part2(string input) => SolveFor(input, 14);
}