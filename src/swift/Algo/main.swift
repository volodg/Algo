import Foundation

// (1, 9) + (1, 9) * 8
var permDequeue = [([Int], [Int])]()
var initialPerm = [0, 1, 2, 3, 4, 5, 6, 7, 8]//, 9]
var completed = false
var maxDepth = 0

func doNextPerm(canSelect: ((Int, [Int]) -> Bool)? = nil) -> [Int]? {
  if completed {
    return nil
  }
  if permDequeue.isEmpty {
    permDequeue.append(([], initialPerm))
  }
  
  while !completed {
    let top = permDequeue.last!
    
    permDequeue.removeLast()
    completed = permDequeue.isEmpty
    
    if top.1.isEmpty {
      return top.0
    }
    
    let allNext = (0..<top.1.count).compactMap { someIndex -> ([Int], [Int])? in
      let index = top.1.count - someIndex - 1
      var newTop = top.1
      newTop.remove(at: index)
      if !(canSelect?(top.1[index], top.0) ?? true) {
        return nil
      }
      //TODO top.0 + [top.1[index]]
      return (top.0 + [top.1[index]], newTop)
    }
    
    permDequeue.append(contentsOf: allNext)
    if maxDepth < permDequeue.count {
      maxDepth = permDequeue.count
    }
    
    completed = permDequeue.isEmpty
  }
  
  return nil
}

let start = Date().timeIntervalSince1970

var perm = doNextPerm(canSelect: nil)
//print("perm: \(perm)")
//print("perm: \(perm)")
var totalSize = 0

while perm != nil {
  totalSize += 1
  //print(perm!)
  perm = doNextPerm(canSelect: nil)
}

let done = Date().timeIntervalSince1970

print("totalSize: \(totalSize)")//3628800
print("maxDepth: \(maxDepth)")//46
print("time: \(done - start)")//38.13204908370972

final class Place {
  let pos: (x: Int, y: Int, hor: Bool)
  let size: Int
  let cross: [Cross]
  
  init(pos: (x: Int, y: Int, hor: Bool), size: Int, cross: [Cross]) {
    self.pos = pos
    self.size = size
    self.cross = cross
  }
}

final class Cross {
  let myPos: Int
  let wordIndex: Int//places[wordIndex]
  let hisPos: Int
  
  init(myPos: Int, wordIndex: Int, hisPos: Int) {
    self.myPos = myPos
    self.wordIndex = wordIndex
    self.hisPos = hisPos
  }
}

func crosswordPuzzle(crossword: [String], words: String) -> [String] {
  let allWords = words.split(separator: ";")
  
  for i in 0..<allWords.count {
    initialPerm.append(i)
  }
  
  var charsTable = Array<[Character]>(repeating: [Character](), count: crossword.count)
  
  for (index, crosswordLine) in crossword.enumerated() {
    charsTable[index] = Array(crosswordLine)
  }
  
  var places = [Place]()
  
  let height = charsTable.count
  let width = charsTable[0].count
  
  var startPos = (x: 0, y: 0)
  
  func findWordStartPos() -> (x: Int, y: Int, hor: Bool)? {
    var startX = startPos.x
    var useStartPos = true
    for y in 0..<height {
      var yy = y
      if useStartPos {
        yy = startPos.y
      }
      for x in startX..<width {
        let currChar = charsTable[yy][x]
        let isStart = currChar == "-" || "0"..."9" ~= currChar
        if isStart {
          if (x + 1 < width && charsTable[y][x + 1] == "-") {
            startPos = (x: x, y: yy)
            return (x: x, y: yy, hor: true)
          }
          if (y + 1 < height && charsTable[y + 1][x] == "-") {
            startPos = (x: x, y: yy)
            return (x: x, y: yy, hor: false)
          }
        }
      }
      startX = 0
      useStartPos = false
    }
    
    return nil
  }
  
  //TODO return crosses
  func fillWord(x: Int, y: Int, hor: Bool, index: Int) -> Place {
    var (currPos, max) = hor ? (x, width) : (y, height)
    
    func currPosXY() -> (x: Int, y: Int) {
      if hor { return (x: currPos, y: y) } else { return (x: x, y: currPos) }
    }
    
    func getCurr() -> Character {
      let pos = currPosXY()
      return charsTable[pos.y][pos.x]
    }
    
    func setCurr(_ newVal: Character) {
      let pos = currPosXY()
      charsTable[pos.y][pos.x] = newVal
    }
    
    var size = 0
    var crosses = [Cross]()
    
    var currChar = getCurr()
    
    func findHisPos(hor: Bool, x: Int, y: Int) -> Int {
      var result = 0
      var xx = x
      var yy = y
      var currChar = charsTable[yy][xx]
      while ("0"..."9" ~= currChar) {
        if (hor) {
          xx -= 1
        } else {
          yy -= 1
        }
        if xx < 0 || yy < 0 {
          return result
        }
        currChar = charsTable[yy][xx]
        result += 1
      }
      return result - 1
    }

    while (currChar == "-" || "0"..."9" ~= currChar) {
      let str = String(index)
      let indexChar = str.startIndex
      if "0"..."9" ~= currChar {
        let crossWordIndex = Int(String(currChar))!
        let myPos = size
        let currXY = currPosXY()
        let hisPos = findHisPos(hor: !hor, x: currXY.x, y: currXY.y)
        let cross = Cross(myPos: myPos, wordIndex: crossWordIndex, hisPos: hisPos)
        crosses.append(cross)
      }
      setCurr(str[indexChar])
      size += 1
      currPos += 1
      if currPos >= max {
        break
      } else {
        currChar = getCurr()
      }
    }
    
    return Place(pos: (x: x, y: y, hor: hor), size: size, cross: crosses)
  }
  
  func findWordPlace() -> Bool {
    guard let wordStartPos = findWordStartPos() else { return false }
    let wordIndex = places.count
    
    let place = fillWord(x: wordStartPos.x, y: wordStartPos.y, hor: wordStartPos.hor, index: wordIndex)
    
    places.append(place)
    
    return true
  }
  
  while findWordPlace() {}
  
  func printCrosses() {
//    for (index, place) in places.enumerated() {
//      let croses = place.cross.map { "wordIndex: \($0.wordIndex) hisPos: \($0.hisPos) myPos: \($0.myPos)" }
//      print("pos: \(index) crosses: \(croses)")
//    }
  }
  printCrosses()
  
  func canSelect(newIndex: Int, existing: [Int]) -> Bool {
    let newWordPlace = places[existing.count]
    let newWord = allWords[newIndex]
    
    if (newWord.count != newWordPlace.size) {
      return false
    }
    
    let result = newWordPlace.cross.first { cross -> Bool in
      let realIndex = existing[cross.wordIndex]
      let existingWord = allWords[realIndex]
      
      let newChar = newWord[newWord.index(newWord.startIndex, offsetBy: cross.myPos)]
      let hisChar = existingWord[existingWord.index(existingWord.startIndex, offsetBy: cross.hisPos)]
      
      return newChar != hisChar
    } == nil
    
    return result
  }
  
  let words = doNextPerm(canSelect: canSelect)!.map { allWords[$0] }
  
  for (index, word) in words.enumerated() {
    
    let wordChars = Array(word)
    let pos = places[index]
    
    for i in 0..<pos.size {
      let yy = (pos.pos.hor) ? pos.pos.y : pos.pos.y + i
      let xx = (pos.pos.hor) ? pos.pos.x + i : pos.pos.x
      charsTable[yy][xx] = wordChars[i]
    }
    
  }
  
  //printState()
  
  let result = charsTable.map { String($0) }
  
  //return words.map { String($0) }
  return result
}

//test 1
//var strings = [
//  "++++++++++",
//  "+------+++",
//  "+++-++++++",
//  "+++-++++++",
//  "+++-----++",
//  "+++-++-+++",
//  "++++++-+++",
//  "++++++-+++",
//  "++++++-+++",
//  "++++++++++",
//  "POLAND;LHASA;SPAIN;INDIA"
//]
//pos: 0 crosses: []
//pos: 1 crosses: ["wordIndex: 0 hisPos: 2 myPos: 0"]
//pos: 2 crosses: ["wordIndex: 1 hisPos: 3 myPos: 0"]
//pos: 3 crosses: ["wordIndex: 2 hisPos: 3 myPos: 0"]

//test 2
var strings = [
  "+-++++++++",
  "+-++++++++",
  "+-++++++++",
  "+-----++++",
  "+-+++-++++",
  "+-+++-++++",
  "+++++-++++",
  "++------++",
  "+++++-++++",
  "+++++-++++",
  "LONDON;DELHI;ICELAND;ANKARA"
]
//pos: 0 crosses: []
//pos: 1 crosses: ["wordIndex: 0 hisPos: 3 myPos: 0"]
//pos: 2 crosses: ["wordIndex: 1 hisPos: 4 myPos: 0"]
//pos: 3 crosses: ["wordIndex: 2 hisPos: 4 myPos: 3"]

//test 3
//var strings = [
//  "+-++++++++",
//  "+-++++++++",
//  "+-------++",
//  "+-++++++++",
//  "+-++++++++",
//  "+------+++",
//  "+-+++-++++",
//  "+++++-++++",
//  "+++++-++++",
//  "++++++++++",
//  "AGRA;NORWAY;ENGLAND;GWALIOR"
//]//ENGLAND;GWALIOR;NORWAY;AGRA
//pos: 0 crosses: []
//pos: 1 crosses: ["wordIndex: 0 hisPos: 2 myPos: 0"]
//pos: 2 crosses: ["wordIndex: 0 hisPos: 5 myPos: 0"]
//pos: 3 crosses: ["wordIndex: 2 hisPos: 4 myPos: 0"]

//test 4
//var strings = [
//  "++++++-+++",
//  "++------++",
//  "++++++-+++",
//  "++++++-+++",
//  "+++------+",
//  "++++++-+-+",
//  "++++++-+-+",
//  "++++++++-+",
//  "++++++++-+",
//  "++++++++-+",
//  "ICELAND;MEXICO;PANAMA;ALMATY"
//]//ICELAND;MEXICO;PANAMA;ALMATY

var index = 0

func readLine2() -> String? {
  if index >= strings.count {
    return nil
  }
  let result = strings[index]
  index += 1
  return result
}

let crossword: [String] = AnyIterator{ readLine2() }.prefix(10).map {
  $0
}

guard crossword.count == 10 else { fatalError("Bad input") }

guard let words = readLine2() else { fatalError("Bad input") }

initialPerm = []//[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
completed = false

let result = crosswordPuzzle(crossword: crossword, words: words)

//let result = rotLeft(a: a, d: d)
print(result.joined(separator: "\n"))
