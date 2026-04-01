fn md5(hash: String) -> String{
    let size : usize = hash.len() * size_of::<char>() * 2 / 512;
    println!("{}", size);
    return hash;
}

fn main(){
    let input = "The quick brown fox jumps over the lazy dog".to_string();
    let mut num = 0;
    let mut hash = md5(input.to_string() + &num.to_string());
    while &hash[..5] != "00000" {
        num += 1;
        hash = md5(input.to_string() + &num.to_string());
    }
    println!("Première partie : {}", num);
}
