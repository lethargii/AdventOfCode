#include <fstream>
#include <iostream>
#include <string>
#include <vector>

int sum = 0;
int maxi;
int mini;
std::string value_raw;
std::vector<int> liste;

int evenly_divisible(){
    for(int i = 0; i < liste.size(); i++){
      for(int j = 0; j < i; j++){
        if(liste[i] % liste[j] == 0){
          return liste[i] / liste[j];
        }
        if(liste[j] % liste[i] == 0){
          return liste[j] / liste[i];
        }
      }
      for(int j = i + 1; j < liste.size(); j++){
        if(liste[i] % liste[j] == 0){
          return liste[i] / liste[j];
        }
        if(liste[j] % liste[i] == 0){
          return liste[j] / liste[i];
        }
      }
    }
    return -1;
}

int main(){
  std::ifstream fichier("input");
  sum = 0;
  std::string ligne;
  while(std::getline(fichier, ligne)){
    std::string value_raw = "";
    for(int i = 0; i < ligne.length(); i++){
      if(ligne[i] == '\t' || ligne[i] == ' '){
        if(value_raw != ""){
          int value = std::stoi(value_raw);
          value_raw = "";
          liste.push_back(value);
        }
      }
      else{
        value_raw += ligne[i];
      }
    }
    int value = std::stoi(value_raw);
    value_raw = "";
    liste.push_back(value);
    for(int i = 0; i < liste.size(); i++){
      std::cout << liste[i] << std::endl;
    }
    int test = evenly_divisible();
    std::cout << test << std::endl;
    sum += test;
    liste.clear();
  }
  std::cout << sum << std::endl;
  fichier.close();
  return 0;
}
