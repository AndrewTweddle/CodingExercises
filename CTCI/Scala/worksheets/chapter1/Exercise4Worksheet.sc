import ctci.chapter1.Exercise4._

object Exercise4Worksheet {
  var stringWithSpaces = "a ..".toArray           //> stringWithSpaces  : Array[Char] = Array(a,  , ., .)
  var len = 2                                     //> len  : Int = 2
  getSpaceCalculationsInReverse(stringWithSpaces, len)
                                                  //> res0: List[ctci.chapter1.Exercise4.Calculation] = List(Calculation(1,1,0))
  replaceSpaces1(stringWithSpaces, len)
  new String(stringWithSpaces) == "a%20"          //> res1: Boolean = true
  
  stringWithSpaces = "a bc  de......".toArray
  len = 8
  getSpaceCalculationsInReverse(stringWithSpaces, len)
                                                  //> res2: List[ctci.chapter1.Exercise4.Calculation] = List(Calculation(3,5,2), C
                                                  //| alculation(2,4,0), Calculation(1,1,2))
  replaceSpaces1(stringWithSpaces, len)
  new String(stringWithSpaces) == "a%20bc%20%20de"//> res3: Boolean = true
  
  // Use shorter, less efficient space calculations:
  stringWithSpaces = "a bc  de......".toArray
  len = 8
  getSpaceCalculationsInReverse2(stringWithSpaces, len)
                                                  //> res4: List[ctci.chapter1.Exercise4.Calculation] = List(Calculation(3,5,2), C
                                                  //| alculation(2,4,0), Calculation(1,1,2))
  replaceSpaces1(stringWithSpaces, len, getSpaceCalculationsInReverse2)
  new String(stringWithSpaces) == "a%20bc%20%20de"//> res5: Boolean = true
  
  
  stringWithSpaces = "a bc  de......".toArray
  len = 8
  replaceSpaces2(stringWithSpaces, len)
  new String(stringWithSpaces) == "a%20bc%20%20de"//> res6: Boolean = true
  
  stringWithSpaces = "a bc  de......".toArray
  len = 8
  replaceSpaces(stringWithSpaces, len)
  new String(stringWithSpaces) == "a%20bc%20%20de"//> res7: Boolean = true
}