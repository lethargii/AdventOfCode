#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

int main(){
  std::ifstream fichier("input");
  std::string ligne;
  int nb_correct = 0;
  while(std::getline(fichier, ligne)){
    std::vector<std::string> words = {};
    bool word_repeated = false;
    std::istringstream iss(ligne);
    std::string word;
    while(iss >> word){
      for(int i = 0; i < words.size(); i++){
        if(word == words[i]){
          word_repeated = true;
          break;
        }
      }
      if(word_repeated){
        break;
      }
      words.push_back(word);
    }
    if(!word_repeated){
      nb_correct++;
    }
  }
  std::cout << nb_correct << std::endl;
}
