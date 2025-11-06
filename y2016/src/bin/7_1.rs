use std::fs;
use regex::Regex;

fn palindrome(ip: &str) -> bool{
    for i in 0..ip.len()-3{
        if &ip[i..i+1] == &ip[i+3..i+4] && &ip[i+1..i+2] == &ip[i+2..i+3] && &ip[i..i+1] != &ip[i+1..i+2]{
            return true;
        }
    }
    return false;
}

fn main(){
    let input = fs::read_to_string("input").expect("Le fichier n'a pas pu être lu.");
    let mut nb_ip = 0;
    let re = Regex::new(r"[\[\]]").unwrap();
    for ligne in input.lines(){
        let ip: Vec<&str> = re.split(ligne).collect();
        let mut abba_out = false;
        let mut abba_in = true;
        for i in (0..ip.len()).step_by(2){
            if palindrome(ip[i]){
                abba_out = true;
                break;
            }
        }
        for i in (1..ip.len()).step_by(2){
            if palindrome(ip[i]){
                abba_in = false;
                break;
            }
        }
        if abba_in && abba_out{
            nb_ip += 1;
        }
    }
    println!("{nb_ip}");
}
