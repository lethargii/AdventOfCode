use std::fs;

fn main(){
    let input = fs::read_to_string("input").expect("Le fichier n'a pas pu être lu");
    let mut code: String = "".to_string();
    let code_btn = vec!["", "", "1", "", "", "", "2", "3", "4", "", "5", "6", "7", "8", "9", "", "A", "B", "C", "", "", "", "D", "", ""];
    let (mut x, mut y): (u32, u32) = (0, 2);
    for ligne in input.lines(){
        for character in ligne.chars(){
            match character{
                'U' => y = (0 + (x as i32 - 2).abs()).max(y as i32 - 1) as u32,
                'D' => y = (4 - (x as i32 - 2).abs()).min(y as i32 + 1) as u32,
                'R' => x = (4 - (y as i32 - 2).abs()).min(x as i32 + 1) as u32,
                'L' => x = (0 + (y as i32 - 2).abs()).max(x as i32 - 1) as u32,
                _ => {},
            }
            println!("{character}");
        }
        code.push_str(code_btn[(y * 5 + x) as usize]);
    }
    println!("{code}");
}
