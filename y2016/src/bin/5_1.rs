use std::f64;

fn md5(message: &str) -> &str{
    let k: Vec<u32>;
    const S: [u32; 64] = [
        7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22,
        5,  9, 14, 20, 5,  9, 14, 20, 5,  9, 14, 20, 5,  9, 14, 20,
        4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23,
        6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
    ];
    k = (0..64).map(|i| 2^32 * f64::floor(f64::sin(i as f64 + 1.)) as u32).collect::<Vec<u32>>();
    let mut a: u32 = 0x67452301;
    let mut b: u32 = 0xefcdab89;
    let mut c: u32 = 0x98badcfe;
    let mut d: u32 = 0x10325476;
    let mut message_pre: Vec<u8> = message.as_bytes().to_vec();
    message_pre.push(0x80);
    while message_pre.len() % 64 != 56{
        message_pre.push(0x00);
    }
    for j in 0..64{
        let mut aa: u32 = a;
        let mut bb: u32 = b;
        let mut cc: u32 = c;
        let mut dd: u32 = d;
        for i in 0..64{
        }
        a += aa;
        b += bb;
        c += cc;
        d += dd;
    }
    let hash: &str;
    return "";
}

fn main(){
    let input = "reyedfim";

}
