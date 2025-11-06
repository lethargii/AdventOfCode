use std::fs;

fn main(){
    let input = fs::read_to_string("input").expect("Le fichier n'a pas pu être lu");
    let mut code: String = "".to_string();
    let code_btn = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let (mut x, mut y): (u32, u32) = (1, 1);
    for ligne in input.lines(){
        for character in ligne.chars(){
            match character{
                'U' => y = 0.max(y as i32 - 1) as u32,
                'D' => y = 2.min(y as i32 + 1) as u32,
                'R' => x = 2.min(x as i32 + 1) as u32,
                'L' => x = 0.max(x as i32 - 1) as u32,
                _ => {},
            }
            println!("{character}");
        }
        code.push_str(code_btn[(y * 3 + x) as usize]);
    }
    println!("{code}");
}
