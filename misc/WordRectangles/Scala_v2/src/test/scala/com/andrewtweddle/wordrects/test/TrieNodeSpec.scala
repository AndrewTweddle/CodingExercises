package com.andrewtweddle.wordrects.test

import com.andrewtweddle.wordrects.WordRectMain._
import org.scalatest._

class TrieNodeSpec extends FeatureSpec with GivenWhenThen {
  feature("MutableTrieNode") {
    scenario("A word is added to a TrieNode") {
      Given("An empty TrieNode")
      val tn = new MutableTrieNode()

      When("adding a word which has 2 or more letters")
      val word = "Hello"
      tn.addString(word)

      Then("The first letter of the word is under the root TrieNode")
      val optNode = tn.getTrieForChar(word(0))
      assert(optNode.nonEmpty)

      And("The sub-TrieNode for the first letter has the second letter")
      val optSubNode = optNode.get.getTrieForChar(word(1))
      assert(optSubNode.nonEmpty)
    }
  }
}
