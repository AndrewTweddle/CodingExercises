using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace AndrewTweddle.CodingExercises.CTCI.Chapter2.Exercise1
{
    public static class DuplicateLetterRemover
    {
        public static void RemoveDuplicateLetters(LinkedList<char> letters)
        {
            ISet<char> charsFound = new HashSet<char>();
            LinkedListNode<char> currNode = letters.First;
            while (currNode != null)
            {
                LinkedListNode<char> nextNode = currNode.Next;
                char currChar = currNode.Value;
                if (charsFound.Contains(currChar))
                {
                    letters.Remove(currNode);
                }
                else
                {
                    charsFound.Add(currChar);
                }
                currNode = nextNode;
            }
        }
    }
}
