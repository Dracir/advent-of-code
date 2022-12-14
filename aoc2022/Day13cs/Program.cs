using System.Linq;

public struct ListOrInt
{
	public int SpecialKey;
	public int Value;
	public List<ListOrInt>? List;
	public bool IsValue;

	public ListOrInt(int value)
	{
		SpecialKey = 0;
		Value = value;
		List = null;
		IsValue = true;
	}

	public ListOrInt(List<ListOrInt> list)
	{
		SpecialKey = 0;
		Value = 0;
		List = list;
		IsValue = false;
	}

	public override string ToString()
	{
		if (IsValue)
			return Value.ToString();
		else
			return ListToString(List!);
	}

	public static string ListToString(List<ListOrInt> list)
	{
		var result = "[";
		result += string.Join(", ", list.Select(x => x.ToString()).ToArray());
		result += "]";
		return result;
	}
}

public class Program
{

	public static void Main(string[] args)
	{
		Console.WriteLine("Hello, World!");
		// var input = File. ReadAllText("../inputs/day13/example1.txt");
		var input = File.ReadAllText("../inputs/day13/me.txt");
		var packets = InputToPaquets(input);
		// Part1(packets);
		Part2(packets);

	}

	private static void Part2(List<ListOrInt> packets)
	{
		var decoderA = InputLineToPacket("[[2]]");
		var decoderB = InputLineToPacket("[[6]]");
		decoderA.SpecialKey = 1;
		decoderB.SpecialKey = 2;
		packets.Add(decoderA);
		packets.Add(decoderB);

		packets.Sort((left, right) =>
		{
			var result = IsInRightOrder(left, right, 0);
			if (result == true)
				return -1;
			else if (result == false)
				return 1;
			else
				return 0;
		});
		Console.WriteLine("Sorted packets:");
		foreach (var packet in packets)
		{
			Console.WriteLine(packet);
		}

		var indexOfDecoderA = packets.IndexOf(decoderA) + 1;
		var indexOfDecoderB = packets.IndexOf(decoderB) + 1;
		Console.WriteLine($"Index of decoder A : {indexOfDecoderA}");
		Console.WriteLine($"Index of decoder B : {indexOfDecoderB}");

		var decoderKey = (long)indexOfDecoderA * indexOfDecoderB;
		Console.WriteLine($"Decoder key : {decoderKey}");
	}

	private static void Part1(List<ListOrInt> packets)
	{
		int sumOfValidPackets = 0;
		for (int i = 0; i < packets.Count; i += 2)
		{
			int pairIndex = i / 2 + 1;
			Console.WriteLine($"\n== Pair {pairIndex} ==");
			var left = packets[i];
			var right = packets[i + 1];
			var result = IsInRightOrder(left, right, 0);
			if (result == true)
				sumOfValidPackets += pairIndex;
			Console.WriteLine($"{result}");
		}
		Console.WriteLine($"Sum of valid packets : {sumOfValidPackets}");
	}

	private static bool? IsInRightOrder(ListOrInt left, ListOrInt right, int depth)
	{
		return (left.IsValue, right.IsValue) switch
		{
			(true, true) => CompareValues(left.Value, right.Value, depth + 1),
			(false, false) => IsInRightOrder(left.List!, right.List!, depth + 1),
			(true, false) => MixedTypes(left.Value, right.List!, depth + 1),
			(false, true) => MixedTypes(left.List!, right.Value, depth + 1),
		};
	}

	private static bool? MixedTypes(int left, List<ListOrInt> right, int depth)
	{
		Console.WriteLine($"{new String(' ', 2 * depth)}Compare {left} vs {ListOrInt.ListToString(right)}");
		Console.WriteLine($"{new String(' ', 2 * depth + 2)}Mixed types; convert left to [{left}] and retry comparison");
		return IsInRightOrder(new List<ListOrInt> { new ListOrInt(left) }, right, depth + 1);
	}

	private static bool? MixedTypes(List<ListOrInt> left, int right, int depth)
	{
		Console.WriteLine($"{new String(' ', 2 * depth)}Compare {ListOrInt.ListToString(left)} vs {right}");
		Console.WriteLine($"{new String(' ', 2 * depth + 2)}Mixed types; convert right to [{right}] and retry comparison");
		return IsInRightOrder(left, new List<ListOrInt> { new ListOrInt(right) }, depth + 1);
	}

	private static bool? CompareValues(int left, int right, int depth)
	{
		Console.WriteLine($"{new String(' ', 2 * depth)}Comparing {left} vs {right}");
		if (left == right)
			return null;
		if (left < right)
			Console.WriteLine($"{new String(' ', 2 * (depth + 1))}Left side is smaller, so inputs are in the right order");
		else
			Console.WriteLine($"{new String(' ', 2 * (depth + 1))}Right side is smaller, so inputs are not in the right order");

		return left <= right;
	}

	private static bool? IsInRightOrder(List<ListOrInt> left, List<ListOrInt> right, int depth)
	{
		Console.WriteLine($"{new String(' ', 2 * depth)}Compare {ListOrInt.ListToString(left)} vs {ListOrInt.ListToString(right)}");
		for (int i = 0; i < left.Count; i++)
		{
			if (i >= right.Count)
			{
				Console.WriteLine($"{new String(' ', 2 * (depth + 1))}Right side ran out of items, so inputs are not in the right order");
				return false;
			}
			var result = IsInRightOrder(left[i], right[i], depth + 1);
			if (result != null)
				return result;
		}
		if (left.Count < right.Count)
		{
			Console.WriteLine($"{new String(' ', 2 * (depth + 1))}Left side ran out of items, so inputs are in the right order");
			return true;
		}
		return null;

	}

	//Examples of inputs :
	// [1,[2,[3,[4,[5,6,7]]]],8,9]
	// [[4,4],4,4,4]
	// [[10],[[[8],2,[8]],[],7],[[2,2,[4,6]],5,6],[[],[[],3],[[9],3]],[3]]
	private static List<ListOrInt> InputToPaquets(string input)
	{
		return input.Split("\n").Where(x => x.Length > 0).Select(x => InputLineToPacket(x)).ToList();

	}

	private static ListOrInt InputLineToPacket(string line)
	{
		// Console.WriteLine("Parsing " + line);
		if (char.IsDigit(line[0]))
		{
			//Read an int from 0 to 10
			if (line.Length > 1 && char.IsDigit(line[1]))
				return new ListOrInt(int.Parse(line.Substring(0, 2)));
			else
				return new ListOrInt(int.Parse(line.Substring(0, 1)));

		}
		else if (line[0] == '[')
		{
			if (line[1] == ']')
				return new ListOrInt(new List<ListOrInt>());
			else
				return ReadListOfInts(line.Substring(1, line.Length - 2));
		}
		else
		{
			throw new Exception("Error parsing " + line);
		}
	}

	//Examples 
	// [4,4],4,4,4
	// 4,4
	// 4,[]
	private static ListOrInt ReadListOfInts(string v)
	{
		var result = new List<ListOrInt>();

		var current = "";
		var depth = 0;
		foreach (var c in v)
		{
			if (c == '[')
			{
				depth++;
			}
			else if (c == ']')
			{
				depth--;
			}
			else if (c == ',' && depth == 0)
			{
				result.Add(InputLineToPacket(current));
				current = "";
				continue;
			}
			current += c;
		}
		if (current.Length > 0)
		{
			result.Add(InputLineToPacket(current));
		}

		return new ListOrInt(result);
	}
}