#include <cmath>
#include <cstdio>
#include <vector>
#include <iostream>
#include <algorithm>
#include <stack>
#include <queue>

using namespace std;

void min_candies_update(size_t offset, const vector<int>& ratings, vector<int>& candies) {
  
  auto ratings_len = ratings.size() - offset;
  
  if (ratings_len == 0 || ratings_len == 1) {
    return;
  }
  
  if (ratings[offset + 0] < ratings[offset + 1]) {
    candies[offset + 1] = max(candies[offset + 1], candies[offset + 0] + 1);
  }
  
  min_candies_update(offset + 1, ratings, candies);
  
  if (ratings[offset + 0] > ratings[offset + 1]) {
    candies[offset + 0] = max(candies[offset + 0], candies[offset + 1] + 1);
  }

}

// Complete the candies function below.
long candies(int n, vector<int> ratings) {
  vector<int> candies(ratings.size());
  for (size_t i = 0; i < ratings.size(); ++i) {
    candies[i] = 1;
  }
  
  min_candies_update(0, ratings, candies);
  
  int sum_of_elems = 0;
  std::for_each(candies.begin(), candies.end(), [&] (int n) {
    sum_of_elems += n;
  });
  
  return sum_of_elems;
}

int main()
{
  //ofstream fout(getenv("OUTPUT_PATH"));
  
//  int n;
//  cin >> n;
//  cin.ignore(numeric_limits<streamsize>::max(), '\n');
  
  //vector<int> arr = {1, 2, 2};
  vector<int> arr = {4, 6, 4, 5, 6, 2};
  
//  for (int i = 0; i < n; i++) {
//    int arr_item;
//    cin >> arr_item;
//    cin.ignore(numeric_limits<streamsize>::max(), '\n');
//
//    arr[i] = arr_item;
//  }
  
  long result = candies(3, arr);
  
  cout << result << "\n";
  
  //fout.close();
  
  return 0;
}
