use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("You need to specify a filename input argument")
    }

    let filepath = &args[1];

    let contents = fs::read_to_string(filepath).expect("Cannot read contents of file");

    println!("file contents:");
    println!("{contents}");

    let mut reports: Vec<&str> = vec![];

    for line in contents.lines() {
        reports.push(line);
    }

    let mut safety_total: u32 = 0;
    for report in reports {
        let mut levels: Vec<i32> = vec![];
        for level in report.split_whitespace() {
            levels.push(level.parse().unwrap());
        }
        
        let mut safe :bool = true;
        if levels.is_sorted_by(|a, b| a < b) || levels.is_sorted_by(|a, b| a > b) {
            for level_num in 1..levels.len() {
                if (levels[level_num] - levels[level_num - 1]).abs() > 3 {
                    safe = false;
                    break;

                }
            }
        }
        else {
            safe = false;
        }
        if safe {
            safety_total = safety_total + 1;
        }
    }
    println!("total number of safe reports is {safety_total}");
}
