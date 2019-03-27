#include <cmath>
#include <cstdio>
#include <vector>
#include <iostream>
#include <algorithm>
#include <stack>
#include <queue>

using namespace std;

struct Player {
  int score;
  string name;
};

class Checker{
public:
  // complete this method
  static int compare(int a, int b)  {
    if (a < b) {
      return -1;
    } else if (a > b) {
      return 1;
    }
    return 0;
  }
  static int comparator(Player a, Player b)  {
    int result = compare(a.score, b.score);
    if (result == 0) {
      auto res1 = b.name.compare(a.name);
      return (res1 > 0) ? 1 : ((res1 < 0) ? -1 : 0);
    }
    return result;
  }
};




bool compare(Player a, Player b) {
  if(Checker::comparator(a,b) == -1)
    return false;
  return true;
}
int main()
{
  int n;
  cin >> n;
  vector< Player > players;
  string name;
  int score;
  for(int i = 0; i < n; i++){
    cin >> name >> score;
    Player player;
    player.name = name;
    player.score = score;
    players.push_back(player);
  }
  sort(players.begin(), players.end(), compare);
  for(int i = 0; i < players.size(); i++) {
    cout << players[i].name << " " << players[i].score << endl;
  }
  return 0;
}
