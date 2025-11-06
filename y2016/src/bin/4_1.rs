use std::fs;
use std::collections::HashMap;

fn main(){
    let input = fs::read_to_string("input").expect("Le fichier n'a pas pu être lu.");
    let mut sum = 0;
    for ligne in input.lines(){
        let encrypted_name = &ligne[..ligne.len() - 7];
        let checksum_input = &ligne[ligne.len() - 6..ligne.len() - 1];
        let decrypted_names = encrypted_name.split("-").collect::<Vec<&str>>();
        let decrypted_name = decrypted_names[..decrypted_names.len()-1].join("");
        let sector_id = decrypted_names[decrypted_names.len()-1].parse::<i32>().unwrap();
        let mut dico: HashMap<char, usize> = HashMap::new();
        for c in decrypted_name.chars(){
            *dico.entry(c).or_insert(0) += 1
        }
        let mut count_list: Vec<(&char, &usize)> = dico.iter().collect();
        count_list.sort_by_key(|x| x.0);
        count_list.reverse();
        count_list.sort_by_key(|x| x.1);
        count_list.reverse();
        let checksum_calc : String = (0..5).map(|i| count_list[i].0).collect::<String>();
        if checksum_input == checksum_calc{
            sum += sector_id;
        }
    }
    println!("{sum}");
}
