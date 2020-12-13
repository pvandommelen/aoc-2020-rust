use std::fs;
use aoc_2020_rust::util::bench;

#[derive(Debug, Clone)]
pub struct Map<T: Copy> {
    pub data: Vec<T>,
    pub width: usize,
    pub height: usize,
    stride_y: usize,
}

type Window<T> = Vec<T>;

trait QueryableMap<T> {
    fn at(&self, x: usize, y: usize) -> T;
    fn set(&mut self, x: usize, y: usize, value: &T);

    fn iter(&'_ self) -> Box<dyn Iterator<Item = (usize, usize, T)> + '_>;

    fn iter_window(&'_ self, distance: usize, default_value: T) -> Box<dyn Iterator<Item = (usize, usize, Window<T>)> + '_>;
}
impl<T: Copy> QueryableMap<T> for Map<T> {
    fn at(&self, x: usize, y: usize) -> T {
        let index = y * self.stride_y + x;
        self.data[index]
    }
    fn set(&mut self, x: usize, y: usize, value: &T) {
        let index = y * self.stride_y + x;
        self.data[index] = *value;
    }

    fn iter(&'_ self) -> Box<dyn Iterator<Item = (usize, usize, T)> + '_> {
        Box::new(
            (0..self.height).flat_map(move |y| {
                (0..self.width).map(move |x| {
                    (x, y)
                })
            }).map(move |(x, y)| {
                (x, y, self.at(x, y))
            })
        )
    }

    fn iter_window(&'_ self, distance: usize, default_value: T) -> Box<dyn Iterator<Item = (usize, usize, Window<T>)> + '_> {
        let window_side_length = distance * 2 + 1;
        let window_size = window_side_length * window_side_length;

        Box::new(
            (0..self.height).flat_map(move |y| {
                (0..self.width).map(move |x| {
                    (x, y)
                })
            }).map(move |(x, y)| {
                let mut window = vec![default_value; window_size];
                for j in 0..window_side_length {
                    for i in 0..window_side_length {
                        window[j * window_side_length + i] = if i + x < distance || j + y < distance || i + x - distance >= self.width || j + y - distance >= self.height {
                            default_value
                        } else {
                            self.at(i + x - distance, j + y - distance)
                        };
                    }
                }
                (x, y, window)
            })
        )
    }
}

fn prepare_input(input: &str) -> Map<u8> {
    let bytes = input.as_bytes().to_owned();
    let width: usize = bytes.iter().position(|&c| c == b'\n').unwrap_or(bytes.len());
    let stride_y = width + 1;
    Map { 
        data: bytes,
        width,
        height: (input.len() + stride_y - 1) / stride_y,
        stride_y,
    }
}

fn evolve(map: &Map<u8>, state: &Map<bool>) -> (Map<bool>, bool) {
    let mut new_state = state.to_owned();
    let mut changed = false;
    state.iter_window(1, false).for_each(
        |(x, y, old_state_window)| {

            if map.at(x, y) == b'L' {
                let old_seat_state = state.at(x, y);
                let filled_seats_in_window_count = old_state_window.iter().filter(|&val| *val).count();

                let filled_seats_adjacent = if old_seat_state { filled_seats_in_window_count - 1 } else { filled_seats_in_window_count };

                let new_seat_state = match old_seat_state {
                    false => filled_seats_adjacent == 0,
                    true => filled_seats_adjacent < 4,
                };
                if new_seat_state != old_seat_state {
                    new_state.set(x, y, &new_seat_state);
                    changed = true;
                }
            }
        }
    );
    (new_state, changed)
}

fn part1(input: &Map<u8>) -> usize {
    let data = vec![false; input.width * input.height];
    let mut state = Map {
        data,
        width: input.width,
        height: input.height,
        stride_y: input.width,
    };

    loop {
        let (evolved_state, changed) = evolve(input, &state);
        if !changed {
            break;
        }
        state = evolved_state;
    }

    state.iter().filter(|(_, _, value)| *value).count()
}
fn part2(input: &Map<u8>) -> u32 {
    0
}

fn main() {
    let input = fs::read_to_string("./src/day11/input.txt").expect("Unable to read input file");

    let prepared_input = bench::time("input preparation", || prepare_input(&input));
    let part1 = bench::time("part 1", || part1(&prepared_input));
    let part2 = bench::time("part 2", || part2(&prepared_input));

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = 
"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
    
    #[test]
    fn parse_example_map() {
        let map = prepare_input(EXAMPLE_INPUT);
        assert_eq!(map.width, 10);
        assert_eq!(map.height, 10);
        assert_eq!(map.at(0, 0), b'L');
        assert_eq!(map.at(9, 0), b'L');
        assert_eq!(map.at(0, 1), b'L');
        assert_eq!(map.at(9, 9), b'L');
    }
    
    #[test]
    fn map_iter_window() {
        let map = prepare_input(EXAMPLE_INPUT);
        assert_eq!(map.iter_window(0, 0).count(), 10*10);
        assert_eq!(map.iter_window(1, 0).count(), 10*10);
    }
    
    #[test]
    fn part1_example() {
        assert_eq!(part1(&prepare_input(EXAMPLE_INPUT)), 37);
    }
    
    #[test]
    fn part2_example() {
        assert_eq!(part2(&prepare_input(EXAMPLE_INPUT)), 0);
    }
}