use std::fs;
use std::collections::HashMap;

fn main(){
    let input = fs::read_to_string("input").expect("Le fichier n'a pas pu être lu");
    let mut decoded_message = "".to_string();
    for i in 0..input.lines().collect::<Vec<&str>>()[0].chars().collect::<Vec<char>>().len(){
        let mut dico: HashMap<char, i32> = HashMap::new();
        for ligne in input.lines(){
            *dico.entry(ligne.chars().collect::<Vec<char>>()[i]).or_insert(0) += 1;
        }
        let cle_max: char = dico.iter().min_by_key(|&(_, v)| v).map(|(&k, _)| k).unwrap();
        decoded_message.push(cle_max);
    }
    println!("{decoded_message}");
}
