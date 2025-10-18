#include <fstream>
#include <iostream>
#include <string>

int main(){
  std::ifstream fichier("input");
  int nb;
  char chara;
  fichier.get(chara);
  char chara0 = chara;
  char chara_prec = chara;
  while(fichier.get(chara)){
    if(chara == '\n'){
      continue;
    }
    if(chara == chara_prec){
      nb += chara - '0';
    }
    chara_prec = chara;
  }
  nb += chara_prec - '0';
  fichier.close();
  std::cout << nb << std::endl;
  return 0;
}
