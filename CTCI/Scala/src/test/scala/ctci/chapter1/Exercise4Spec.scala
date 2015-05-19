package ctci.chapter1

import org.scalatest.FunSpec
import org.scalatest.GivenWhenThen

import Exercise4._

class Exercise4Spec extends FunSpec with GivenWhenThen {
  describe("Exercise4") {
    
    describe("moveChars") {
      it("should be able to move a block of characters further on in the string") {
        given("an array of characters")
        val originalString = "Shoot a wabbit  !"
        val srcPos = 8
        val destPos = 10
        val numberOfCharsToMove = 6
        val chars = originalString.toArray
        
        when("copying a subset of characters to an overlapping range further on")
        moveChars(chars, srcPos, destPos, numberOfCharsToMove)
        
        then("the destination characters have changed")
        val charsToMove = originalString.drop(srcPos).take(numberOfCharsToMove) 
        expectResult(charsToMove) {
          new String(chars.drop(destPos).take(numberOfCharsToMove))
        }
        
        and("the moved characters are as expected for the defined range")
        expectResult("wabbit") {
          new String(charsToMove)
        }
        
        and("prior characters are unchanged")
        expectResult(originalString.take(destPos)) {
          new String(chars.take(destPos))  
        }
        
        and("subsequent characters are unchanged")
        expectResult(originalString.drop(destPos + numberOfCharsToMove)) {
          new String(chars.drop(destPos + numberOfCharsToMove))
        }
        
        and("the final string is as expected")
        expectResult("Shoot a wawabbit!") {
          new String(chars)
        }
      }
      
      it("should be able to move a block of characters earlier in the string") {
        given("An array of characters")
        val originalString = "Go pigbang!"
        val srcPos = 6
        val destPos = 3
        val numberOfCharsToMove = 4
        val chars = originalString.toArray
        
        when("copying a subset of characters to an overlapping range earlier on")
        moveChars(chars, srcPos, destPos, numberOfCharsToMove)
        
        then("the destination characters have changed")
        val charsToMove = originalString.drop(srcPos).take(numberOfCharsToMove) 
        expectResult(charsToMove) {
          new String(chars.drop(destPos).take(numberOfCharsToMove))
        }
        
        and("the moved characters are as expected for the defined range")
        expectResult("bang") {
          new String(charsToMove)
        }
        
        and("prior characters are unchanged")
        expectResult(originalString.take(destPos)) {
          new String(chars.take(destPos))  
        }
        
        and("subsequent characters are unchanged")
        expectResult(originalString.drop(destPos + numberOfCharsToMove)) {
          new String(chars.drop(destPos + numberOfCharsToMove))
        }
        
        and("the final string is as expected")
        expectResult("Go bangang!") {
          new String(chars)
        }
      }
    }
    
    describe("getSpaceCalculationsInReverse") {
      describe("the list of calculations") {
        it("should be empty if there are no spaces") {
          given("a string with no spaces")
          val chars = "abc".toArray
          val len = 3
          
          when("performing the calculations for each space")
          val calcs = getSpaceCalculationsInReverse(chars, len)
          
          then("there are no calculations")
          expectResult(Nil: List[Calculation]) {
            calcs  
          }
        }
        
        it("should have a single calculation if there is only one space") {
          given("a string with a single space before its length, and other spaces afterwards")
          val chars = "ab cde  ".toArray
          val len = 6
          
          when("performing the calculations")
          val calcs = getSpaceCalculationsInReverse(chars, len)
          
          then("there is a single calculation")
          expectResult(1) {
            calcs.length
          }
          val calc = calcs.head
          
          and("the calculation is as expected")
          expectResult(Calculation(totalSpaces = 1, spacePos = 2, followingNonSpaces = 3)) {
            calc
          }
        } 
      }
    }
  }
}
