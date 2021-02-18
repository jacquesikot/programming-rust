use std::io::Write;
use std::str::FromStr;

pub fn cli() {
    let mut numbers: Vec<u64> = Vec::new();

    for arg in std::env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        writeln!(std::io::stderr(), "Usage: gcd NUMBER ...").unwrap();
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = euclid::gcd(d, *m);
    }
    println!("The greates common divisor of  {:?} is {}", numbers, d)
}
