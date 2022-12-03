printfn "Hello from F#"

let inputs = System.IO.File.ReadLines("examples/day2/exemple6.txt")
// let inputs = System.IO.File.ReadLines("inputs/day2-input.txt")

let handShapeToPoint(handShape:char) = 
    match handShape with
    | 'A' | 'X' -> 1
    | 'B'| 'Y' -> 2
    | 'C'| 'Z' -> 3
    | _ -> 0

let handShapeThatKills(handShape:int) = 
    match handShape with
    | 1 -> 2
    | 2 -> 3
    | 3 -> 1
    | _ -> 0

let strategy1 (evil:int, good:int) : int =
    if evil = good then
        3 + good
    elif handShapeThatKills(evil) = good then
        6 + good
    else
        0 + good

let strategy2 (evil:int, good:int) : int =
    if good = 1 then 
        0 + handShapeThatKills(handShapeThatKills(evil))
    elif good = 2 then //Draw
        3 + evil
    else
        6 + handShapeThatKills(evil)
        
let evilVSGood = inputs |> Seq.map (fun game -> handShapeToPoint(game.[0]) , handShapeToPoint(game.[2]))
evilVSGood |> Seq.map (strategy1) |> Seq.sum |> printfn "Strategy 1: %d"
evilVSGood |> Seq.map (strategy2) |> Seq.sum |> printfn "Strategy 2: %d"
