namespace AndrewTweddle.Katas.Bowling;

using System.Runtime.InteropServices;

public class RustBowlingScorer: IBowlingScorer
{
    public int Calculate(string throwSymbols)
    {
        return score_with_rust(throwSymbols);
        // TODO: Check for negative scores (signalling an error), and throw a suitable exception
    }
    
    [DllImport("../../../../../../../../rust_bowl/target/debug/librust_bowl.so")]
    static extern Int32 score_with_rust([MarshalAs(UnmanagedType.LPUTF8Str)] string throws);
}