use std::io;

fn roman2(num: uint, mul: uint, base: String, mid: String) -> String {
    match num {
        1 => base,
        2...3 => roman(1 * mul) + roman((num - 1) * mul),
        4 => roman(1 * mul) + roman(5 * mul),
        5 => mid,
        6...8 => roman((num - 5) * mul) + roman(5 * mul),
        9 => roman(1 * mul) + roman(10 * mul),
        _ => "".to_string()
    }
}

fn roman(num: uint) -> String {
    roman2(num / 1000, 1000, "M".to_string(), "MMMMM".to_string()) + 
        roman2((num % 1000) / 100, 100, "C".to_string(), "D".to_string()) + 
        roman2((num % 100) / 10, 10, "X".to_string(), "L".to_string()) + 
        roman2(num % 10, 1, "I".to_string(), "V".to_string())
}

#[test]
fn test_roman() {
    assert!(roman(1954) == "MCMLIV".to_string())
    assert!(roman(1990) == "MCMXC".to_string())
    assert!(roman(2014) == "MMXIV".to_string())
}

fn main() {
    loop {
        println!("Enter a number: ");
        let input = io::stdin().read_line().ok().expect("Failed to read line");
        let input_num: Option<uint> = from_str(input.as_slice().trim());
        println!("{}", roman(input_num.unwrap()))
    }
}
