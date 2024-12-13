mod input_parsing;

mod grid_traversal;
use grid_traversal::{
    Position,
    Velocity,
    Unit_Velocity
};


fn main() {
    let contents = input_parsing::get_raw_input_arg1(); 

    println!("{contents}\r\n");

    let text_map :Vec<Vec<char>> = vec![];
    let text_map = (contents, text_map);

    println!("{:?}", text_map)

}