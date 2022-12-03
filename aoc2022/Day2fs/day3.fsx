
printfn "Hello Day 3"

let inputs = System.IO.File.ReadLines("examples/day3/exemple2.txt")
// let inputs = System.IO.File.ReadLines("inputs/day3-input.txt")

let characterToIndex (character:char) = 
    (int(character) - int('A') + 26) % 52

let indexesToBitMask (index:int[]) : uint64=
    index |> Seq.map (fun i -> uint64(1) <<< i) |> Seq.sum

let charactersToIndex(text:string) =
    text|> Seq.map characterToIndex |> Seq.toArray

// inputs |> Seq.map charactersToIndex  |> Seq.map indexesToBitMask |>  Seq.toArray |> printfn "%A"

inputs |> Seq.map (Seq.splitInto 2) |> printfn "%A"

let findDuplicate( text:char[]) =
    let distinctCharacters = 
        text |> Seq.distinct |> Seq.toArray
    distinctCharacters
    //text |> Seq.filter (fun c -> !distinctCharacters.contains c )
