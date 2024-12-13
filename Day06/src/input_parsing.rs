use std::env;
use std::fmt::Debug;
use std::fs;
use std::str::FromStr;

pub fn get_raw_input_arg1() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("\r\nERROR: NO INPUT FILE SPECIFIED\r\n");
    }

    let filepath = &args[1];
    fs::read_to_string(filepath).expect("ERROR: CANNOT ACCESS FILE")
}

pub fn string_to_2d_vec<T>(input :String, mut vec :Vec<Vec<T>>) -> Vec<Vec<T>> 
where 
    T: FromStr,
    T::Err: Debug,
{
    for line in input.lines() {
        let str_vec : Vec<&str> = line.split_whitespace().collect();
        let mut line_vec : Vec<T> = vec![];
        for s in str_vec {
            line_vec.push(s.parse::<T>().unwrap());
        }
        vec.push(line_vec);
    }
    vec
}

pub fn string_to_2d_vec_chars(input :String, mut vec :Vec<Vec<char>>) -> Vec<Vec<char>> {
    for line in input.lines() {
        vec.push(line.chars().collect());
    }
    vec
}
