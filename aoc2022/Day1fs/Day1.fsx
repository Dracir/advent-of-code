open System
open System.IO

let lines = File.ReadAllText("2022/Inputs/Day1.txt")

let calories = 
    lines.Split("\n\n")
    |> Array.map (fun x -> x.Split("\n"))
    |> Array.map (fun x -> x |> Array.map (fun y -> int y))
    |> Array.map (fun x -> x |> Array.sum)

let maxColories = calories |> Array.max

let topThree = calories |> Array.sortDescending |> Array.take 3 |> Array.sum