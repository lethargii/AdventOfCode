#include <algorithm>
#include <cstdlib>
#include <iostream>
#include <vector>
int squareInput = 277678;
std::vector<int> squares = {1, 1};

int main(){
  int length = 3;
  int k = 1;
  int next_k = 9;
  while(squares[length - 1] <= squareInput){
    if(k * 8){
      k++;
    }
    std::cout << squares[length - 1] << std::endl;
    length++;
    squares.push_back(squares[length - 1]);
  }  
  std::cout << squares[length - 1] << std::endl;
}
