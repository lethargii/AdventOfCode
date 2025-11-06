using System;
using System.IO;

int res = 0;
try{
  using(StreamReader sr = new StreamReader("input")){
    string ligne;
    while((ligne = sr.ReadLine()) != null){
      if(ligne[0] == '+'){
        res += int.Parse(ligne.Substring(1));
      }
      else{
        res -= int.Parse(ligne.Substring(1));
      }
    }
  }
  Console.WriteLine(res);
}
catch{
  Console.WriteLine("Le fichier n'a pas pu être lu.");
}
