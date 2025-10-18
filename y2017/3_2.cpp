#include <algorithm>
#include <cstdlib>
#include <iostream>
#include <vector>
int squareInput = 277678;
std::vector<int> squares = {1, 1, 2};

int main(){
  int length = 3;
  int i = 0;
  int j = 0;
  int u = 1;
  int v = 0;
  int k = 0;
  while(squares[length - 1] <= squareInput){
    k = std::max(std::abs(i), std::abs(j));
    std::cout << squares[length - 1] << std::endl;
    length++;
    squares.push_back(squares[length - 1]);
  }  
  std::cout << squares[length - 1] << std::endl;
}
