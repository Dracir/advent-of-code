using System.Text.RegularExpressions;

public class Solvers
{
	public enum DataType { INT_TUPPLE, RANGE }
	public enum Technic { SPLIT, REGEX, SPAN }


	private static (int a1, int a2, int b1, int b2) ParseToIntsWithRegex(string group)
	{
		var match = Regex.Match(group, @"(\d+)-(\d+),(\d+)-(\d+)");
		return (int.Parse(match.Groups[1].Value), int.Parse(match.Groups[2].Value), int.Parse(match.Groups[3].Value), int.Parse(match.Groups[4].Value));
	}
	private static (Range a, Range b) ParseToRangeWithRegex(string group)
	{
		var match = Regex.Match(group, @"(\d+)-(\d+),(\d+)-(\d+)");
		return (new Range(int.Parse(match.Groups[1].Value), int.Parse(match.Groups[2].Value)), new Range(int.Parse(match.Groups[3].Value), int.Parse(match.Groups[4].Value)));
	}

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

	private static (Range a, Range b) ParseToRangesWithSpan(string group)
	{
		ReadOnlySpan<char> asSpan = group;
		var firstDash = asSpan.IndexOf('-');
		var firstComma = asSpan.IndexOf(',');
		var secondDash = asSpan.LastIndexOf('-');

		return (
			new Range(int.Parse(asSpan.Slice(0, firstDash)), int.Parse(asSpan.Slice(firstDash + 1, firstComma - firstDash - 1))),
			new Range(int.Parse(asSpan.Slice(firstComma + 1, secondDash - firstComma - 1)), int.Parse(asSpan.Slice(secondDash + 1)))
		);
	}

	private static (Range a, Range b) ParseToRangeWithSubSplit(string group)
	{
		var split = group.Split(',');
		var left = split[0].Split('-');
		var right = split[1].Split('-');
		return (new Range(int.Parse(left[0]), int.Parse(left[1])), new Range(int.Parse(right[0]), int.Parse(right[1])));
	}
	private static (int a1, int a2, int b1, int b2) ParseToIntsWithSplit(string group)
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

	public static int Solve_Part1(string input, DataType dataType, Technic technic)
	{
		switch (dataType)
		{
			case DataType.INT_TUPPLE:
				switch (technic)
				{
					case Technic.SPLIT:
						return input.Split(Environment.NewLine).Select(ParseToIntsWithSplit).Count(x => OneFullyOverlapOther(x.a1, x.a2, x.b1, x.b2));
					case Technic.REGEX:
						return input.Split(Environment.NewLine).Select(ParseToIntsWithRegex).Count(x => OneFullyOverlapOther(x.a1, x.a2, x.b1, x.b2));
					case Technic.SPAN:
						return input.Split(Environment.NewLine).Select(ParseToIntsWithSpan).Count(x => OneFullyOverlapOther(x.a1, x.a2, x.b1, x.b2));
				}
				break;

			case DataType.RANGE:
				switch (technic)
				{
					case Technic.SPLIT:
						return input.Split(Environment.NewLine).Select(ParseToRangeWithSubSplit).Count(x => OneFullyOverlapOther(x.a, x.b));
					case Technic.REGEX:
						return input.Split(Environment.NewLine).Select(ParseToRangeWithRegex).Count(x => OneFullyOverlapOther(x.a, x.b));
					case Technic.SPAN:
						return input.Split(Environment.NewLine).Select(ParseToRangesWithSpan).Count(x => OneFullyOverlapOther(x.a, x.b));
				}
				break;
		}
		return 0;
	}

	public static int UltimateSolution_Part1(string input)
	{
		ReadOnlySpan<char> fullInputSpan = input;
		int overlaps = 0;
		foreach (var line in fullInputSpan.EnumerateLines())
		{
			var commaIndex = line.IndexOf(',');
			var firstDashIndex = line.IndexOf('-');
			var secondDashIndex = line.LastIndexOf('-');
			int a1 = int.Parse(line.Slice(0, firstDashIndex));
			int a2 = int.Parse(line.Slice(firstDashIndex + 1, commaIndex - firstDashIndex - 1));
			int b1 = int.Parse(line.Slice(commaIndex + 1, secondDashIndex - commaIndex - 1));
			int b2 = int.Parse(line.Slice(secondDashIndex + 1));
			if (a1 <= b1 && a2 >= b2 || b1 <= a1 && b2 >= a2)
			{
				overlaps++;
			}
		}

		return overlaps;

	}

	public static int Dyke10_Part1(string input)
	{
		ReadOnlySpan<char> fullInputSpan = input;
		int overlaps = 0;
		var elf1 = new HashSet<int>();
		var elf2 = new HashSet<int>();

		foreach (var line in fullInputSpan.EnumerateLines())
		{
			var commaIndex = line.IndexOf(',');
			var firstDashIndex = line.IndexOf('-');
			var secondDashIndex = line.LastIndexOf('-');
			int a1 = int.Parse(line.Slice(0, firstDashIndex));
			int a2 = int.Parse(line.Slice(firstDashIndex + 1, commaIndex - firstDashIndex - 1));
			int b1 = int.Parse(line.Slice(commaIndex + 1, secondDashIndex - commaIndex - 1));
			int b2 = int.Parse(line.Slice(secondDashIndex + 1));

			elf1.Clear();
			for (int i = a1; i <= a2; i++)
				elf1.Add(i);

			elf2.Clear();
			for (int i = b1; i <= b2; i++)
				elf2.Add(i);

			if (elf1.IsSubsetOf(elf2) || elf2.IsSubsetOf(elf1))
			{
				overlaps++;
			}
		}

		return overlaps;
	}
}