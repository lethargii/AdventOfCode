use std::fs;
use regex::Regex;

fn main(){
    let input = fs::read_to_string("input").expect("Le fichier n'a pas pu être lu.");
    let re: Regex = Regex::new(r"[()]").expect("Expression non valide");
    for ligne in input.lines(){
        let sections_raw: Vec<&str> = re.split(ligne).collect();
        let sections: Vec<String> = (0..sections_raw.len()).map(|i| {
            if i%2 == 0{
                sections_raw[i].to_string()
            }
            else{
                format!("({})", sections_raw[i])
            }
        }).collect();
        let mut decompressed = "".to_string();
        let mut decompressed_pattern = "".to_string();
        let mut i = 0;
        let mut j = 0;
        let mut count_letters = 0;
        let mut times_letters = 1;
        while i < sections.len(){
            while count_letters != 0 && i < sections.len(){
                if j < sections[i].len(){
                    decompressed_pattern.push_str(&sections[i][j..j+1]);
                    j += 1;
                    count_letters -= 1;
                }
                else{
                    i += 1;
                    j = 0;
                }
            }
            for _i in 0..times_letters{
                decompressed.push_str(&decompressed_pattern);
            }
            decompressed_pattern = "".to_string();
            if i < sections.len(){
                if i%2 == 0 || j != 0{
                    decompressed.push_str(&sections[i][j..]);
                    j = 0;
                }
                else{
                    let marker: Vec<&str> = sections[i].split("x").collect();
                    count_letters = marker[0][1..].parse::<i32>().expect("La chaine n'est pas numérique");
                    times_letters = marker[1][..marker[1].len()-1].parse::<i32>().expect("La chaine n'est pas numérique");
                }
                i += 1;
            }
        }
        println!("{decompressed}");
        println!("{}", decompressed.len());
    }
}
