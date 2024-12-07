
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


    let mut word_map: Vec<Vec<char>> = vec![];
    for line in contents.lines()
    {
        word_map.push(line.chars().collect());
    }

    let search_string = String::from("XMAS");

    let mut total = 0;
    for y in 0..word_map.len() {
        for x in 0..word_map[0].len() {
            if search_direction(&word_map, &search_string, y, x, &Unit_Velocity::UP) {
                total = total + 1;
            }
            if search_direction(&word_map, &search_string, y, x, &Unit_Velocity::UP_RIGHT) {
                total = total + 1;
            }
            if search_direction(&word_map, &search_string, y, x, &Unit_Velocity::RIGHT) {
                total = total + 1;
            }
            if search_direction(&word_map, &search_string, y, x, &Unit_Velocity::DOWN_RIGHT) {
                total = total + 1;
            }
            if search_direction(&word_map, &search_string, y, x, &Unit_Velocity::DOWN) {
                total = total + 1;
            }
            if search_direction(&word_map, &search_string, y, x, &Unit_Velocity::DOWN_LEFT) {
                total = total + 1;
            }
            if search_direction(&word_map, &search_string, y, x, &Unit_Velocity::LEFT) {
                total = total + 1;
            }
            if search_direction(&word_map, &search_string, y, x, &Unit_Velocity::UP_LEFT) {
                total = total + 1;
            }
        }
    }

    println!("XMAS COUNT = {total}");
    /*****************************PART 1 COMPLETE******************************************/
    total = 0;
    for y in 0..word_map.len() {
        for x in 0..word_map[0].len() {
            total = total + check_mas_x(&word_map, y, x)
        }
    }

    println!("X-MAS COUNT = {total}");
}

fn search_direction(word_map :&Vec<Vec<char>>, cypher :&String,  y :usize, x :usize, direction :&Velocity) -> bool{
    let mut root = Position::new(y, x, Some(word_map.len()), Some(word_map[0].len()));
    let mut word = String::new();
    for i in 0..cypher.len() {
        word.push(word_map[root.y][root.x]);
        root.apply_velocity(direction);
    }
    if &word == cypher {
        return true;
    }
    return false;
}

fn check_mas_x(word_map :&Vec<Vec<char>>, y:usize, x:usize) -> i32 {
    let AS = String::from("AS");
    let AM = String::from("AM");
    let mut count = 0;
    
    if (search_direction(word_map, &AS, y, x, &Unit_Velocity::UP_RIGHT) &&
        search_direction(word_map, &AS, y, x, &Unit_Velocity::DOWN_RIGHT) &&
        search_direction(word_map, &AM, y, x, &Unit_Velocity::DOWN_LEFT) &&
        search_direction(word_map, &AM, y, x, &Unit_Velocity::UP_LEFT)) {
            count = count+1;
    }
    if (search_direction(word_map, &AS, y, x, &Unit_Velocity::DOWN_RIGHT) &&
        search_direction(word_map, &AS, y, x, &Unit_Velocity::DOWN_LEFT) &&
        search_direction(word_map, &AM, y, x, &Unit_Velocity::UP_LEFT) &&
        search_direction(word_map, &AM, y, x, &Unit_Velocity::UP_RIGHT)) {
            count = count+1;
    }
    if (search_direction(word_map, &AS, y, x, &Unit_Velocity::DOWN_LEFT) &&
        search_direction(word_map, &AS, y, x, &Unit_Velocity::UP_LEFT) &&
        search_direction(word_map, &AM, y, x, &Unit_Velocity::UP_RIGHT) &&
        search_direction(word_map, &AM, y, x, &Unit_Velocity::DOWN_RIGHT)) {
            count = count+1;
    }
    if (search_direction(word_map, &AS, y, x, &Unit_Velocity::UP_LEFT) &&
        search_direction(word_map, &AS, y, x, &Unit_Velocity::UP_RIGHT) &&
        search_direction(word_map, &AM, y, x, &Unit_Velocity::DOWN_RIGHT) &&
        search_direction(word_map, &AM, y, x, &Unit_Velocity::DOWN_LEFT)) {
            count = count+1;
    }
    return count;
}

