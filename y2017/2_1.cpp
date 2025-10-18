#include <fstream>
#include <iostream>
#include <string>

int sum = 0;
int maxi;
int mini;
std::string value_raw;

void update_value(){
  int value = std::stoi(value_raw);
  value_raw = "";
  if(value > maxi){
    maxi = value;
  }
  if(value < mini){
    mini = value;
  }
}

int main(){
  std::ifstream fichier("input");
  sum = 0;
  std::string ligne;
  while(std::getline(fichier, ligne)){
    maxi = 0;
    mini = 9999;
    value_raw = "";
    for(int i = 0; i < ligne.length(); i++){
      if(ligne[i] == '\t'){
        if(value_raw != ""){
          update_value();
        }
      }
      else{
        value_raw += ligne[i];
      }
    }
    update_value();
    sum += maxi - mini;
  }
  std::cout << sum << std::endl;
  fichier.close();
  return 0;
}
