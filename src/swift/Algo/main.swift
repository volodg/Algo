import Foundation

class SetWr {
  var set = Set<Int>()
}

class NodesSet {
  var set: SetWr
  
  init(element: Int) {
    self.set = SetWr()
    self.set.set.insert(element)
  }
}

func minTime(roads: [[Int]], machines: [Int]) -> Int {
  
  var citiesSets = Array<NodesSet>()
  for i in 0..<(roads.count + 1) {
    citiesSets.append(NodesSet(element: i))
  }
  
  var result: Int = 0
  
  for machine in machines {
    citiesSets[machine].set.set.insert(-1)
  }
  
  var roads = roads
  roads.sort { (a, b) -> Bool in
    a[2] > b[2]
  }
  
  for edge in roads {
    let fromNode = citiesSets[edge[0]]
    let toNode = citiesSets[edge[1]]
    let fromHasNachine = fromNode.set.set.contains(-1)
    let toHasNachine = toNode.set.set.contains(-1)
    if fromHasNachine && toHasNachine {
      result += edge[2]
    } else {
      fromNode.set.set.formUnion(toNode.set.set)
      toNode.set = fromNode.set
    }
  }
  
  return result
}

//var strings = [
//  "5 3",
//  "2 1 8",
//  "1 0 5",
//  "2 4 5",
//  "1 3 4",
//  "2",
//  "4",
//  "0"
//]//10
//

var strings = [
"100 23",
"9 78 35",
"9 54 45",
"78 69 27",
"9 55 9",
"9 1 78",
"1 92 7",
"55 42 57",
"1 84 4",
"1 5 38",
"92 8 75",
"55 30 99",
"69 7 9",
"1 81 45",
"8 31 4",
"42 23 100",
"78 95 3",
"54 14 14",
"84 53 80",
"92 32 8",
"42 86 40",
"1 64 93",
"78 60 65",
"64 76 24",
"42 89 86",
"7 28 48",
"69 62 26",
"1 40 23",
"78 38 29",
"8 44 39",
"78 3 37",
"54 26 17",
"62 50 24",
"76 66 37",
"30 51 75",
"86 43 91",
"5 77 32",
"64 91 11",
"14 10 36",
"26 20 19",
"9 52 50",
"77 94 32",
"44 67 63",
"64 15 61",
"92 0 73",
"10 37 23",
"89 2 37",
"92 18 51",
"26 47 25",
"30 87 15",
"47 36 35",
"92 72 16",
"28 75 93",
"78 73 66",
"20 19 64",
"73 57 1",
"91 6 50",
"54 33 41",
"78 11 38",
"37 71 55",
"5 63 52",
"10 46 22",
"94 82 19",
"95 83 51",
"57 90 10",
"63 58 94",
"43 45 23",
"72 68 62",
"82 85 88",
"58 4 94",
"82 41 62",
"3 22 68",
"54 70 78",
"31 74 27",
"36 29 61",
"33 24 76",
"40 35 61",
"83 79 51",
"8 59 20",
"45 34 26",
"38 12 18",
"70 99 25",
"40 80 81",
"31 97 58",
"69 21 16",
"83 13 22",
"80 48 49",
"97 65 44",
"74 17 1",
"68 16 92",
"50 98 54",
"94 27 76",
"81 61 67",
"85 49 96",
"81 93 31",
"22 25 67",
"57 96 93",
"82 88 92",
"86 56 80",
"25 39 44",
"1",
"95",
"90",
"11",
"48",
"49",
"23",
"6",
"0",
"76",
"3",
"83",
"85",
"31",
"44",
"54",
"87",
"38",
"16",
"61",
"22",
"21",
"29"
]//610

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

guard let nkTemp = readLine2() else { fatalError("Bad input") }
let nk = nkTemp.split(separator: " ").map{ String($0) }

guard let n = Int(nk[0].trimmingCharacters(in: .whitespacesAndNewlines))
  else { fatalError("Bad input") }

guard let k = Int(nk[1].trimmingCharacters(in: .whitespacesAndNewlines))
  else { fatalError("Bad input") }

let roads: [[Int]] = AnyIterator{ readLine2() }.prefix(n - 1).map {
  let roadsRow: [Int] = $0.split(separator: " ").map {
    if let roadsItem = Int($0.trimmingCharacters(in: .whitespacesAndNewlines)) {
      return roadsItem
    } else { fatalError("Bad input") }
  }
  
  guard roadsRow.count == 3 else { fatalError("Bad input") }
  
  return roadsRow
}

guard roads.count == n - 1 else { fatalError("Bad input") }

let machines: [Int] = AnyIterator{ readLine2() }.prefix(k).map {
  if let machinesItem = Int($0.trimmingCharacters(in: .whitespacesAndNewlines)) {
    return machinesItem
  } else { fatalError("Bad input") }
}

guard machines.count == k else { fatalError("Bad input") }

let result = minTime(roads: roads, machines: machines)

print(result)
//print("test ok")
