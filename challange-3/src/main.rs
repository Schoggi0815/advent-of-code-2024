use std::fs;

use regex::Regex;

fn main() {
    let text_input = fs::read_to_string("./input.txt").unwrap();
    let regex =
        Regex::new(r"(mul\((\d+),(\d+)\))|(do\(\)()())|(don't\(\)()())").expect("Invalid regex");
    let captures = regex.captures_iter(&text_input).map(|c| c.extract());

    let mut sum = 0;
    let mut do_mul = true;
    for (match_text, [_, mul1, mul2]) in captures {
        match match_text {
            "do()" => {
                do_mul = true;
            }
            "don't()" => {
                do_mul = false;
            }
            _ => {
                let mul1 = mul1.parse::<i32>().expect("Invalid number");
                let mul2 = mul2.parse::<i32>().expect("Invalid number");

                if do_mul {
                    sum += mul1 * mul2;
                }
            }
        }
    }

    println!("{}", sum);
}
