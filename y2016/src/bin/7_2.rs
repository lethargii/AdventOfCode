use std::fs;
use regex::Regex;

fn get_abas(ip: &str) -> Vec<&str>{
    let mut abas: Vec<&str> = Vec::new();
    for i in 0..ip.len()-2{
        if &ip[i..i+1] == &ip[i+2..i+3] && &ip[i..i+1] != &ip[i+1..i+2]{
            abas.push(&ip[i..i+3]);
        }
    }
    return abas;
}

fn aba_to_bab(aba: &str) -> String{
    let mut bab: String = String::new();
    bab.push_str(&aba[1..2]);
    bab.push_str(&aba[0..1]);
    bab.push_str(&aba[1..2]);
    return bab;
}

fn main(){
    let input = fs::read_to_string("input").expect("Le fichier n'a pas pu être lu.");
    let mut nb_ip = 0;
    let re = Regex::new(r"[\[\]]").unwrap();
    for ligne in input.lines(){
        let ip: Vec<&str> = re.split(ligne).collect();
        let mut abas: Vec<&str> = Vec::new();
        for i in (0..ip.len()).step_by(2){
            for aba in get_abas(ip[i]){
                abas.push(aba);
            }
        }
        let babs: Vec<String> = abas.iter().map(|aba| aba_to_bab(aba)).collect::<Vec<String>>();
        'outer: for i in (1..ip.len()).step_by(2){
            for bab in babs.iter(){
                if ip[i].contains(bab){
                    nb_ip += 1;
                    break 'outer;
                }
            }
        }
    }
    println!("{nb_ip}");
}
