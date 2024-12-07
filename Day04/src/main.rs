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

    println!("{total}");
}

fn search_direction(word_map :&Vec<Vec<char>>, cypher :&String,  y :usize, x :usize, direction :&Velocity) -> bool{
    let mut root = Position::new(y, x, Some(word_map.len()), Some(word_map[0].len()));
    let mut word = String::new();
    for i in 0..4 {
        word.push(word_map[root.y][root.x]);
        root.apply_velocity(direction);
    }
    if &word == cypher {
        return true;
    }
    return false;
}

