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
    let pattern = Regex::new(r"mul\((\d*),(\d*)\)").unwrap();
    let mut results: Vec<(i32, i32)> = vec![];
    for (_, [num1_str, num2_str]) in pattern.captures_iter(&contents).map(|c| c.extract()) {
        results.push((num1_str.parse::<i32>().unwrap(), num2_str.parse::<i32>().unwrap()));
    }

    let mut sum = 0;
    for (num1, num2) in results {
        println!("mul({num1}, {num2})");
        let product = num1*num2;
        sum = sum + product;
    }

    println!();
    println!("The sum is: {sum}");

}