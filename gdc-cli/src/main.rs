use std::io::Write;
use std::str::FromStr;

fn main() {
    let mut numbers = vec![];

    for arg in std::env::args().skip(1) {
        numbers.push(u64::from_str(&arg)
                        .expect("err parsing arg to u64"));
    }

    if numbers.len() == 0 {
        writeln!(std::io::stderr(), "Usage: gcd NUMBER ...")
                        .expect("err writing to stderr");
        std::process::exit(1);
    }

    let mut d = numbers[0];

    for n in numbers[1..].into_iter() {
        d = gdc(d, *n);
    }

    println!("The greatest common divisor of {:?} is {}",
            numbers, d);
}

fn gdc(mut x: u64, mut y: u64) -> u64 {
    // to find gdc of two ints
    // divide the bigger int by the smaller
    // set the bigger of the two to equal the remainder
    // you again have two ints
    // loop until you get a remainder
    // of zero
    // return the remaining non zero int

    assert!(x != 0 && y != 0);
    while y != 0 {
        if y < x {
            let t = y;
            y = x;
            x = t;
        }
        y %= x;
    }
    x
}