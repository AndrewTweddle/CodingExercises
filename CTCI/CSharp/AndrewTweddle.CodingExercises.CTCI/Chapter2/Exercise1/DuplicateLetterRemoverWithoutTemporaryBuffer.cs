using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace AndrewTweddle.CodingExercises.CTCI.Chapter2.Exercise1
{
    public class DuplicateLetterRemoverWithoutTemporaryBuffer: IDuplicateLetterRemover
    {
        public void RemoveDuplicateLetters(LinkedList<char> letters)
        {
            LinkedListNode<char> currNode = letters.First;
            while (currNode != null)
            {
                char currChar = currNode.Value;

                // Check all subsequent nodes to see if they match the current node:
                LinkedListNode<char> nodeToCheck = currNode.Next;
                while (nodeToCheck != null)
                {
                    LinkedListNode<char> nextNodeToCheck = nodeToCheck.Next;
                    if (nodeToCheck.Value == currChar)
                    {
                        letters.Remove(nodeToCheck);
                    }
                    nodeToCheck = nextNodeToCheck;
                }

                currNode = currNode.Next;
            }
        }
    }
}
