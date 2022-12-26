Console.WriteLine("Hello, World!");
var input = File.ReadAllText("../inputs/day17/me.txt");

var instructions = Fonctions.ParseInputToDirection(input);

var chamber = new List<int>();
var top = 0;
var instructionIndex = instructions.Length - 1;
var shapeIndex = RockShapes.AllShapes.Count - 1;
var rocks = 0;

for (int i = 0; i < 1000000000; i++)
{
	shapeIndex = (shapeIndex + 1) % RockShapes.AllShapes.Count;
	var rockSharp = RockShapes.AllShapes[shapeIndex];
	Fonctions.EnsureSpace(chamber, top + 7);
	var shapeTop = 0;
	(instructionIndex, shapeTop) = Fonctions.AddShape(chamber, instructions, rockSharp, instructionIndex: instructionIndex, top: top);
	rocks += rockSharp.Rocks;
	if (shapeTop > top)
		top = shapeTop;
	// Fonctions.PrintChamberSection(chamber, 0, top + 2);
	// Fonctions.PrintChamberSectionBinary(chamber, 0, top + 2);
}
// Fonctions.PrintChamberSection(chamber, 0, top + 2);

Console.WriteLine($"Rocks: {rocks}");
Console.WriteLine($"Top: {top}");