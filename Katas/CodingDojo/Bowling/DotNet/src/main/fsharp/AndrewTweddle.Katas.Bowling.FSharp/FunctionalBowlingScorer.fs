namespace AndrewTweddle.Katas.Bowling.FSharp

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
        | _ when frame > 10 -> failwith "Too many frames"
        | 'X' :: bonusThrows & [_; _; _] when frame = 10
            -> totalScore + 10 + (sumOfFirstTwoThrows bonusThrows)
        | _ :: '/' :: bonusThrow :: [] when frame = 10
            -> totalScore + 10 + (getPinsDown bonusThrow)
        | lastFrameThrows & [_; _] when frame = 10
            -> totalScore + sumOfFirstTwoThrows lastFrameThrows
        | 'X' :: remainingThrows
            -> calculateRemainingFrames remainingThrows (frame+1)
                (totalScore + 10 + (sumOfFirstTwoThrows remainingThrows))
        | _ :: '/' :: remainingThrows
            -> calculateRemainingFrames remainingThrows (frame+1)
                (totalScore + 10 + (getPinsDown remainingThrows.Head))
        | _ :: _ :: remainingThrows
            -> calculateRemainingFrames remainingThrows (frame+1)
                (totalScore + (sumOfFirstTwoThrows symbols))
        | _ -> failwith (sprintf "Invalid pattern in frame %d" frame )

open Scoring
open AndrewTweddle.Katas.Bowling

type FunctionalBowlingScorer() = 
    interface IBowlingScorer with
        member this.Calculate(throwSymbols) =
            let symbols = [for c in throwSymbols -> c]
            calculateRemainingFrames symbols 1 0
