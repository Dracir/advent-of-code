public static class Day2Map
{
	public static Dictionary<string, int> Strategy1 = new Dictionary<string, int>()
	{
		{"A Y", 6 + 2}, {"B Z", 6 + 3}, {"C X", 6 + 1}, //Win
		{"A X", 3 + 1}, {"B Y", 3 + 2}, {"C Z", 3 + 3}, //Tie
		{"A Z", 0 + 3}, {"B X", 0 + 1}, {"C Y", 0 + 2}, //Loose
	};

	public static Dictionary<string, int> Strategy2 = new Dictionary<string, int>()
	{
		{"A Z", 6 + 2}, {"B Z", 6 + 3}, {"C Z", 6 + 1}, //Win
		{"A Y", 3 + 1}, {"B Y", 3 + 2}, {"C Y", 3 + 3}, //Tie
		{"A X", 0 + 3}, {"B X", 0 + 1}, {"C X", 0 + 2}, //Loose
	};

	public static int Solve_Part1(string input)
	{
		return input.Split("\n")
			.Select(x => Strategy1[x])
			.Sum();
	}

	public static int Solve_Part2(string input)
	{
		return input.Split("\n")
			.Select(x => Strategy2[x])
			.Sum();
	}
}