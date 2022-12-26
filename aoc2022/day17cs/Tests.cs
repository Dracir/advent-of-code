using NUnit.Framework;


public class Tests
{
	[Test]
	public void After1Round()
	{
		var instructions = Fonctions.ParseInputToDirection(File.ReadAllText("example1.txt"));

		var chamber = new List<int>();
		var top = 0;
		var instructionIndex = instructions.Length - 1;
		var shapeIndex = RockShapes.AllShapes.Count - 1;

		for (int i = 0; i < 1; i++)
		{
			shapeIndex = (shapeIndex + 1) % RockShapes.AllShapes.Count;
			var rockSharp = RockShapes.AllShapes[shapeIndex];
			Fonctions.EnsureSpace(chamber, top + 7);
			(instructionIndex, top) = Fonctions.AddShape(chamber, instructions, rockSharp, instructionIndex: instructionIndex, top: top);
		}
		Assert.AreEqual(1, top);
	}

	[Test]
	public void After2Rounds()
	{
		var instructions = Fonctions.ParseInputToDirection(File.ReadAllText("example1.txt"));

		var chamber = new List<int>();
		var top = 0;
		var instructionIndex = instructions.Length - 1;
		var shapeIndex = RockShapes.AllShapes.Count - 1;

		for (int i = 0; i < 2; i++)
		{
			shapeIndex = (shapeIndex + 1) % RockShapes.AllShapes.Count;
			var rockSharp = RockShapes.AllShapes[shapeIndex];
			Fonctions.EnsureSpace(chamber, top + 7);
			(instructionIndex, top) = Fonctions.AddShape(chamber, instructions, rockSharp, instructionIndex: instructionIndex, top: top);
		}
		Assert.AreEqual(4, top);
	}

	[Test]
	public void After3Rounds()
	{
		var instructions = Fonctions.ParseInputToDirection(File.ReadAllText("example1.txt"));

		var chamber = new List<int>();
		var top = 0;
		var instructionIndex = instructions.Length - 1;
		var shapeIndex = RockShapes.AllShapes.Count - 1;

		for (int i = 0; i < 3; i++)
		{
			shapeIndex = (shapeIndex + 1) % RockShapes.AllShapes.Count;
			var rockSharp = RockShapes.AllShapes[shapeIndex];
			Fonctions.EnsureSpace(chamber, top + 7);
			(instructionIndex, top) = Fonctions.AddShape(chamber, instructions, rockSharp, instructionIndex: instructionIndex, top: top);
		}
		Assert.AreEqual(6, top);
	}

	[Test]
	public void After4Rounds()
	{
		var instructions = Fonctions.ParseInputToDirection(File.ReadAllText("example1.txt"));

		var chamber = new List<int>();
		var top = 0;
		var instructionIndex = instructions.Length - 1;
		var shapeIndex = RockShapes.AllShapes.Count - 1;

		for (int i = 0; i < 4; i++)
		{
			shapeIndex = (shapeIndex + 1) % RockShapes.AllShapes.Count;
			var rockSharp = RockShapes.AllShapes[shapeIndex];
			Fonctions.EnsureSpace(chamber, top + 7);
			(instructionIndex, top) = Fonctions.AddShape(chamber, instructions, rockSharp, instructionIndex: instructionIndex, top: top);
		}
		Assert.AreEqual(7, top);
	}

	public static List<int> TouchMe_TestChamber = new int[] {
		0b0000000,
		0b0000000,
		0b0000000,
		0b0000000,
		0b0001000,
		0b0011100,
		0b0001000,
		0b0111100
		}.Reverse().ToList();

	[TestCase(2, 4, false)]
	[TestCase(2, 3, true)]
	[TestCase(0, 2, true)]
	[TestCase(0, 1, true)]
	public void IsTouching_L(int x, int y, bool expected)
	{
		var res = Fonctions.SpaceTouchsSomethingAt(TouchMe_TestChamber, RockShapes.L, x, y);
		Assert.AreEqual(expected, res);
	}
}