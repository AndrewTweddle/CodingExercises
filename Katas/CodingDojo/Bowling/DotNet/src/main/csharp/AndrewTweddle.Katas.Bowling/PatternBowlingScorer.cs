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

    /// <summary>
    /// Score remaining frames from a particular frame number (from 1 to 10) onwards.
    /// </summary>
    /// <param name="frame">The frame number of the first of the remaining frames</param>
    /// <param name="remainingSymbols">A slice of remaining throws (char symbols)</param>
    /// <returns>The score of the remaining frames</returns>
    /// <exception cref="BowlingScorerException">Thrown when an invalid pattern of symbols is encountered</exception>
    private int ScoreRemainingFrames(int frame, ReadOnlySpan<char> remainingSymbols)
    {
        return remainingSymbols switch
        {
            _ when frame > 10 => throw new BowlingScorerException("Too many frames"), 
            ['X', var throw2, var throw3] when frame == 10 => 10 + ScoreNextTwoThrows(throw2, throw3),
            ['X', var throw1, var throw2, ..] =>
                10 + ScoreNextTwoThrows(throw1, throw2) +
                ScoreRemainingFrames(frame + 1, remainingSymbols.Slice(1)),
            ['/', ..] => throw new BowlingScorerException($"Frame {frame} has a spare as the first throw"),
            [_, 'X', ..] => throw new BowlingScorerException($"Frame {frame} has a strike as the second throw"),
            [_, '/', var throw3] when frame == 10 => 10 + PinToValue(throw3),
            [_, '/', var throw3, ..] =>
                10 + PinToValue(throw3) + ScoreRemainingFrames(frame + 1, remainingSymbols.Slice(2)),
            [var throw1, var throw2] when frame == 10 => PinToValue(throw1) + PinToValue(throw2),
            [var throw1, var throw2, .. var remainingThrows] =>
                ScoreNextTwoThrows(throw1, throw2) + ScoreRemainingFrames(frame + 1, remainingThrows),
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
            // A spare should never be evaluated on its own, so don't match it.
            'X' => 10,
            >= '0' and <= '9' => pin - '0',
            _ => throw new ApplicationException($"Unrecognized character {pin}")
        };
    }
}