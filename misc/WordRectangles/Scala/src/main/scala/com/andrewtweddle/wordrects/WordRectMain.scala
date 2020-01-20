package com.andrewtweddle.wordrects

import java.io.{BufferedWriter, FileOutputStream, OutputStreamWriter}
import scala.io.Source
import scala.annotation.tailrec
import scala.collection.mutable
import org.scalameter._

object WordRectMain {
  val MAX_NO_OF_SOLNS_TO_FIND = 1000;

  // Customize the following to skip rectangle sizes that have been previously checked
  val MAX_GRID_SIZE = 100;

  class TrieNode(subTrieMap: mutable.Map[Char, TrieNode] = mutable.Map()) {
    @tailrec
    final def addChars(chars: List[Char]): Unit = {
      chars match {
        case firstChar :: otherChars => {
          val subTrieNode = subTrieMap.getOrElseUpdate(firstChar, new TrieNode())
          subTrieNode.addChars(otherChars)
        }
        case _ =>  // duplicate
      }
    }
    def addString(word: String): Unit = addChars(word.toList)
    def getChildren: Iterator[(Char, TrieNode)] = subTrieMap.iterator
    def getTrieForChar(ch: Char): Option[TrieNode] = subTrieMap.get(ch)
  }

  case class GridDimension(rowCount: Int, colCount: Int) extends Ordered[GridDimension] {
    def size = rowCount * colCount
    override def compare(that: GridDimension): Int = size.compare(that.size)
  }

  case class Solution(rowWords: Seq[String], colCount: Int)

  def main(args: Array[String]): Unit = {
    if (args.length == 0 || args(0) == "--help") {
      printHelp()
    } else {
      // 1 million words x 30 characters max (~ "floccinaucinihilipilification") will easily fit in memory.
      // So read all words up front, then close the file...
      val inputFilePath = args(0)
      val words: Seq[String] = readWordsFromInputFile(inputFilePath)
      val outStrm = if (args.length > 1) new FileOutputStream(args(1)) else System.out
      val bw = new BufferedWriter(new OutputStreamWriter(outStrm))
      try {
        val totalDuration = measure {
          val solns = solve(words)
          bw.write(s"${solns.size} solutions found")
          if (solns.isEmpty) bw.newLine() else {
            val firstSoln = solns.head
            bw.write(s" with ${firstSoln.rowWords.length} rows and ${firstSoln.colCount} columns")
            bw.newLine()
            solns.foreach(writeSoln(bw))
          }
        }
        println(s"TOTAL DURATION: $totalDuration")
      }
      finally {
        bw.flush()
      }
    }
  }

  def printHelp(): Unit = {
    println("wordrect PathToFileWithAWordPerLine [PathToOutputFile]")
    println();
    println("The input file path is an ASCII text file with words on each line.")
    println("If the output file path is missing, solutions are written to standard output.")
  }

  def readWordsFromInputFile(inputFilePath: String): Seq[String] = {
    val wordSource = Source.fromFile(inputFilePath)
    try {
      wordSource.getLines().toList
    } finally {
      wordSource.close()
    }
  }

  def writeSoln(writer: BufferedWriter)(soln: Solution): Unit = {
    soln.rowWords.foreach { word =>
      writer.write(word)
      writer.newLine()
    }
    writer.newLine()
  }

  def solve(words: Seq[String]): mutable.ArrayBuffer[Solution] = {
    val triesBySize = generateMapOfTriesBySize(words)

    def getGridDimensions = {
      val wordSizes = triesBySize.keys
      for {
        rowCount <- wordSizes
        colCount <- wordSizes
        if rowCount <= colCount
      } yield GridDimension(rowCount, colCount)
    }

    val dimensions = getGridDimensions
    val dimsBySize = dimensions.groupBy(_.size)
    val sortedGridSizes = dimsBySize.keys.toSeq.sorted(Ordering.Int.reverse)
    sortedGridSizes.map { gridSize =>
      println(s"  Grid size: $gridSize")
      var solnsForGridSize = mutable.ArrayBuffer[Solution]()

      val dims = dimsBySize(gridSize).toSeq.sortWith(_.rowCount > _.rowCount)
      dims.foreach { dim =>
        val rowCount = dim.rowCount
        val colCount = dim.colCount
        println(s"    $rowCount x $colCount")
        if (gridSize > MAX_GRID_SIZE) {
          println("        Skipping as this has been checked in the past")
        } else if (solnsForGridSize.size > MAX_NO_OF_SOLNS_TO_FIND) {
          println("Skipping as the maximum number of solutions has been found")
        } else {
          val dimDuration = measure {
            val solnsForDim = solveGrid(rowCount, colCount, triesBySize(rowCount), triesBySize(colCount))
            println(s"        ${solnsForDim.size} solutions found")
            solnsForGridSize ++= solnsForDim
          }
          println(s"        Search duration: $dimDuration")
        }
      }
      solnsForGridSize
    }.headOption.getOrElse(mutable.ArrayBuffer.empty)
  }

  def generateMapOfTriesBySize(words: Seq[String]): mutable.Map[Int, TrieNode] = {
    var triesBySize = mutable.Map[Int, TrieNode]()
    for (w <- words) {
      var trieNode = triesBySize.getOrElseUpdate(w.length, new TrieNode())
      trieNode.addString(w)
    }
    triesBySize
  }

  def solveGrid(rowCount: Int, colCount: Int, rootRowTrie: TrieNode, rootColTrie: TrieNode)
        : mutable.ArrayBuffer[Solution] = {
    val charGrid = Array.ofDim[Char](rowCount, colCount)
    val solns = mutable.ArrayBuffer[Solution]()
    var isMaxNoOfSolnsReached = false;

    def solveColumn(colId:Int, rowTriesInPrevCol: Array[TrieNode]): Unit = {
      val rowTriesInCurrCol = new Array[TrieNode](rowCount)

      def solveCell( rowId: Int, currColTrie: TrieNode): Unit = {
        if (!isMaxNoOfSolnsReached) {
          val rowTrieInSameRowOfPrevCol = rowTriesInPrevCol(rowId)
          val cellCharsWithTries = rowTrieInSameRowOfPrevCol.getChildren

          cellCharsWithTries.foreach { case (ch, rowTrieForCurrCol) =>
            currColTrie.getTrieForChar(ch).foreach { newCurrColTrieNode =>
              charGrid(rowId)(colId) = ch
              rowTriesInCurrCol(rowId) = rowTrieForCurrCol;
              if (rowId == rowCount - 1) {
                solveColumn(colId + 1, rowTriesInCurrCol)
              } else {
                // Find solutions for next cell (one row down in same column)
                solveCell(rowId + 1, newCurrColTrieNode)
              }
            }
          }
        }
      }

      if (colId == colCount) {
        // Grid full, so a solution was found...
        val rowWords = charGrid.map(new String(_))
        rowWords.foreach(word => println(s"        $word"))
        println()
        solns.append(new Solution(rowWords, colCount))
        if (solns.size >= MAX_NO_OF_SOLNS_TO_FIND) {
          isMaxNoOfSolnsReached = true;
        }
      } else {
        solveCell(0, rootColTrie)
      }
    }

    def solveFirstColumn(): Unit = {
      val rowTriesInFirstCol = new Array[TrieNode](rowCount)

      def solveCellInFirstColumn(rowId: Int, firstColTrie: TrieNode): Unit = {
        val cellCharsWithTries = firstColTrie.getChildren
        cellCharsWithTries.foreach { case (ch, newFirstColTrieNode) =>
          rootRowTrie.getTrieForChar(ch).foreach { rowTrieForFirstCol =>
            if (!isMaxNoOfSolnsReached) {
              charGrid(rowId)(0) = ch
              rowTriesInFirstCol(rowId) = rowTrieForFirstCol
              if (rowId == rowCount - 1) {
                solveColumn(1, rowTriesInFirstCol)
              } else {
                // Find solutions for next cell (one row down in same column)
                solveCellInFirstColumn(rowId + 1, newFirstColTrieNode)
              }
            }
          }
        }
      }
      solveCellInFirstColumn(0, rootColTrie);
    }

    solveFirstColumn()
    solns
  }
}
