use std::env;
use std::fs;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("You need to specify a filename input argument")
    }

    let filepath = &args[1];

    let contents = fs::read_to_string(filepath).expect("Cannot read contents of file");

    let pattern = Regex::new(r"do\(\)|don't\(\)|mul\((\d*),(\d*)\)").unwrap();
    
    let mut enabled = true;
    let mut results: Vec<(i32, i32)> = vec![];
    let mut sum:i32 = 0;
    for caps in pattern.captures_iter(&contents) {
        let cap = caps.get(0).map(|m| m.as_str()).unwrap();
        match cap {
            "do()" => enabled = true,
            "don't()" => enabled =false,
            _ => {
                if enabled {
                    let mul_pattern = Regex::new(r"mul\((\d*),(\d*)\)").unwrap();
                    let (_, [num1_str, num2_str]) = mul_pattern.captures(cap).unwrap().extract();
                    let num1 = num1_str.parse::<i32>().unwrap();
                    let num2 = num2_str.parse::<i32>().unwrap();
                    println!("num1 is {num1} num2 is {num2}");
                    let product = num1 * num2;
                    sum = sum + product;
                }
            },
        }
    };

    println!();
    println!("The sum is: {sum}");

}