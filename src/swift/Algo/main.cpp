#include <cmath>
#include <cstdio>
#include <vector>
#include <iostream>
#include <algorithm>
#include <stack>
#include <queue>
using namespace std;

struct Node {
  int data;
  Node* left;
  Node* right;
};

bool checkBST(Node* root, int* leftLimit, int* rightLimit) {
  if (root == nullptr) { return true; }
  
  if (leftLimit != nullptr && root->data <= *leftLimit) {
    return false;
  }
  
  if (rightLimit != nullptr && root->data >= *rightLimit) {
    return false;
  }
  
  return checkBST(root->left, leftLimit, &root->data) && checkBST(root->right, &root->data, rightLimit);
}

bool checkBST(Node* root) {
  return checkBST(root, nullptr, nullptr);
}

int main() {
  /* Enter your code here. Read input from STDIN. Print output to STDOUT */
  return 0;
}

