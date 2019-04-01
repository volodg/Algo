import Foundation

func nmax(_ num: Int, _ a: Int, _ b: Int) -> Int {
  if (a ^ num) > (b ^ num) {
    return a
  }
  return b
}

final class Trie {
  var left: Trie? = nil
  var right: Trie? = nil
  var end: Bool = false
  var _max = 0
  
  func append(_ val: Int) {
    if val > _max {
      _max = val
    }
    
    if val == 0 {
      end = true
    } else {
      let bit = val & 1
      if bit == 0 {
        let left = self.left ?? Trie()
        self.left = left
        left.append(val >> 1)
      } else {
        let right = self.right ?? Trie()
        self.right = right
        right.append(val >> 1)
      }
    }
  }
  
  func maxXor(_ num: Int, myBit: Int) -> Int {
    var subMax1: Int?
    if end {
      subMax1 = myBit
    }
    
    if (right ?? left) == nil {
      return myBit
    }
    
    var subMax = 0
    
    let mmmax = _max << 1 + myBit
    
    let next_bit = num & 2
    if next_bit == 0 {
      let nmyBit = (right != nil) ? 1 : 0
      subMax = (right ?? left)!.maxXor(num >> 1, myBit: nmyBit) << 1 + myBit
    } else {
      let nmyBit = (left != nil) ? 0 : 1
      subMax = (left ?? right)!.maxXor(num >> 1, myBit: nmyBit) << 1 + myBit
    }
    
    if let subMax1 = subMax1 {
      let yy = nmax(num, subMax, subMax1)
      return nmax(num, yy, mmmax)
    }
    //return subMax
    return nmax(num, subMax, mmmax)
  }
  
  func maxXor(_ num: Int) -> Int {
    if num == 0 {
      return _max
    }
    
    var subMax1: Int?
    if end {
      subMax1 = 0
    }
    
    var y = 0
    
    let bit = num & 1
    if bit == 0 {
      let nmyBit = (right != nil) ? 1 : 0
      y = (right ?? left)?.maxXor(num, myBit: nmyBit) ?? 0
    } else {
      let nmyBit = (left != nil) ? 0 : 1
      y = (left ?? right)?.maxXor(num, myBit: nmyBit) ?? 0
    }
    
    let yy = nmax(num, y, _max)
    if let subMax1 = subMax1 {
      return nmax(num, yy, subMax1)
    }
    return yy
  }
}

let trie = Trie()

func maxXor(arr: [Int], queries: [Int]) -> [Int] {
  
  arr.forEach { val in
    trie.append(val)
  }
  
  //return queries.map { trie.maxXor($0) ^ $0 }
  return queries.map { trie.maxXor($0) }
}

func maxXor_old(arr: [Int], queries: [Int]) -> [Int] {
  return queries.map { query -> Int in
    var max = 0
    var num: Int?
    for el in arr {
      let curr = el ^ query
      if curr > max {
        max = curr
        num = el
      }
    }
    return num ?? arr[0]
    //return max
  }
}


//test 1

//var strings = [
//  "4",
//  "1 3 5 7",
//  "2",
//  "17",
//  "6"
//]//[22, 7] - [7, 1]

//var strings = [
//  "5",
//  "5 1 7 4 3",
//  "2",
//  "2",
//  "0"
//]//[7, 7] - [5, 7]

//var strings = [
//  "3",
//  "0 1 2",
//  "3",
//  "3",
//  "7",
//  "2"
//]//[3, 7, 3] - [0, 0, 1]

var strings = [
  "1",
  "1 2",//"8"
  "10",//index
//  "3",
//  "7",
//  "2",
//  "1",
//  "4",
//  "5",
//  "6",
  "23423423",
//  "9",
//  "19",
]//[3, 7, 3] - [0, 0, 1]

var index = 0

func readLine2() -> String? {
  if index >= strings.count {
    return nil
  }
  let result = strings[index]
  index += 1
  return result
}

guard let n = Int((readLine2()?.trimmingCharacters(in: .whitespacesAndNewlines))!)
  else { fatalError("Bad input") }

guard let arrTemp = readLine2() else { fatalError("Bad input") }
let arr: [Int] = arrTemp.split(separator: " ").map {
  if let arrItem = Int($0.trimmingCharacters(in: .whitespacesAndNewlines)) {
    return arrItem
  } else { fatalError("Bad input") }
}

//guard arr.count == n else { fatalError("Bad input") }

guard let m = Int((readLine2()?.trimmingCharacters(in: .whitespacesAndNewlines))!)
  else { fatalError("Bad input") }

let queries: [Int] = AnyIterator{ readLine2() }.prefix(m).map {
  if let queriesItem = Int($0.trimmingCharacters(in: .whitespacesAndNewlines)) {
    return queriesItem
  } else { fatalError("Bad input") }
}

//guard queries.count == m else { fatalError("Bad input") }

let result1 = maxXor_old(arr: arr, queries: queries)
print(result1)

let result2 = maxXor(arr: arr, queries: queries)
print(result2)

//assert(result1 == result2)
