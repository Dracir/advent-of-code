printfn "Hello from F#"

// let inputs = System.IO.File.ReadLines("inputs/day2-exemple1.txt")
let inputs = System.IO.File.ReadLines("inputs/day2-input.txt")

let strategy1 (game:string) : int =
    match (game) with
    //Win
    | "A Y" -> 6 + 2
    | "B Z" -> 6 + 3
    | "C X" -> 6 + 1
    // Tie
    | "A X" -> 3 + 1
    | "B Y" -> 3 + 2
    | "C Z" -> 3 + 3
    // Loose
    | "A Z" -> 0 + 3
    | "B X" -> 0 + 1
    | "C Y" -> 0 + 2
    | _ -> 0


let strategy2 (game:string) : int =
    match (game) with
    //Win
    | "A Z" -> 6 + 2
    | "B Z" -> 6 + 3
    | "C Z" -> 6 + 1
    // Tie
    | "A Y" -> 3 + 1
    | "B Y" -> 3 + 2
    | "C Y" -> 3 + 3
    // Loose
    | "A X" -> 0 + 3
    | "B X" -> 0 + 1
    | "C X" -> 0 + 2
    | _ -> 0


inputs |> Seq.map (strategy1) |> Seq.sum |> printfn "Strategy 1: %d"
inputs |> Seq.map (strategy2) |> Seq.sum |> printfn "Strategy 2: %d"
