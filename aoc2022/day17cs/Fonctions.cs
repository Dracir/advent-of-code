using System.Text;

public static class Fonctions
{
	public static bool[] ParseInputToDirection(string text)
	{
		return text.Select(x => x == '>').ToArray();
	}

	public static (int InstructionIndex, int Top) AddShape(List<int> chamber, bool[] instructions, RockShapes shape, int instructionIndex, int top)
	{
		int y = top + 3; // maybe 2 ?
		int x = 2;

		while (y >= 0)
		{
			instructionIndex = (instructionIndex + 1) % instructions.Length;
			int movement = 0;
			if (instructions[instructionIndex])
				movement = 1;
			else
				movement = -1;

			if (x + movement < 0) { }
			else if (x + movement + shape.Width > 7) { }
			else if (SpaceTouchsSomethingAt(chamber, shape, x + movement, y))
			{

			}
			else
			{
				x += movement;
				// Console.WriteLine("Push " + (instructions[instructionIndex] ? "right" : "left"));
			}
			// Fonctions.PrintChamberSectionWithRock(chamber, 0, top + 2, shape, x, y);

			if (y == 0)
				break;
			else if (SpaceTouchsSomethingAt(chamber, shape, x, y - 1))
				break;
			y--;
			// Fonctions.PrintChamberSectionWithRock(chamber, 0, top + 2, shape, x, y);
			// Console.WriteLine("Fall down 1 unit");
		}

		// Console.WriteLine($"x={x}, y={y}");
		for (int i = 0; i < shape.Shape.Length; i++)
		{
			int line = shape.Shape[i];
			int lineY = y + i;

			int lineX = x;
			int mask = line << lineX;
			chamber[lineY] |= mask;
		}
		return (instructionIndex, y + shape.Shape.Length);
	}

	public static bool SpaceTouchsSomethingAt(List<int> chamber, RockShapes rock, int x, int y)
	{
		for (int i = 0; i < rock.Shape.Length; i++)
		{
			int line = rock.Shape[i];
			int lineY = y + i;

			int lineX = x;
			int mask = line << lineX;
			if ((chamber[lineY] & mask) != 0)
			{
				return true;
			}
		}
		return false;
	}

	public static void EnsureSpace(List<int> chamber, int v)
	{
		while (chamber.Count < v)
			chamber.Add(0);
	}

	public static void PrintChamberSection(List<int> chamber, int bottom, int top)
	{
		Console.WriteLine("");
		for (int i = top - 1; i >= bottom; i--)
		{
			var s = Convert.ToString(chamber[i], 2).PadLeft(7, '0').Replace("0", ".").Replace("1", "#").Reverse();
			Console.WriteLine("|" + new String(s.ToArray()) + "|");
		}
		Console.WriteLine("+-------+");
	}

	public static void PrintChamberSectionWithRock(List<int> chamber, int bottom, int top, RockShapes shape, int x, int y)
	{
		var strs = new List<String>();
		Console.WriteLine("");
		for (int i = bottom; i < top - 1; i++)
		{
			var s = Convert.ToString(chamber[i], 2).PadLeft(7, '0').Replace("0", ".").Replace("1", "#").Reverse();
			var str = "|" + new String(s.ToArray()) + "|";
			strs.Add(str);
			// Console.WriteLine("|" + new String(s.ToArray()) + "|");
		}
		strs.Add("|.......| ");
		strs.Add("|.......| ");
		strs.Add("|.......| ");
		strs.Add("|.......| ");
		strs.Add("|.......| ");
		strs.Add("|.......| ");
		strs.Add("|.......| ");
		for (int i = 0; i < shape.Shape.Length; i++)
		{
			int line = shape.Shape[i];
			int lineY = y + i;

			int lineX = x;
			int mask = line << lineX;
			var str = strs[lineY];
			var sb = new StringBuilder(str);
			for (int j = 0; j < 7; j++)
			{
				if ((mask & (1 << j)) != 0)
				{
					sb[j + 1] = '@';
				}
			}
			strs[lineY] = sb.ToString();
		}
		strs.Reverse();
		foreach (var str in strs)
		{
			Console.WriteLine(str);
		}
		Console.WriteLine("+-------+");
	}
	public static void PrintChamberSectionBinary(List<int> chamber, int bottom, int top)
	{
		for (int i = top - 1; i >= bottom; i--)
		{
			var s = Convert.ToString(chamber[i], 2).PadLeft(7, '0');
			Console.WriteLine(new String(s.ToArray()));
		}
	}
}