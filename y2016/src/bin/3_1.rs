use std::fs;

fn main(){
    let input = fs::read_to_string("input").expect("Le fichier n'a pas pu être lu.");
    let mut nb_triangle = 0;
    for ligne in input.lines(){
        let cotes: Vec<u32> = ligne.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect();
        if cotes[0] + cotes[1] > cotes[2] && cotes[1] + cotes[2] > cotes[0] && cotes[2] + cotes[0] > cotes[1]{
            nb_triangle += 1
        }
    }
    println!("{nb_triangle}")
}
