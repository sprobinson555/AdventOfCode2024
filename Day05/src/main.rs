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

    let mut sum_of_middles = 0;
    let pages_copy :Vec<Vec<i32>> = vec![];

    for page in pages.iter() {
        let mut valid = true;
        for (index, num) in page.iter().enumerate() {
            for rule in rules.iter().filter(|(a, b)| a == num) {
                if page[0..index].contains(&rule.1) {
                    valid = false;
                }
            }
        }
        println!("{:?} validity is {valid}", page);
        if valid {
            let middle = &page[page.len()/2];
            println!("the middle number is {middle}");
            sum_of_middles = sum_of_middles + middle;
        }
    }

    println!("sum of middles is {sum_of_middles}\r\n");

    /************************************ PART TWO  **********************************/

    let mut sum_of_middles = 0;
    for page in pages.iter() {
        let mut valid = true;
        let mut new_page :Vec<i32> = vec![];
        for (index, num) in page.iter().enumerate() {
            let index_to_insert = page_item_breaks_rules(index, num, &new_page, &rules);
            if  index_to_insert != new_page.len() {
                valid = false;
                new_page.insert(index_to_insert, *num);
            }
            else {
                new_page.push(*num);
            }
        }
        println!("{:?} validity is {valid}", page);
        println!("new page is {:?}", new_page);
        if valid {
            let middle = &page[page.len()/2];
            println!("the middle number is {middle}");
            sum_of_middles = sum_of_middles + middle;
        }
    }


}

fn page_item_breaks_rules(index :usize, num: &i32, page: &Vec<i32>, rules :&Vec<(i32, i32)>) -> usize {
    for rule in rules.iter().filter(|(a, b)| a == num) {
        for (slice_index, item) in page[0..index].iter().enumerate() {
            if item == &rule.1 {
                return slice_index;
            }   
        }
    }
    return page.len();
}