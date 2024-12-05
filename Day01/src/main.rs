use std::env;
use std::fs;
use std::iter;
use std::string;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("You need to specify a filename input argument")
    }

    let filepath = &args[1];

    let contents = fs::read_to_string(filepath).expect("Cannot read contents of file");

    let num_slices = contents.split_whitespace();

    let mut list1: Vec<i32> = vec![];
    let mut list2: Vec<i32> = vec![];

    let mut cnt = 0;
    for num_str in num_slices {
        if cnt % 2 == 0 {
            list1.push(num_str.parse().unwrap());
        }
        else {
            list2.push(num_str.parse().unwrap());
        }
        cnt = cnt + 1;
    }

    list1.sort();
    list2.sort();


    let mut diff :i32 = 0;
    for i in 0..list1.len() {
        let left = list1[i];
        let right = list2[i];
        diff = diff + (right - left).abs();
        // println!("left list = {left}  right list = {right}");
        // println!("new diff is {diff}");
    }
    println!("difference is {diff}");

    let mut similarity_score:i32 = 0;
    for item1 in &list1 {
        let mut copies = 0;
        for item2 in &list2 {
            if item2 == item1 {
                copies = copies+1;
            }
            if item2 > item1 {
                break;
            }
        }
        similarity_score = similarity_score + (item1 * copies);
    }

    println!("similarity score is {similarity_score}");

}
