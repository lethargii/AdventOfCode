use std::fs;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
enum Direction{
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl Direction{
    fn move_dir(self, dir: char) -> Option<Direction>{
        if dir == 'L'{
            match self as u8{
                0 => Some(Direction::West),
                1 => Some(Direction::North),
                2 => Some(Direction::East),
                3 => Some(Direction::South),
                _ => None,
            }
        }
        else{
            match self as u8{
                0 => Some(Direction::East),
                1 => Some(Direction::South),
                2 => Some(Direction::West),
                3 => Some(Direction::North),
                _ => None,
            }
        }
    }
}

fn main(){
    let input = fs::read_to_string("input").expect("Le fichier n'a pas pu être lu.");
    let (mut x, mut y): (i32, i32) = (0, 0);
    let mut direction: Direction = Direction::North;
    for instruction in input.trim().split(", "){
        direction = direction.move_dir(instruction.chars().nth(0).expect("Le charactère n'existe pas")).unwrap();
        let move_nb = &instruction[1..].parse().expect("La chaine n'est pas numérique");
        match direction{
            Direction::North => y += move_nb,
            Direction::East => x += move_nb,
            Direction::South => y -= move_nb,
            Direction::West => x -= move_nb,
        }
    }
    println!("{}", x.abs() + y.abs());
}
