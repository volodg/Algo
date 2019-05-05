#include <cmath>
#include <cstdio>
#include <vector>
#include <iostream>
#include <algorithm>
#include <stack>
#include <queue>
#include <map>
#include <unordered_map>

using namespace std;

vector<int> freqQuery(vector<vector<int>> queries) {
  
  vector<int> result;
  
  unordered_map <int, int> s1;
  unordered_map <int, int> s2;
  
  for (auto& query: queries) {
    auto const& cmd = query[0];
    auto const& num = query[1];
    switch (cmd) {
    case 1:
      {
        auto prev = s1[num];
        auto newVal = prev + 1;
        s1[num] = newVal;

        //remove old
        if (prev != 0) {
          auto oldFrIt = s2.find(prev);
          if (oldFrIt != s2.end()) {
            auto oldFr = oldFrIt->second;
            if (oldFr == 1) {
              s2.erase(prev);
            } else {
              s2[prev] = oldFr - 1;
            }
          }
        }
        
        //append new
        s2[newVal] = s2[newVal] + 1;
        
        break;
      }
    case 2:
      {
        auto prevIt = s1.find(num);
        if (prevIt != s1.end()) {
          auto prev = prevIt->second;
          auto newVal = prev - 1;
          if (newVal != 0) {
            s1[num] = newVal;
          } else {
            s1.erase(num);
          }
          
          if (newVal > 0) {
            s2[newVal] = s2[newVal] + 1;
          }
          
          auto oldFrIt = s2.find(prev);
          if (oldFrIt != s2.end()) {
            auto oldFr = oldFrIt->second;
            if (oldFr == 1) {
              s2.erase(prev);
            } else {
              s2[prev] = oldFr - 1;
            }
          }
        }
      }
      break;
    case 3:
      if (s2.find(num) != s2.end()) {
        result.push_back(1);
      } else {
        result.push_back(0);
      }
      break;
    default:
        break;
    }
  }

  
  return result;
}

int main() {
  
  freqQuery({{1, 5, 5, 25, 125}});
  
  return 0;
}
