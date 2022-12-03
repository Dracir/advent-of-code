let inputs = System.IO.File.ReadLines("inputs/day2-input.txt")

let strategy1 = Map[
    ("A Y", 6 + 2); ("B Z", 6 + 3); ("C X", 6 + 1); // Win
    ("A X", 3 + 1); ("B Y", 3 + 2); ("C Z", 3 + 3); // Tie
    ("A Z", 0 + 3); ("B X", 0 + 1); ("C Y", 0 + 2); // Loose
]

let strategy2 = Map[
    ("A Z", 6 + 2); ("B Z", 6 + 3); ("C Z", 6 + 1); // Win
    ("A Y", 3 + 1); ("B Y", 3 + 2); ("C Y", 3 + 3); // Tie
    ("A X", 0 + 3); ("B X", 0 + 1); ("C X", 0 + 2); // Loose
    ]

inputs |> Seq.map (fun game -> strategy1.[game]) |> Seq.sum |> printfn "Strategy 1: %d"
inputs |> Seq.map (fun game -> strategy2.[game]) |> Seq.sum |> printfn "Strategy 2: %d"