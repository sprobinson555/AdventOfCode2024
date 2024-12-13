use std::env;
use std::fs;

mod grid_traversal;
use grid_traversal::{
    Position,
    Velocity,
    Unit_Velocity
};


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("You need to specify a filename input argument")
    }

    let filepath = &args[1];

    let contents = fs::read_to_string(filepath).expect("Cannot read contents of file");

    let mut rules :Vec<(i32, i32)> = vec![];
    let mut pages :Vec<Vec<i32>> = vec![];

    // parse the inputs
    for line in contents.lines() {
        if line.contains("|") {
            let mut nums = line.split('|');
            let (num1, num2) = (nums.next().unwrap(), nums.next().unwrap());
            rules.push((num1.parse::<i32>().unwrap(), num2.parse::<i32>().unwrap()));
        }
        if line.contains(",") {
            let mut page : Vec<i32> = vec![];
            for nums in line.split(',') {
                page.push(nums.parse::<i32>().unwrap());
            }
            pages.push(page);
        }
    }
    
    // println!("rules:\r\n {:?}", rules); 
    // println!("pages:\r\n {:?}", pages);

    let mut sum_of_valids = 0;
    let mut sum_of_invalids = 0;

    for page in pages.iter() {
        let mut valid = true;
        let mut new_page :Vec<i32> = vec![];
        for item in page {
            new_page.push(*item);
        }
        // for (index, num) in new_page.iter_mut().enumerate().rev()
        for index in (0..new_page.len()).rev() {
            let mut violator_index = check_previous_entries(index, &new_page, &rules);
            while violator_index != new_page.len() {
                valid = false;
                let num = new_page[index];
                new_page.remove(index);
                new_page.insert(violator_index, num);
                violator_index = check_previous_entries(index, &new_page, &rules)
            }
        }
        println!("{:?} validity is {valid}", page);
        println!("new page is {:?}", new_page);
        let middle = &new_page[new_page.len()/2];
        println!("the middle number is {middle}");
        if valid {
            sum_of_valids = sum_of_valids + middle;
        }
        else {
            sum_of_invalids = sum_of_invalids + middle;
        }
    }
    println!("the sum of the valid pages is {sum_of_valids}");
    println!("the sum of the invalid pages is {sum_of_invalids}");
}




fn check_previous_entries(index :usize, page: &Vec<i32>, rules :&Vec<(i32, i32)>) -> usize {
    for (slice_index, item) in page[0..index].iter().enumerate() {
        for rule in rules.iter().filter(|(a, b)| a == &page[index]) {
            if item == &rule.1 {
                return slice_index;
            }   
        }
    }
    return page.len();
}