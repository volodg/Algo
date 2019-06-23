import Foundation

func dynamicArray(n: Int, queries: [[Int]]) -> [Int] {
  
  var result = [Int]()
  
  var cache = [[Int]](repeating: [], count: n)
  
  let N = n
  
  for query in queries {
    let type = query[0]
    let x = query[1]
    let y = query[2]
    
    let index = (x ^ (result.last ?? 0)) % N
    switch type {
    case 1:
      var seq = cache[index]
      seq.append(y)
      cache[index] = seq
    case 2:
      let seq = cache[index]
      let value = seq[y % seq.count]
      result.append(value)
    default:
      break
    }
  }
  
  return result
}

var strings = [
  "2 5",
  "1 0 5",
  "1 1 7",
  "1 0 3",
  "2 1 0",
  "2 1 1"
]//[7, 3]

//var strings = [
//  "10",
//  "abcbaba"
//]//10

//var strings = [
//  "10",
//  "aaaa"
//]//10

var index = 0

//let aStreamReader = StreamReader(path: "/Users/volodg/Sources/Algo/src/swift/Algo/input.txt")!

func readLine2() -> String? {
  if index >= strings.count {
    return nil
  }
  let result = strings[index]
  index += 1
//  let result = aStreamReader.nextLine()
  return result
}

guard let nqTemp = readLine2()?.replacingOccurrences(of: "\\s+$", with: "", options: .regularExpression) else { fatalError("Bad input") }
let nq = nqTemp.split(separator: " ").map{ String($0) }

guard let n = Int(nq[0])
  else { fatalError("Bad input") }

guard let q = Int(nq[1])
  else { fatalError("Bad input") }

let queries: [[Int]] = AnyIterator{ readLine2()?.replacingOccurrences(of: "\\s+$", with: "", options: .regularExpression) }.prefix(q).map {
  let queriesRow: [Int] = $0.split(separator: " ").map {
    if let queriesItem = Int($0) {
      return queriesItem
    } else { fatalError("Bad input") }
  }
  
  guard queriesRow.count == 3 else { fatalError("Bad input") }
  
  return queriesRow
}

guard queries.count == q else { fatalError("Bad input") }

let result = dynamicArray(n: n, queries: queries)

print(result)
//print("test ok")
