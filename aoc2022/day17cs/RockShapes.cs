public struct RockShapes
{
	public int[] Shape;
	public int Width;
	public int Rocks;

	public RockShapes(int[] shape, int width, int rocks)
	{
		Shape = shape;
		Width = width;
		Rocks = rocks;
	}

	public static RockShapes Horizontal = new RockShapes(new int[] { 0b1111 }, 4, 4);
	public static RockShapes Plus = new RockShapes(new int[] { 0b010, 0b111, 0b010 }, 3, 5);
	public static RockShapes Vertical = new RockShapes(new int[] { 0b1, 0b1, 0b1, 0b1 }, 1, 4);
	public static RockShapes L = new RockShapes(new int[] { 0b111, 0b100, 0b100, }, 3, 5);
	public static RockShapes Square = new RockShapes(new int[] { 0b11, 0b11 }, 2, 4);

	public static List<RockShapes> AllShapes = new List<RockShapes>()
	{
		Horizontal,
		Plus,
		L,
		Vertical,
		Square
	};

}