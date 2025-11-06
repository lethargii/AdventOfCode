using System;
using System.IO;
using System.Collections.Generic;

int frequency = 0;
List<int> frequencies = new List<int>();
try{
  string[] lignes = File.ReadAllLines("input");
  while(!frequencies.Contains(frequency)){
    foreach(string ligne in lignes){
      frequencies.Add(frequency);
      if(ligne[0] == '+'){
        frequency += int.Parse(ligne.Substring(1));
      }
      else{
        frequency -= int.Parse(ligne.Substring(1));
      }
      if(frequencies.Contains(frequency)){
        break;
      }
    }
    Console.WriteLine(frequency);
  }
}
catch{
  Console.WriteLine("Le fichier n'a pas pu être lu.");
}
