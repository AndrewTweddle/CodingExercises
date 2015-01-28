using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace AndrewTweddle.CodingExercises.CTCI.Chapter5.Exercise1
{
    public static class BitInserter
    {
        /// <summary>
        /// Replace certain bit positions in a number n with the bits in number m
        /// starting at bit position i and ending at bit position j
        /// </summary>
        /// <param name="m">The number to be inserted</param>
        /// <param name="n">The number, some of whose bits are to be replaced by the bits in m</param>
        /// <param name="i">The starting position in n where m's bits will be inserted 
        /// i.e. the index of the lowest bit to be replaced
        /// </param>
        /// <param name="j">The ending position in n where m's bits will be inserted up to
        /// i.e. the index of the highest bit to be replaced
        /// </param>
        /// <returns>n after the relevant bits have been replaced by the bits in m</returns>
        public static int InsertBits(int m, int n, short i, short j)
        {
            // Check inputs:
            if (i < 0 || i > 31)
            {
                throw new ArgumentException("i must be a valid bit position in a 32 bit integer", "i");
            }

            if (j < 0 || j > 31)
            {
                throw new ArgumentException(
                    "j must be a valid bit position in a 32 bit integer", "j");
            }

            if (j < i)
            {
                throw new ArgumentException(
                    "i must not be greater than j as i is the starting bit position", "i");
            }

            // Get the bit mask for the insertion bits:
            int bitMask = 0;
            for (int k = 0; k <= j; k++)
            {
                bitMask <<= 1;
                if (k <= j - i)
                {
                    bitMask |= 1;
                }
            }

            // Perform the insertion:
            int nWithInsertionBitsZeroedOut = n & ~bitMask; // or: (n | bitMask) ^ bitMask;
            int mShiftedIntoPosition = m << i;
            return nWithInsertionBitsZeroedOut | mShiftedIntoPosition;
        }
    }
}
