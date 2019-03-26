import Foundation

func luckBalance(k: Int, contests: [[Int]]) -> Int {
  
  let (zeros, nonZeros) = contests.reduce(([Int](), [Int]())) { (acc, record) in
    if record[1] == 0 {
      return (acc.0 + [record[0]], acc.1)
    }
    return (acc.0, acc.1 + [record[0]])
  }
  
  let sortedNonZeros = nonZeros.sorted { (a, b) -> Bool in
    return a > b
  }
  
  var sum = 0
  for el in zeros {
    sum += el
  }
  var index = 0
  while index < sortedNonZeros.count && index < k {
    sum += sortedNonZeros[index]
    index += 1
  }
  
  while index < sortedNonZeros.count {
    sum -= sortedNonZeros[index]
    index += 1
  }
  
  return sum
}

//test 2
var strings = [
  "6 3",//+
  "5 1",//+
  "2 1",//+
  "1 1",//-
  "8 1",//+
  "10 0",//+
  "5 0"//+
]//29

var index = 0

func readLine2() -> String? {
  if index >= strings.count {
    return nil
  }
  let result = strings[index]
  index += 1
  return result
}

guard let nkTemp = readLine2() else { fatalError("Bad input") }
let nk = nkTemp.split(separator: " ").map{ String($0) }

guard let n = Int(nk[0].trimmingCharacters(in: .whitespacesAndNewlines))
  else { fatalError("Bad input") }

guard let k = Int(nk[1].trimmingCharacters(in: .whitespacesAndNewlines))
  else { fatalError("Bad input") }

let contests: [[Int]] = AnyIterator{ readLine2() }.prefix(n).map {
  let contestsRow: [Int] = $0.split(separator: " ").map {
    if let contestsItem = Int($0.trimmingCharacters(in: .whitespacesAndNewlines)) {
      return contestsItem
    } else { fatalError("Bad input") }
  }
  
  guard contestsRow.count == 2 else { fatalError("Bad input") }
  
  return contestsRow
}

guard contests.count == n else { fatalError("Bad input") }

let result = luckBalance(k: k, contests: contests)

print(result)
