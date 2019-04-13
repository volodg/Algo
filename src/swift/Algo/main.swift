import Foundation

func maxRectanges(arr: [Int]) -> [Int:Int] {
  var stack = [(num: Int, pos: Int)]()
  
  var result = [Int:Int]()
  
  for i in 0..<arr.count {
    let el = arr[i]
    
    var startPos = i
    
    while !stack.isEmpty && stack.last!.num > el {
      startPos = stack.last!.pos
      
      let prevMax = result[stack.last!.num] ?? 0
      let len = i - stack.last!.pos
      if len > prevMax {
        result[stack.last!.num] = len
      }
      
      stack.removeLast()
    }
    
    stack.append((num: el, pos: startPos))
  }
  
  while !stack.isEmpty {
    let i = arr.count
    
    let prevMax = result[stack.last!.num] ?? 0
    let len = i - stack.last!.pos
    if len > prevMax {
      result[stack.last!.num] = len
    }
    
    stack.removeLast()
  }
  
  return result
}

func riddle(arr: [Int]) -> [Int] {
  let maxRanges = maxRectanges(arr: arr)
  
  var reversedMax = [Int:Int]()
  
  for (key, value) in maxRanges {
    let currMax = reversedMax[value] ?? 0
    if key > currMax {
      reversedMax[value] = key
    }
  }
  
  var result = [Int]()
  
  var lastCurrValue = reversedMax[arr.count]!
  var i = arr.count
  while i >= 1 {
    lastCurrValue = max(reversedMax[i] ?? lastCurrValue, lastCurrValue)
    result.append(lastCurrValue)
    i -= 1
  }
  
  return result.reversed()
}

func test(arr: [Int], res: [Int:Int]) {
  
  for (key, value) in res {
    let indexes = arr.enumerated().compactMap { x -> Int? in
      (x.element == key) ? x.offset : nil
    }
    
    assert(indexes.count > 0)
    
    var maxWidth = 0
    
    for index in indexes {
      var width = 1
      
      var i = index - 1
      while i >= 0 && arr[i] >= key {
        width += 1
        i -= 1
      }
      
      i = index + 1
      while i < arr.count && arr[i] >= key {
        width += 1
        i += 1
      }
      
      if width > maxWidth {
        maxWidth = width
      }
    }
    
    assert(maxWidth == value)
  }
  
}

//var strings = [
//  "5",
//  "6 3 5 1 12",
//]//[12, 3, 3, 1, 1]

//var strings = [
//  "4",
//  "2 6 1 12",
//]//[12: 1, 2: 2, 1: 4, 6: 1]

//var strings = [
//  "8",
//  "11 2 3 14 5 2 11 12",
//]//[3: 3, 2: 8, 5: 2, 11: 2, 14: 1, 12: 1]

//var strings = [
//  "7",
//  "1 2 3 5 1 13 3",
//]//[5: 1, 3: 2, 13: 1, 2: 3, 1: 7]

//var strings = [
//  "6",
//  "3 5 4 7 6 2",
//]//[5: 1, 4: 4, 6: 2, 3: 5, 2: 6, 7: 1]

//1 2 3
var strings = [
  "11",
  "789168277 694294362 532144299 20472621 316665904 59654039 685958445 925819184 371690486 285650353 522515445 624800694 396417773 467681822 964079876 355847868 424895284 50621903 728094833 535436067 221600465 832169804 641711594 518285605 235027997 904664230 223080251 337085579 5125280 448775176 831453463 550142629 822686012 555190916 911857735 144603739 751265137 274554418 450666269 984349810 716998518 949717950 313190920 600769443 140712186 218387168 416515873 194487510 149671312 241556542 575727819 873823206",
]
//[600769443: 1, 221600465: 10, 925819184: 1, 522515445: 2, 716998518: 3, 535436067: 2, 789168277: 1, 822686012: 1, 832169804: 1, 450666269: 4, 532144299: 3, 140712186: 23, 50621903: 24, 949717950: 1, 555190916: 3, 223080251: 7, 831453463: 1, 416515873: 1, 911857735: 1, 313190920: 6, 448775176: 6, 5125280: 52, 904664230: 1, 964079876: 1, 194487510: 3, 396417773: 5, 728094833: 1, 518285605: 3, 218387168: 2, 694294362: 2, 59654039: 13, 20472621: 28, 575727819: 2, 467681822: 2, 641711594: 2, 550142629: 5, 144603739: 15, 751265137: 1, 235027997: 5, 371690486: 3, 424895284: 1, 149671312: 7, 316665904: 1, 285650353: 11, 274554418: 8, 337085579: 1, 984349810: 1, 355847868: 7, 873823206: 1, 624800694: 1, 241556542: 3, 685958445: 2]

//var strings = [
//  "11",
//  "7 6 5 2 3",
//]

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

let res = riddle(arr: arr).map { String($0) }.joined(separator: " ")
//let res = maxRectanges(arr: arr)
//test(arr: arr, res: res)

print(res)
//print("test ok")
