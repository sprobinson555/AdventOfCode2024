mod input_parsing;

mod grid_traversal;
use grid_traversal::{
    Position,
    Velocity,
    Unit_Velocity
};
use input_parsing::string_to_2d_vec_chars;

struct Guard {
    pos :Position,
    dir :Velocity,
}

fn main() {
    let contents = input_parsing::get_raw_input_arg1(); 

    let x_char = 'X';

    let text_map :Vec<Vec<char>> = vec![];
    let mut text_map = string_to_2d_vec_chars(contents, text_map);

    let start_index = find_first_item('^', &text_map).expect("ERROR: NO GUARD FOUND");

    let mut guard = Guard {
        pos : Position::new(
            start_index.0, 
            start_index.1, 
            Some(text_map.len()), 
            Some(text_map[0].len())),
        dir : Unit_Velocity::UP,
    };

    check_guard_front(&mut guard, &text_map);
    text_map[guard.pos.y][guard.pos.x] = 'X';
    // while guard.pos.apply_velocity(&guard.dir) {
    loop {
        if !(guard.pos.apply_velocity(&guard.dir)) {
            break;
        } 
        check_guard_front(&mut guard, &text_map);
        text_map[guard.pos.y][guard.pos.x] = x_char;
    }

    let mut total_traversed = 0;
    for line in text_map.iter() {
        // println!("{:?}", line);
        for item in line.iter() {
            if item == &x_char {
                total_traversed = total_traversed + 1;
            }
        }
    }
    
    println!("total traversed = {total_traversed}");

}

fn check_guard_front(guard :&mut Guard, map :&Vec<Vec<char>>)
{
    let mut vel = Velocity {
        y : guard.dir.y,
        x : guard.dir.x,
    };
    let obstacle = String::from("#");
    while search_direction(&map, &obstacle, guard.pos.y, guard.pos.x, &vel) {

        match vel {
            Unit_Velocity::UP => vel = Unit_Velocity::RIGHT,
            Unit_Velocity::RIGHT => vel = Unit_Velocity::DOWN,
            Unit_Velocity::DOWN => vel = Unit_Velocity::LEFT,
            Unit_Velocity::LEFT => vel = Unit_Velocity::UP,
            _ => vel = Unit_Velocity::DOWN,
        }
    }
    guard.dir = vel;
}

fn find_first_item<T:std::cmp::PartialEq>(item_to_find :T, item_map :&Vec<Vec<T>>) -> Option<(usize, usize)> {
    for (row_index, row) in item_map.iter().enumerate() {
        for (item_index, item) in row.iter().enumerate() {
            if item == &item_to_find {
                return Some((row_index, item_index))
            }
        }
    }
    None
}

fn search_direction(word_map :&Vec<Vec<char>>, cypher :&String,  y :usize, x :usize, direction :&Velocity) -> bool{
    let mut root = Position::new(y, x, Some(word_map.len()), Some(word_map[0].len()));
    let mut word = String::new();
    for i in 0..cypher.len() {
        root.apply_velocity(direction);
        word.push(word_map[root.y][root.x]);
    }
    if &word == cypher {
        return true;
    }
    return false;
}