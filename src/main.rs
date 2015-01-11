use std::io;

fn roman(num: i32) -> String {
    fn roman2(num: i32, mul: i32, base: &'static str, mid: &'static str) -> String {
        match num {
            1 => String::from_str(base),
            2...3 => roman(1 * mul) + &roman((num - 1) * mul)[],
            4 => roman(1 * mul) + &roman(5 * mul)[],
            5 => String::from_str(mid),
            6...8 => roman((num - 5) * mul) + &roman(5 * mul)[],
            9 => roman(1 * mul) + &roman(10 * mul)[],
            _ => String::from_str("")
        }
    }

    roman2(num / 1000, 1000, "M", "MMMMM") +
        &roman2((num % 1000) / 100, 100, "C", "D")[] +
        &roman2((num % 100) / 10, 10, "X", "L")[] +
        &roman2(num % 10, 1, "I", "V")[]
}

#[test]
fn test_roman() {
    assert!(&roman(1954)[] == "MCMLIV");
    assert!(&roman(1990)[] == "MCMXC");
    assert!(&roman(2014)[] == "MMXIV");
}

fn main() {
    loop {
        println!("Enter a number: ");
        let input: Option<i32>  = io::stdin()
            .read_line()
            .ok()
            .expect("Failed to read line")
            .trim()
            .parse();
        if let Some(num) = input {
            println!("{}", roman(num))
        } else {
            println!("Je suis Charlie");
            return
        }
    }
}
