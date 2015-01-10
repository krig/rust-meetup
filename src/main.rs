use std::ops::Add;
use std::io;
use std::num;

fn roman(num: usize) -> String {
    fn roman2(num: usize, mul: usize, base: &'static str, mid: &'static str) -> String {
        match num {
            1 => String::from_str(base),
            2...3 => roman(1 * mul) + roman((num - 1) * mul),
            4 => roman(1 * mul) + roman(5 * mul),
            5 => String::from_str(mid),
            6...8 => roman((num - 5) * mul) + roman(5 * mul),
            9 => roman(1 * mul) + roman(10 * mul),
            _ => String::from_str("")
        }
    }

    let mut s = roman2(num / 1000, 1000, "M", "MMMMM");
    s.push_str(roman2((num % 1000) / 100, 100, "C", "D").as_slice());
    s.push_str(roman2((num % 100) / 10, 10, "X", "L").as_slice());
    s.push_str(roman2(num % 10, 1, "I", "V").as_slice());
    s
}

#[test]
fn test_roman() {
    assert!(roman(1954).as_slice() == "MCMLIV")
    //std::assert!(roman(1990).as_slice() == "MCMXC")
    //std::assert!(roman(2014).as_slice() == "MMXIV")
}

fn main() {
    loop {
        println!("Enter a number: ");
        let input = io::stdin().read_line().ok().expect("Failed to read line");
        let input_num: Option<usize>  = input.parse();
        if let Some(num) = input_num {
           println!("{}", roman(num))
        } else {
          return
        }
    }
}
