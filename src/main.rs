use std::io;

fn roman(num: u64) -> String {
    fn roman_rec(s: &mut String, num: u64) {
        fn roman_part(s: &mut String, num: u64, mul: u64, base: &str, mid: &str) {
            if num < 1 || num > 9 {
                // fall through
            } else if num == 1 {
                s.push_str(base);
            } else if num < 4 {
                roman_rec(s, 1 * mul);
                roman_rec(s, (num - 1) * mul);
            } else if num == 4 {
                roman_rec(s, 1 * mul);
                roman_rec(s, 5 * mul);
            } else if num == 5 {
                s.push_str(mid);
            } else if num < 9 {
                roman_rec(s, (num - 5) * mul);
                roman_rec(s, 5 * mul);
            } else {
                roman_rec(s, 1 * mul);
                roman_rec(s, 10 * mul);
            }
        }
        roman_part(s, num / 1000, 1000, "M", "MMMMM");
        roman_part(s, (num % 1000) / 100, 100, "C", "D");
        roman_part(s, (num % 100) / 10, 10, "X", "L");
        roman_part(s, num % 10, 1, "I", "V");
    }
    if num > 3999 {
        panic!("Requested number is unreasonably large.");
    }
    let mut s = String::with_capacity(24);
    roman_rec(&mut s, num);
    s
}

macro_rules! eq {
    ($a: expr, $b: expr) => {
        if $a != $b {
            panic!(concat!("Assertion failed: ",
                           stringify!($a),
                           " == ",
                           stringify!($b),
                           ": expected {}, got {}"), $a, $b);
        }
    }
}

#[test]
fn basic_conversion() {
    eq!("MCMLIV", &roman(1954));
    eq!("MCMXC", &roman(1990));
    eq!("MMXIV", &roman(2014));
    eq!("MMXIV", &roman(2015));
}

fn main() {
    loop {
        println!("Enter a number: ");
        let mut buf = String::with_capacity(24);
        if let None = io::stdin().read_line(&mut buf).ok() {
            println!("Failed to read input");
            return
        }
        let input: Option<u64> = buf.trim().parse().ok();
        match input {
            Some(num) => println!("{}", roman(num)),
            None => {
                println!("Not a number");
                return
            }
        }
    }
}
