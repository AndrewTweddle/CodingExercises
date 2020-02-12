package com.andrewtweddle.wordrects

import java.io.{BufferedWriter, FileOutputStream, OutputStreamWriter}

import scala.io.Source
import scala.annotation.tailrec
import scala.collection.mutable
import org.scalameter._

object WordRectMain {
  val MAX_NO_OF_SOLNS_TO_FIND = 100;

  // Customize the following to skip rectangle sizes that have been previously checked
  val MAX_GRID_SIZE = 1000;

  trait AbstractTrieNode[
      TTrieNode <: AbstractTrieNode[TTrieNode, TSubTrieContainer],
      TSubTrieContainer <: mutable.Iterable[(Char, TTrieNode)]] {
    val subTries: TSubTrieContainer
    def getChildren: Iterator[(Char, TTrieNode)] = subTries.iterator
  }

  class MutableTrieNode extends AbstractTrieNode[MutableTrieNode, mutable.Map[Char, MutableTrieNode]] {
    override val subTries: mutable.Map[Char, MutableTrieNode] = mutable.Map()

    @tailrec
    final def addChars(chars: List[Char]): Unit = {
      chars match {
        case firstChar :: otherChars => {
          val subTrieNode = subTries.getOrElseUpdate(firstChar, new MutableTrieNode())
          subTrieNode.addChars(otherChars)
        }
        case _ =>  // duplicate
      }
    }
    def addString(word: String): Unit = addChars(word.toList)
    def getTrieForChar(ch: Char): Option[MutableTrieNode] = subTries.get(ch)
  }

  class TrieNode(src: MutableTrieNode) extends AbstractTrieNode[TrieNode, mutable.ArrayBuffer[(Char, TrieNode)]]{
    override val subTries = src.getChildren.map {
      case (ch: Char, mtn: MutableTrieNode) => (ch, new TrieNode(mtn))
    }.to(mutable.ArrayBuffer).sortInPlaceWith(_._1 <= _._1)
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
        println()
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
        if rowCount >= colCount
      } yield GridDimension(rowCount, colCount)
    }

    val dimensions = getGridDimensions
    val dimsBySize = dimensions.groupBy(_.size)
    val sortedGridSizes = dimsBySize.keys.toSeq.sorted(Ordering.Int.reverse).to(LazyList)
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
          println("        Skipping as the maximum number of solutions has been found")
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
    }.filter(_.nonEmpty).headOption.getOrElse(mutable.ArrayBuffer.empty)
  }

  def generateMapOfTriesBySize(words: Seq[String]): mutable.Map[Int, TrieNode] = {
    var triesBySize = mutable.Map[Int, MutableTrieNode]()
    for (w <- words) {
      var trieNode = triesBySize.getOrElseUpdate(w.length, new MutableTrieNode())
      trieNode.addString(w)
    }
    triesBySize.map{ case (count: Int, mutTN: MutableTrieNode) => (count, new TrieNode(mutTN)) }
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
          val rowCharsWithTriesIter = rowTrieInSameRowOfPrevCol.getChildren
          val colCharsWithTriesIter = currColTrie.getChildren

          while (rowCharsWithTriesIter.hasNext && colCharsWithTriesIter.hasNext){
            var (rowCh, rowTn) = rowCharsWithTriesIter.next
            var (colCh, colTn) = colCharsWithTriesIter.next
            while (rowCh < colCh && rowCharsWithTriesIter.hasNext) {
              val rowChTn = rowCharsWithTriesIter.next
              rowCh = rowChTn._1
              rowTn = rowChTn._2
            }
            if (rowCh >= colCh) {
              while (colCh < rowCh && colCharsWithTriesIter.hasNext) {
                val colChTn = colCharsWithTriesIter.next
                colCh = colChTn._1
                colTn = colChTn._2
              }
              if (rowCh == colCh) {
                charGrid(rowId)(colId) = rowCh
                rowTriesInCurrCol(rowId) = rowTn;
                if (rowId == rowCount - 1) {
                  solveColumn(colId + 1, rowTriesInCurrCol)
                } else {
                  // Find solutions for next cell (one row down in same column)
                  solveCell(rowId + 1, colTn)
                }
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

    val rowTriesForFirstCol = Array.fill(rowCount)(rootRowTrie)
    solveColumn(0, rowTriesForFirstCol)
    solns
  }
}
