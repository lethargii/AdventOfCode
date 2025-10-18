#include <cstdlib>
#include <iostream>
int squareInput = 277678;

int main(){
  int square = 2;
  int k = 0;
  int i = 0;
  int j = 1;
  while(square < squareInput){
    std::cout << square << std::endl;
    if(square + 8 * (k + 1) <= squareInput){
      square += 8 * (k + 1);
      k++;
      i++;
      j++;
    }
    else{
      if(squareInput - square > 0){
        i -= std::min(1 + 2 * k, squareInput - square);
        square += std::min(1 + 2 * k, squareInput - square);
      }
      if(squareInput - square > 0){
        j -= std::min(2 * (k + 1), squareInput - square);
        square += std::min(2 * (k + 1), squareInput - square);
      }
      if(squareInput - square > 0){
        i += std::min(2 * (k + 1), squareInput - square);
        square += std::min(2 * (k + 1), squareInput - square);
      }
      if(squareInput - square > 0){
        j += std::min(2 * (k + 1), squareInput - square);
        square += std::min(2 * (k + 1), squareInput - square);
      }
    }
  }
  std::cout << squareInput << std::endl;
  std::cout << square << std::endl;
  int res = abs(i) + abs(j);
  std::cout << res << std::endl;
}
