use number_calc::primary::FactMap;
use number_calc::sqrt::Sqrt;

fn main() {
    let x = 2*2*2*5*5*7*7*7*13;
    println!("{}", x);
    println!("{}", FactMap::new(x).unwrap().to_mathjax());
    println!("{}", Sqrt::from_sqrt(x).unwrap().to_mathjax());
}
