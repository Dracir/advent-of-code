public static class Solver
{

	public static List<Stack<char>> InputToStacks(string input)
	{
		var stacks = new List<Stack<char>>();

		var lines = input.Split(Environment.NewLine);
		int i = 0;

		var nbStacks = (lines[0].Length + 1) / 4;
		for (int j = 0; j < nbStacks; j++)
			stacks.Add(new Stack<char>());

		while (lines[i].Contains('['))
		{
			var line = lines[i];
			for (int j = 0; j < nbStacks; j++)
			{
				char c = line[j * 4 + 1];
				if (c != ' ')
					stacks[j].Push(c);
			}
			i++;
		}

		for (int s = 0; s < nbStacks; s++)
			stacks[s] = new Stack<char>(stacks[s]);

		return stacks;
	}

	public static List<(int from, int to, int amount)> InputToInstructions(string input)
	{
		return input
			.Split("\n")
			.Select(x => x.Split(" "))
			.Select(x => (int.Parse(x[3]), int.Parse(x[5]), int.Parse(x[1])))
			.ToList();
	}

	public static (List<Stack<char>> stacks, List<(int from, int to, int amount)> instructions) InputToStackInstructions(string input)
	{
		var splited = input.Split("\n\n");
		var stacks = InputToStacks(splited[0]);
		var instructions = InputToInstructions(splited[1]);
		return (stacks, instructions);
	}

	public static string StacksTopToString(List<Stack<char>> stacks)
	{
		return string.Join("", stacks.Select(x => x.Peek()));
	}

	public static void Part1(string input)
	{
		var (stacks, instructions) = InputToStackInstructions(input);

		foreach (var instruction in instructions)
		{
			for (int i = 0; i < instruction.amount; i++)
			{
				stacks[instruction.to - 1].Push(stacks[instruction.from - 1].Pop());
			}
		}

		Console.WriteLine($"Part1 result: {StacksTopToString(stacks)}");

	}

	public static void Part2(string input)
	{
		var (stacks, instructions) = InputToStackInstructions(input);

		foreach (var instruction in instructions)
		{
			var tmp = new Stack<char>();
			for (int i = 0; i < instruction.amount; i++)
				tmp.Push(stacks[instruction.from - 1].Pop());
			for (int i = 0; i < instruction.amount; i++)
				stacks[instruction.to - 1].Push(tmp.Pop());
		}

		Console.WriteLine($"Part2 result: {StacksTopToString(stacks)}");
	}
}