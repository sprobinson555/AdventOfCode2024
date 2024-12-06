use std::env;
use std::fs;

const X :u8 = "X".as_bytes()[0];
const M :u8 = "M".as_bytes()[0];
const A :u8 = "A".as_bytes()[0];
const S :u8 = "S".as_bytes()[0];
const xmas_arr:[&u8; 4] = [&X, &M, &A, &S];
    

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("You need to specify a filename input argument")
    }

    let filepath = &args[1];

    let contents = fs::read_to_string(filepath).expect("Cannot read contents of file");


    let mut word_map = vec![];
    for line in contents.lines()
    {
        word_map.push(line.as_bytes());
    }

    let mut total = 0;
    for y in 0..word_map.len() {
        for x in 0..word_map[0].len() {
            if check_right(&word_map, y, x) {
                total = total + 1;
            }
        }
    }

    println!("{total}");
}

fn check_right(word_map :&Vec<&[u8]>, y :usize, x :usize) -> bool{
    if x > (word_map[y].len() - 4) {
        return false;
    }
    let wut :[&u8; 4] = [&word_map[y][x], &word_map[y][x+1], &word_map[y][x+2], &word_map[y][x+3]];
    if wut == xmas_arr {
        return true;
    }
    return false;
}

fn check_left(word_map :&Vec<&[u8]>, y :usize, x :usize) -> bool{
    if x < 3 {
        return false;
    }
    let wut :[&u8; 4] = [&word_map[y][x], &word_map[y][x-1], &word_map[y][x-2], &word_map[y][x-3]];
    if wut == xmas_arr {
        return true;
    }
    return false;
}
