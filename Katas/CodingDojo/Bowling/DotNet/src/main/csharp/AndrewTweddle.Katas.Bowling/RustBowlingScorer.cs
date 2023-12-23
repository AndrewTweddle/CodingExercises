namespace AndrewTweddle.Katas.Bowling;

using System.Runtime.InteropServices;

public class RustBowlingScorer: IBowlingScorer
{
    public int Calculate(string throwSymbols)
    {
        return score_with_rust(throwSymbols);
    }
    
    [DllImport("../../../../../../../../rust_bowl/target/debug/librust_bowl.so")]
    public static extern Int32 score_with_rust([MarshalAs(UnmanagedType.LPUTF8Str)] string throws);
}