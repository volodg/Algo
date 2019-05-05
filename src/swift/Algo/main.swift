import Foundation

enum State {
  case Palindromic1(start: Int)
  case TwoDiffer(start: Int)
  case Palindromic2(start: Int, middle: Int)
}

func sumSeq(_ num: Int) -> Int64 {
  return Int64(num + 1) * Int64(num) / 2
}

//mnonopoo
func substrCount(n _: Int, s: String) -> Int64 {
  
  let chars: [Character] = Array(s)
  
  var result: Int64 = 0
  var state: State = .Palindromic1(start: 0)
  
  for i in 1..<chars.count {
    switch state {
    case .Palindromic1(let start):
      let prevChar = chars[start]
      let currChar = chars[i]
      if prevChar != currChar {
        let len = i - start
        result += sumSeq(len)
        state = .TwoDiffer(start: start)
      }
    case .TwoDiffer(let start):
      let firstChar  = chars[start]
      let secondChar = chars[i - 1]
      let currChar   = chars[i]
      if firstChar == currChar {
        result += 1
        state = .Palindromic2(start: start, middle: i - 1)
      } else if secondChar == currChar {
        state = .Palindromic1(start: i - 1)
      } else {
        result += 1
        state = .TwoDiffer(start: i - 1)
      }
    case .Palindromic2(let start, let middle):
      let firstChar  = chars[start]
      let secondChar = chars[middle]
      let currChar   = chars[i]
      if currChar == firstChar {
        let left  = middle - start
        let right = i - middle
        if right > left {
          result += Int64(left)
          state = .Palindromic1(start: middle + 1)
        }
      } else if currChar == secondChar {
        let right = i - middle - 1
        result += Int64(right)
        
        if right == 1 {
          result += 1
          state = .Palindromic2(start: middle, middle: middle + 1)
        } else {
          result += sumSeq(right)
          state = .TwoDiffer(start: middle + 1)
        }
      } else {
        let right = i - middle - 1
        result += Int64(right)
        result += sumSeq(right)
        state = .TwoDiffer(start: middle + 1)
      }
    }
  }
  
  let i = chars.count
  
  switch state {
  case .Palindromic1(let start):
    let len = i - start
    result += sumSeq(len)
  case .TwoDiffer:
    result += 1
  case .Palindromic2(_, let middle):
    let right = i - middle - 1
    result += Int64(right)
    result += sumSeq(right)
  }

  return result
}

//var strings = [
//  "10",
//  "mnonopoo"
//]//12

//var strings = [
//  "10",
//  "asasd"
//]//7

//var strings = [
//  "10",
//  "abcbaba"
//]//10

//var strings = [
//  "10",
//  "aaaa"
//]//10

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

guard let s = readLine2() else { fatalError("Bad input") }

let result = substrCount(n: n, s: s)

print(result)
//print("test ok")
