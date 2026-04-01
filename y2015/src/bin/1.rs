use std::fs;

fn main(){
    let contenu = fs::read_to_string("inputs/input1").expect("Le fichier n'a pas pu être lu.");
    let mut floor: i64 = 0;
    for chara in contenu.chars(){
        match chara {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {},
        }
    }
    println!("Première partie : {}", floor);
    floor = 0;
    let mut i = 0;
    for chara in contenu.chars(){
        i += 1;
        match chara {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {},
        }
        if floor == -1{
            break;
        }
    }
    println!("Deuxième partie : {}", i);
}
