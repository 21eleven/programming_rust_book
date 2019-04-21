use num::Complex;
use std::str::FromStr;

fn main() {

}
/// Try to determine if `c` is in the Mandelbrot set, using at most `limit` 
/// iterations to decide
/// 
/// if the complex num is ever more than two units away from the origin
/// we know that it is not in the set
fn excape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    None
}

/// parse inputs like "200x100" or "1.0,0.5" into a
/// coordinate pair
fn parse_pair<T: FromStr>(s: &str, seperator: char) -> Option<(T, T)> {
    match s.find(seperator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index+1..])) {
                (Ok(l), Ok(r)) => Some((l,r)),
                _ => None
            }
        }
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10,20)));
}

/// Parse two floating point numbers into a complex number
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex(",-0.0634"), None);
    assert_eq!(parse_complex("1.25,-0.0634"),
                                Some(Complex { re: 1.25, im: -0.0634 }));
}

