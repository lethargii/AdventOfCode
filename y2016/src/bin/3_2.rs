use std::fs;

fn main(){
    let input = fs::read_to_string("input").expect("Le fichier n'a pas pu être lu.");
    let mut nb_triangle = 0;
    let mut cotes: Vec<u32> = vec![];
    for i in 0..3{
        for ligne in input.lines(){
            cotes.push(ligne.split_whitespace().collect::<Vec<&str>>()[i].parse::<u32>().unwrap());
            if cotes.len() == 3{
                if cotes[0] + cotes[1] > cotes[2] && cotes[1] + cotes[2] > cotes[0] && cotes[2] + cotes[0] > cotes[1]{
                    nb_triangle += 1
                }
                cotes = vec![]
            }
        }
    }
    println!("{nb_triangle}")
}
