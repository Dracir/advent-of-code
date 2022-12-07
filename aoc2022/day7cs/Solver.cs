
public record class ElfDirectory(string Name, ElfDirectory? parent, Dictionary<string, ElfDirectory> Directories, Dictionary<string, ElfFile> Files)
{
	public ElfDirectory(string Name, ElfDirectory? parent) : this(Name, parent, new Dictionary<string, ElfDirectory>(), new Dictionary<string, ElfFile>()) { }
}
public record class ElfFile(string Name, int Size);
public record class CommandAndOutput(string Command, string Argument, string[] Output);

public static class Solver
{

	public static CommandAndOutput[] InputToCommandOutputs(string input)
	{
		int i = 0;
		var lines = input.Split("\n");

		var commandsAndOutputs = new List<CommandAndOutput>();
		var currentoutput = new List<string>();

		while (i < lines.Length)
		{
			// if (!lines[i].StartsWith("$"))
			// 	throw new System.Exception("WHAT");
			var split = lines[i].Split(" ");

			if (split[1] == "cd")
				commandsAndOutputs.Add(new CommandAndOutput("cd", split[2], new string[0]));
			else // ASS-ume it's ls
			{
				currentoutput.Clear();
				while (i + 1 < lines.Length && !lines[i + 1].StartsWith("$"))
				{
					currentoutput.Add(lines[i + 1]);
					i++;
				}
				commandsAndOutputs.Add(new CommandAndOutput("ls", "", currentoutput.ToArray()));
			}
			i++;
		}
		return commandsAndOutputs.ToArray();
	}

	public static string PWD(ElfDirectory directory)
	{
		if (directory.parent == null)
			return "/";
		return PWD(directory.parent) + directory.Name + "/";
	}
	public static ElfDirectory InputToElfDirectory(string input)
	{
		ElfDirectory root = new ElfDirectory("/", null);
		ElfDirectory currentDirectory = root;

		var commandsAndOutputs = InputToCommandOutputs(input);
		for (int i = 0; i < commandsAndOutputs.Length; i++)
		{
			// Console.ForegroundColor = ConsoleColor.Blue;
			// Console.Write($"{PWD(currentDirectory)}");
			// Console.ForegroundColor = ConsoleColor.White;
			// Console.WriteLine($"$ {commandsAndOutputs[i].Command} {commandsAndOutputs[i].Argument}");

			var commandAndOutputs = commandsAndOutputs[i];
			if (commandAndOutputs.Command == "cd")
			{
				if (commandAndOutputs.Argument == "..")
				{
					if (currentDirectory.parent != null)
						currentDirectory = currentDirectory.parent;
				}
				else if (commandAndOutputs.Argument == "/")
				{
					currentDirectory = root;
				}
				else
				{
					currentDirectory = currentDirectory.Directories[commandAndOutputs.Argument];
				}
			}
			else
			{ //It's an LS !
				foreach (var line in commandAndOutputs.Output)
				{
					var split = line.Split(" ");
					if (split[0] == "dir")
					{
						if (!currentDirectory.Directories.ContainsKey(split[1]))
							currentDirectory.Directories.Add(split[1], new ElfDirectory(split[1], currentDirectory));
					}
					else
					{
						if (!currentDirectory.Files.ContainsKey(split[1]))
							currentDirectory.Files.Add(split[1], new ElfFile(split[1], int.Parse(split[0])));
					}
				}
			}
		}

		return root;
	}

	public static int DirectoryTotalSize(ElfDirectory directory)
	{
		int total = 0;
		foreach (var file in directory.Files)
			total += file.Value.Size;
		foreach (var dir in directory.Directories)
			total += DirectoryTotalSize(dir.Value);
		return total;
	}

	public static IEnumerable<ElfDirectory> GetAllDirectoriesFlat(ElfDirectory directory)
	{
		foreach (var dir in directory.Directories)
		{
			yield return dir.Value;
			foreach (var subDir in GetAllDirectoriesFlat(dir.Value))
				yield return subDir;
		}
	}

	public static void PrintDirectory(ElfDirectory directory, int depth = 0)
	{
		Console.WriteLine($"{new string(' ', depth * 2)}{directory.Name}");
		foreach (var dir in directory.Directories)
			PrintDirectory(dir.Value, depth + 1);
	}

	public static int Part1(string input)
	{
		var root = InputToElfDirectory(input);
		// PrintDirectory(root);
		return GetAllDirectoriesFlat(root)
		.Select(x => DirectoryTotalSize(x))
		.Where(x => x <= 100000)
		.Sum();
	}
	public static int Part2(string input)
	{
		var root = InputToElfDirectory(input);
		var totalSpace = DirectoryTotalSize(root);
		var freeSpace = 70000000 - totalSpace;
		var neededSpace = 30000000 - freeSpace;
		return GetAllDirectoriesFlat(root)
		.Select(x => DirectoryTotalSize(x))
		.Where(x => x >= neededSpace)
		.Min();
	}
}
