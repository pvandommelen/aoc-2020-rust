use std::fs;
use std::time;

mod day03;

fn main() {
    let input = fs::read("./src/day03/input.txt").expect("Unable to read input file");

    let start = time::Instant::now();

    let map = day03::Map{data: input, width: 31};

    let slope_3_1_tree_count = day03::count_encountered_trees(&map, (3, 1));
    
    let end = time::Instant::now();

    println!("(3, 1): {}", slope_3_1_tree_count);

    println!("Time: {:?}", end - start);
}