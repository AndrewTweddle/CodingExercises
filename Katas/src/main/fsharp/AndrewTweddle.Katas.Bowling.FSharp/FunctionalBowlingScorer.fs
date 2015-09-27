namespace AndrewTweddle.Katas.Bowling.FSharp

open AndrewTweddle.Katas.Bowling

module Scoring =
    let getPinsDown symbol =
      match symbol with
      | '-' -> 0
      | '1' -> 1
      | '2' -> 2
      | '3' -> 3
      | '4' -> 4
      | '5' -> 5
      | '6' -> 6
      | '7' -> 7
      | '8' -> 8
      | '9' -> 9
      | 'X' -> 10
      | _ -> failwith (sprintf "Unrecognised symbol %c" symbol)

    let sumOfFirstTwoThrows symbols =
      match symbols with
      | _ :: '/' :: _ -> 10
      | throw1 :: throw2 :: _ -> getPinsDown throw1 + getPinsDown throw2
      | _ -> failwith "Insufficient remaining throws"

    let rec calculateRemainingFrames symbols frame totalScore =
        match symbols with
        | 'X' :: remainingSymbols when frame = 10
            -> totalScore + 10 + (sumOfFirstTwoThrows remainingSymbols)
        | _ :: '/' :: remainingSymbol :: [] when frame = 10
            -> totalScore + 10 + (getPinsDown remainingSymbol)
        | lastFrameThrows when frame = 10
            -> totalScore + sumOfFirstTwoThrows lastFrameThrows
        | 'X' :: remainingSymbols
            -> calculateRemainingFrames remainingSymbols (frame+1)
                (totalScore + 10 + (sumOfFirstTwoThrows remainingSymbols))
        | _ :: '/' :: remainingSymbols
            -> calculateRemainingFrames remainingSymbols (frame+1)
                (totalScore + 10 + (getPinsDown remainingSymbols.Head))
        | _ :: _ :: remainingSymbols
            -> calculateRemainingFrames remainingSymbols (frame+1)
                (totalScore + (sumOfFirstTwoThrows symbols))
        | _ -> failwith (sprintf "No remaining symbols in frame %d" frame)

open Scoring

type FunctionalBowlingScorer() = 
    interface IBowlingScorer with
        member this.Calculate(throwSymbols) =
            let symbols = [for c in throwSymbols -> c]
            calculateRemainingFrames symbols 1 0
