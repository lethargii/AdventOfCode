using System;
using System.IO;
using System.Collections.Generic;

int nb_twos = 0;
int nb_threes = 0;
try{
  string[] lignes = File.ReadAllLines("input");
    foreach(string ligne in lignes){
      Dictionary<char, int> occurences = new Dictionary<char, int>();
      foreach(char chara in ligne){
        if(occurences.ContainsKey(chara)){
          occurences[chara]++;
        }
        else{
          occurences[chara] = 1;
        }
      }
      if(occurences.ContainsValue(2)){
        nb_twos++;
      }
      if(occurences.ContainsValue(3)){
        nb_threes++;
      }
    }
    Console.WriteLine(nb_twos * nb_threes);
}
catch{
  Console.WriteLine("Le fichier n'a pas pu être lu.");
}
