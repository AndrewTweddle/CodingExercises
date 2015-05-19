package ctci.chapter1

object Exercise4 {
  
  def moveChars(chars: Array[Char], fromPos: Int, toPos: Int, charCount: Int): Unit = {
    val rng = if (fromPos >= toPos) 0 until charCount else charCount - 1 to 0 by -1 
    for (i <- rng) {
      val s = fromPos + i
      val d = toPos + i
      chars(d) = chars(s)
    }
  }
  
  def writeSpaceCode(chars: Array[Char], pos: Int): Unit = {
    chars(pos) = '%'
    chars(pos + 1) = '2'
    chars(pos + 2) = '0'
  }
  
  /* Use an example to work out the calculations required to know the ranges of characters to copy and where to 
   * 
   * Example: 
   * 
   * Inputs:          chars = "a bc  de......", len = 8, ".." are a pair of spare slots per "%20" at end of string
   * Expected output: chars = "a%20bc%20%20de"
   * Expected actions:
   *   Copy 2 chars "de" from pos 6 to pos 12 and 13; copy "%20" to pos 9
   *   Copy 0 chars ""                              ; copy "%20" to pos 6
   *   Copy 2 chars "bc" from pos 2 to pos 4 and 5  ; copy "%20" to pos 1
   *   
   * Rule per action line:
   *   Track total # of spaces found so far (totalSpaces)
   *   Find position of space (spacePos)
   *   Count subsequent non-space characters (followingNonSpaces)
   *   if (followingNonSpaces > 0) 
   *     Copy chars from (spacePos + 1, spacePos + followingNonSpaces) 
   *                  to (spacePos + 1 + 2 * totalSpaces, spacePos + followingNonSpaces + 2 * totalSpaces)   
   *   Copy "%20" to (spacePos - 2 + 2 * totalSpaces, spacePos + 2 * totalSpaces )
   * 
   * Calculations:  
   *   (totalSpaces, spacePos, followingNonSpaces)
   *   (1, 1, 2)
   *   (2, 4, 0)
   *   (3, 5, 2)
   */
  case class Calculation(totalSpaces: Int, spacePos: Int, followingNonSpaces: Int)
  
  def getSpaceCalculationsInReverse(chars: Array[Char], len: Int): List[Calculation] = {
    @annotation.tailrec
    def loop(currPos: Int, totalSpaces: Int, lastSpacePos: Int, 
        followingNonSpaces: Int, acc: List[Calculation]): List[Calculation] = {
      if (currPos == len) {
        if (totalSpaces == 0) acc else {
          Calculation(totalSpaces, lastSpacePos, followingNonSpaces) :: acc
        }
      }
      else {
        val currChar = chars(currPos)
	    if (currChar != ' ') {
	      loop(currPos + 1, totalSpaces, lastSpacePos, followingNonSpaces + 1, acc)   
	    } 
	    else {
	      val newAcc = if (totalSpaces == 0) acc else {
	        val calc = Calculation(totalSpaces, lastSpacePos, followingNonSpaces)
	        calc :: acc
	      }
	      loop(currPos + 1, totalSpaces + 1, lastSpacePos = currPos, 
	          followingNonSpaces = 0, acc = newAcc)
	    }
      }
    }
      
    loop(currPos = 0, totalSpaces = 0, lastSpacePos = -1, 
        followingNonSpaces = 0, acc = List.empty)
  }
  
  // Less efficient, but shorter:
  def getSpaceCalculationsInReverse2(chars: Array[Char], len: Int): List[Calculation] = {
    def loop(remSpacePositions: List[Int], totalSpaces: Int, acc: List[Calculation]): List[Calculation] = 
      remSpacePositions match {
        case Nil => acc
        case h :: Nil => Calculation(totalSpaces + 1, h, len - h - 1) :: acc
        case h1 :: (t @ List(h2, _*)) => {
          val newAcc = Calculation(totalSpaces + 1, h1, h2 - h1 - 1) :: acc
          loop(t, totalSpaces + 1, newAcc)
        }    
      }
  
    val spacePoses = 
      chars.take(len).toList.zipWithIndex.filter(_._1 == ' ').map(_._2)
    loop(spacePoses, 0, Nil)
  }
  
  def replaceSpaces1(chars: Array[Char], len: Int,
      calculator: Function2[Array[Char], Int, List[Calculation]] = getSpaceCalculationsInReverse)
    : Unit = {
    val calculationsInReverse = calculator(chars, len)
    for (calc <- calculationsInReverse) {
      val destSpace = calc.spacePos - 2 + 2 * calc.totalSpaces
      if (calc.followingNonSpaces != 0) {
        moveChars(chars, calc.spacePos + 1, destSpace + 3 , calc.followingNonSpaces )  
      }
      writeSpaceCode(chars, destSpace)
    }
  }
  
  /* A simpler implementation that works backwards from the end copying or replacing characters as required.
   * The only nuance is that it must count the number of spaces up front to calculate the destination
   */
  def replaceSpaces2(chars: Array[Char], len: Int): Unit = {
    var remSpaces = chars.take(len).count(_ == ' ')
    var dest = len + 2 * remSpaces - 1
    var src = len - 1
    while (remSpaces > 0) { 
      /* An alternative condition above is "while (dest != src)",
       * removing the need to decrement remSpaces below,
       * but this way is easier to understand.
       */
      if (chars(src) == ' ') {
        dest -= 2
        writeSpaceCode(chars, dest)
        remSpaces -= 1
      } else {
        chars(dest) = chars(src)
      }
      dest -= 1
      src -= 1
    }
  }
  // The above would be very elegant using char pointers in C or C++
  
  // Best implementation:
  val replaceSpaces = replaceSpaces2 _
}
