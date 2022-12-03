
public static class Day2Match
{
	public static int Strategy1(string game) => game switch
	{
		//Win
		"A Y" => 6 + 2,
		"B Z" => 6 + 3,
		"C X" => 6 + 1,
		// Tie
		"A X" => 3 + 1,
		"B Y" => 3 + 2,
		"C Z" => 3 + 3,
		// Loose
		"A Z" => 0 + 3,
		"B X" => 0 + 1,
		"C Y" => 0 + 2,
		_ => 0
	};
	public static int Strategy2(string game) => game switch
	{
		//Win
		"A Z" => 6 + 2,
		"B Z" => 6 + 3,
		"C Z" => 6 + 1,
		// Tie
		"A Y" => 3 + 1,
		"B Y" => 3 + 2,
		"C Y" => 3 + 3,
		// Loose
		"A X" => 0 + 3,
		"B X" => 0 + 1,
		"C X" => 0 + 2,
		_ => 0
	};

	public static int Solve_Part1(string input)
	{
		return input.Split("\n")
			.Select(Strategy1)
			.Sum();
	}

	public static int Solve_Part2(string input)
	{
		return input.Split("\n")
			.Select(Strategy2)
			.Sum();
	}
}