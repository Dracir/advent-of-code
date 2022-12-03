
public static class Day3_Char
{
	public static void Solve()
	{
		// var input = File.ReadAllLines("../examples/day3/exemple2.txt");
		// var input = File.ReadAllLines("../examples/day3/test1.txt").Select(x => x.Trim());
		var input = File.ReadAllLines("../inputs/day3-input.txt");

		var splitInHalf = (string x) => (x.Substring(0, x.Length / 2).ToCharArray(), x.Substring(x.Length / 2).ToCharArray());
		var itemToPriority = (char c) => char.IsLower(c) ? c - 'a' + 1 : c - 'A' + 27;

		var part1 = input
			.Select(splitInHalf)
			.Select(x => x.Item1.Intersect(x.Item2).ToArray())
			.Select(x => itemToPriority(x[0]))
			.Sum();
		Console.WriteLine($"Part 1: {part1}");

	}

	public static int Solve_Part2(string[] input)
	{
		var itemToPriority = (char c) => char.IsLower(c) ? c - 'a' + 1 : c - 'A' + 27;
		var sacksIntersect = (char[] a, char[] b, char[] c) =>
			{
				var hash = new HashSet<char>(a);
				hash.IntersectWith(b);
				hash.IntersectWith(c);
				return hash.First();
			};

		var rucksacks = input
			.Select(x => x.ToCharArray())
			.ToList();

		var part2 = Enumerable.Range(0, rucksacks.Count / 3)
			.Select(i => (rucksacks[i * 3], rucksacks[i * 3 + 1], rucksacks[i * 3 + 2]))
			.Select(sacks => sacksIntersect(sacks.Item1, sacks.Item2, sacks.Item3))
			.Select(itemToPriority)
			.Sum();

		return part2;
	}

}