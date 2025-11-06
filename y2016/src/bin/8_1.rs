use std::fs;

fn draw_matrix(width: usize, height: usize, matrix: &Vec<bool>){
    for i in 0..height{
        for j in 0..width{
            if matrix[i * width + j]{
                print!("#");
            }
            else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn main(){
    let input = fs::read_to_string("input").expect("Le fichier n'a pas pu être lu.");
    let width = 50;
    let height = 6;
    let mut matrix = vec![false; width * height];
    for ligne in input.lines(){
        let instructions: Vec<&str> = ligne.split_whitespace().collect();
        match instructions[0]{
            "rect" => {
                let ab: Vec<&str> = instructions[1].split("x").collect();
                let a = ab[0].parse::<usize>().expect("Ce n'est pas une chaine numérique");
                let b = ab[1].parse::<usize>().expect("Ce n'est pas une chaine numérique");
                for i in 0..a{
                    for j in 0..b{
                        matrix[i + j * width] = true;
                    }
                }
            },
            "rotate" => {
                match instructions[1]{
                    "row" => {
                        let a = instructions[2].split("=").collect::<Vec<&str>>()[1].parse::<usize>().expect("Ce n'est pas une chaine numérique");
                        let b = instructions[4].parse::<usize>().expect("Ce n'est pas une chaine numérique");
                        let pivot: Vec<bool> = (width-b..width).map(|k| matrix[k + a * width]).collect::<Vec<bool>>();
                        for k in (b..width).rev(){
                            matrix[k + a * width] = matrix[k-b + a * width];
                        }
                        for k in 0..b{
                            matrix[k + a * width] = pivot[k];
                        }
                    },
                    "column" => {
                        let a = instructions[2].split("=").collect::<Vec<&str>>()[1].parse::<usize>().expect("Ce n'est pas une chaine numérique");
                        let b = instructions[4].parse::<usize>().expect("Ce n'est pas une chaine numérique");
                        let pivot: Vec<bool> = (height-b..height).map(|k| matrix[a + k * width]).collect::<Vec<bool>>();
                        for k in (b..height).rev(){
                            matrix[k * width + a] = matrix[(k-b) * width + a];
                        }
                        for k in 0..b{
                            matrix[k * width + a] = pivot[k];
                        }
                    },
                    _ => {}
                }
            },
            _ => {}
        }
        draw_matrix(width, height, &matrix);
    }
    let mut nb_lights = 0;
    for light in matrix{
        if light{
            nb_lights += 1;
        }
    }
    println!("{nb_lights}");
}
