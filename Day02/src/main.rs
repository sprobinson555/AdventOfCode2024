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
        let mut safe = false;
        for problem_child in 0..levels.len() {
            let mut sub_levels : Vec<&i32> = vec![];
            for i in 0..levels.len() {
                if i != problem_child {
                    sub_levels.push(&levels[i]);
                }
            }
            if check_report_safety(&sub_levels) {
                safe = true;
                break;
            }
        }
        
        if safe {
            safety_total = safety_total + 1;
        }
    }
    println!("total number of safe items is {safety_total}");
}

fn check_report_safety(report: &Vec<&i32>) -> bool {
    if report.is_sorted_by(|a, b| a < b) || report.is_sorted_by(|a, b| a > b) {
        for level_num in 1..report.len() {
            if (report[level_num] - report[level_num - 1]).abs() > 3 {
                return false;
            }
        }
    }
    else {
        return false;
    }
    true
}