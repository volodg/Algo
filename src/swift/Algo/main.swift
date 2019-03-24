import Foundation

final class DoublyLinkedListNode {
  var data: Int
  var next: DoublyLinkedListNode?
  weak var prev: DoublyLinkedListNode?
  //var prev: DoublyLinkedListNode?
  
  public init(nodeData: Int) {
    self.data = nodeData
  }
}

final class DoublyLinkedList {
  var head: DoublyLinkedListNode?
  var tail: DoublyLinkedListNode?
  
  public init() {}
  
  public func insertNode(nodeData: Int) {
    self.insertNode(node: DoublyLinkedListNode(nodeData: nodeData))
  }
  
  private func insertNode(node: DoublyLinkedListNode) {
    if let tail = tail {
      tail.next = node
      node.prev = tail
    } else {
      head = node
    }
    
    tail = node
  }
}

func printDoublyLinkedList(head: DoublyLinkedListNode?, sep: String) {
  var node = head
  
  while node != nil {
    //fileHandle.write(String(node!.data).data(using: .utf8)!)
    print(node!.data, terminator: "")
    
    node = node!.next
    
    if node != nil {
      //fileHandle.write(sep.data(using: .utf8)!)
      print(sep, terminator: "")
    }
  }
}

func reverse(llist head: DoublyLinkedListNode?) -> DoublyLinkedListNode? {
  guard let head = head else { return nil }
  
  var curr = head
  var prevHolder: DoublyLinkedListNode? = nil
  
  while (curr.next != nil) {
    let next = curr.next!
    let prev = prevHolder
    prevHolder = curr
    curr.next = prev
    curr.prev = next
    
    curr = next
  }
  
  curr.next = curr.prev
  curr.prev = nil
  
  return curr
}//[3 +2 -> +1]

////test 1
//var strings = [
//  "1",
//  "4",
//  "1",
//  "2",
//  "3",
//  "4"
//]

//test 2
var strings = [
  "1",
  "4",
  "1",
  "2",
  "3",
  "4"
]

var index = 0

func readLine2() -> String? {
  if index >= strings.count {
    return nil
  }
  let result = strings[index]
  index += 1
  return result
}

guard let t = Int((readLine2()?.trimmingCharacters(in: .whitespacesAndNewlines))!)
  else { fatalError("Bad input") }

for _ in 1...t {
  guard let llistCount = Int((readLine2()?.trimmingCharacters(in: .whitespacesAndNewlines))!)
    else { fatalError("Bad input") }
  
  let llist = DoublyLinkedList()
  
  for _ in 1...llistCount {
    guard let llistItem = Int((readLine2()?.trimmingCharacters(in: .whitespacesAndNewlines))!)
      else { fatalError("Bad input") }
    llist.insertNode(nodeData: llistItem)
  }
  
  let llist1 = reverse(llist: llist.head!)
  
  printDoublyLinkedList(head: llist1, sep: " ")
  print("")
  //fileHandle.write("\n".data(using: .utf8)!)
}
