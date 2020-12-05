use std::fs;
use std::time;

mod day03;

fn main() {
    let input = fs::read("./src/day03/input.txt").expect("Unable to read input file");

    let start = time::Instant::now();

    let map = day03::Map{data: input, width: 31};
    
    let slopes = [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ];
    let part2_answer: u32 = slopes
        .iter()
        .map(|&slope| day03::count_encountered_trees(&map, slope))
        .product();
    
    let end = time::Instant::now();

    println!("Multiplied: {}", part2_answer);

    println!("Time: {:?}", end - start);
}
