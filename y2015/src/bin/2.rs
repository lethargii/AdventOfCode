use std::fs;

fn main(){
    let contenu = fs::read_to_string("inputs/input2").expect("Le fichier n'a pas pu être lu.");
    let mut wrapping_paper_area = 0;
    let mut ribbon_length = 0;
    for line in contenu.lines(){
        let mut dimension: Vec<u64> = line.split('x').map(|x| x.parse::<u64>().expect("La chaine n'est pas numérique.")).collect();
        dimension.sort();
        let (l, w, h) = (dimension[0], dimension[1], dimension[2]);
        let (lw, wh, hl) = (l * w, w * h, h * l);
        wrapping_paper_area += 2 * lw + 2 * wh + 2 * hl + lw.min(wh.min(hl));
        ribbon_length += 2 * (l + w) + l * w * h;
    }
    println!("Première partie : {}", wrapping_paper_area);
    println!("Deuxième partie : {}", ribbon_length);
}
