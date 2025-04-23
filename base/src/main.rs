use base::*;

fn main() {
    let s = "AAA";
    let (s2, s8, s10, s16) = convert_all(&s, Radix::Hex).unwrap();
    println!("{}", s2);
    println!("{}", s8);
    println!("{}", s10);
    println!("{}", s16);
}
