#include <fstream>
#include <iostream>
#include <ostream>
#include <string>

int main(){
  std::ifstream fichier("input");
  int nb = 0;
  std::string suite;
  std::getline(fichier, suite);
  fichier.close();
  int range = suite.length()/2;
  for(int i = 0; i < suite.length(); i++){
    if(suite[i] == suite[(i+range)%suite.length()]){
      nb += suite[i] - '0';
    }
  }
  std::cout << nb << std::endl;
  return 0;
}
