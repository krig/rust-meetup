use std::io;

fn roman(num: u64) -> String {
    fn roman2(num: u64, mul: u64, base: &'static str, mid: &'static str) -> String {
        match num {
            1 => base.to_string(),
            2...3 => format!("{}{}", roman(1 * mul), roman((num - 1) * mul)),
            4 => format!("{}{}", roman(1 * mul), roman(5 * mul)),
            5 => mid.to_string(),
            6...8 => format!("{}{}", roman((num - 5) * mul), roman(5 * mul)),
            9 => format!("{}{}", roman(1 * mul), roman(10 * mul)),
            _ => "".to_string()
        }
    }

    format!("{}{}{}{}",
            roman2(num / 1000, 1000, "M", "MMMMM"),
            roman2((num % 1000) / 100, 100, "C", "D"),
            roman2((num % 100) / 10, 10, "X", "L"),
            roman2(num % 10, 1, "I", "V"))
}

macro_rules! assert_eq {
    ($a: expr, $b: expr) => {
        {
            let a = $a;
            let b = $b;
            assert!(a == b);
        }
    }
}

#[test]
fn basic_conversion() {
    assert_eq!(roman(1954), "MCMLIV".to_string());
    assert_eq!(roman(1990), "MCMXC".to_string());
    assert_eq!(roman(2014), "MMXIV".to_string());
}

fn main() {
    loop {
        println!("Enter a number: ");
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).ok().expect("failed to read line");
        let input: Option<u64> = buf.trim().parse().ok();
        if let Some(num) = input {
            println!("{}", roman(num))
        } else {
            println!("Je suis Charlie");
            return
        }
    }
}
