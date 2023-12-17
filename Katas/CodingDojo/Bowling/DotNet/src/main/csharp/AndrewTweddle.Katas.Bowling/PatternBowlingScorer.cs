namespace AndrewTweddle.Katas.Bowling;

public class BowlingScorerException: Exception
{
    public BowlingScorerException(string ExceptionMessage): base(ExceptionMessage)
    { }
}

/// <summary>
/// This class implements the IBowlingScorer interface.
/// It could be a static class, were it not for the need to implement that interface.
/// </summary>
public class PatternBowlingScorer: IBowlingScorer
{
    public int Calculate(string throwSymbols)
    {
        char[] throws = throwSymbols.ToCharArray();
        return ScoreRemainingFrames(1, new ReadOnlySpan<char>(throws));
    }

    private int ScoreRemainingFrames(int frame, ReadOnlySpan<char> remainingSymbols)
    {
        if (frame == 10)
        {
            return remainingSymbols switch
            {
                ['X', var throw2, var throw3] => 10 + ScoreNextTwoThrows(throw2, throw3),
                ['/', ..] => throw new BowlingScorerException("Frame 10 starts with a spare"),
                [_, 'X', ..] => throw new BowlingScorerException("Frame 10 has a strike as the second throw"),
                [_, '/', var throw3] => 10 + PinToValue(throw3),
                [var throw1, var throw2] => PinToValue(throw1) + PinToValue(throw2),
                _ => throw new BowlingScorerException($"Unrecognized pattern in 10th frame: {remainingSymbols}")
            };
        }
        return remainingSymbols switch
        {
            ['X', var throw1, var throw2, ..] =>
                10 + ScoreNextTwoThrows(throw1, throw2) +
                ScoreRemainingFrames(frame + 1, remainingSymbols.Slice(1)),
            ['/', ..] => throw new BowlingScorerException($"Frame {frame} has a spare as the first throw"),
            [_, 'X', ..] => throw new BowlingScorerException($"Frame {frame} has a strike as the second throw"),
            [_, '/', var throw1, ..] =>
                10 + PinToValue(throw1) + ScoreRemainingFrames(frame + 1, remainingSymbols.Slice(2)),
            [var throw1, var throw2, .. var newRemainingSymbols] =>
                ScoreNextTwoThrows(throw1, throw2) + ScoreRemainingFrames(frame + 1, newRemainingSymbols),
            _ => throw new BowlingScorerException(
                $"Frame {frame} has an invalid pattern of remaining throws: {remainingSymbols}")
        };
    }
    
    private static int ScoreNextTwoThrows(char pin1, char pin2)
    {
        if (pin2 == '/')
        {
            return 10;
        }
        else
        {
            return PinToValue(pin1) + PinToValue(pin2);
        }
    }

    private static int PinToValue(char pin)
    {
        return pin switch
        {
            '-' => 0,
            '/' or 'X' => 10,
            >= '0' and <= '9' => pin - '0',
            _ => throw new ApplicationException($"Unrecognized character $pin")
        };
    }
}