public static class Day2HandShapes
{
	public static int handShapeToPoint(char handShape) => handShape switch
	{
		'A' => 1,
		'B' => 2,
		'C' => 3,
		'X' => 1,
		'Y' => 2,
		'Z' => 3,
		_ => throw new ArgumentException("Invalid hand shape", nameof(handShape))
	};

	public static int handShapeThatKills(int handShapePoint) => handShapePoint switch
	{
		1 => 2,
		2 => 3,
		3 => 1,
		_ => throw new ArgumentException("Invalid hand shape point", nameof(handShapePoint))
	};

	public static int Strategy1(int evil, int good)
	{
		if (evil == good)
			return 3 + good;
		else if (handShapeThatKills(evil) == good)
			return 6 + good;
		else
			return 0 + good;
	}

	public static int Strategy2(int evil, int good)
	{
		if (good == 1)
			return 0 + handShapeThatKills(handShapeThatKills(evil));
		else if (good == 2)
			return 3 + evil;
		else
			return 6 + handShapeThatKills(evil);
	}

	public static int Solve_Part1(string input)
	{
		return input.Split("\n")
			.Select(game => (handShapeToPoint(game[0]), handShapeToPoint(game[2])))
			.Select(game => Strategy1(game.Item1, game.Item2))
			.Sum();
	}

	public static int Solve_Part2(string input)
	{
		return input.Split("\n")
			.Select(game => (handShapeToPoint(game[0]), handShapeToPoint(game[2])))
			.Select(game => Strategy2(game.Item1, game.Item2))
			.Sum();
	}
}