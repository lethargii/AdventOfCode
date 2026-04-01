use std::fs;

struct Way {
    way: Vec<(i64, i64)>,
}

impl Way {
    fn new() -> Self{
        return Self{way: Vec::new()};
    }
    fn add(&mut self, x: i64, y: i64){
        for i in 0..self.way.len(){
            if (x, y) == self.way[i]{
                return;
            }
        }
        self.way.push((x, y));
    }
    fn nb_houses(&self) -> usize{
        return self.way.len();
    }
}

struct Santa {
    x: i64,
    y: i64,
}

impl Santa {
    fn new() -> Self{
        return Self{x: 0, y: 0,}
    }
    fn move_santa(&mut self, arrow: char, way: &mut Way){
        match arrow {
            '^' => self.y += 1,
            '>' => self.x += 1,
            '<' => self.x -= 1,
            'v' => self.y -= 1,
            _ => {},
        }
        way.add(self.x, self.y);
    }
}

fn main(){
    let contenu = fs::read_to_string("inputs/input3").expect("Le fichier n'a pas pu être lu.");
    let mut way: Way = Way::new();
    let mut santa: Santa = Santa::new();
    let mut way_robot: Way = Way::new();
    let mut santa_human: Santa = Santa::new();
    let mut santa_robot: Santa = Santa::new();
    for (i, arrow) in contenu.chars().enumerate(){
        santa.move_santa(arrow, &mut way);
        if i % 2 == 0{
            santa_human.move_santa(arrow, &mut way_robot);
        }
        else{
            santa_robot.move_santa(arrow, &mut way_robot);
        }
    }
    println!("Première partie : {}", way.nb_houses());
    println!("Deuxième partie : {}", way_robot.nb_houses());
}
