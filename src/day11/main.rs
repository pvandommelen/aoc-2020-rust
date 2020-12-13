use std::fs;
use aoc_2020_rust::util::bench;

#[derive(Debug, Clone)]
pub struct Map<T: Copy> {
    pub data: Vec<T>,
    pub width: usize,
    pub height: usize,
    stride_y: usize,
}

trait QueryableMap<T> {
    fn at(&self, x: i32, y: i32) -> Option<T>;
    fn at_unchecked(&self, x: usize, y: usize) -> T;

    fn set(&mut self, x: usize, y: usize, value: &T);

    fn iter(&'_ self) -> Box<dyn Iterator<Item = (usize, usize, T)> + '_>;
}
impl<T: Copy> QueryableMap<T> for Map<T> {
    fn at(&self, x: i32, y: i32) -> Option<T> {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            None
        } else {
            Some(self.at_unchecked(x as usize, y as usize))
        }
    }
    fn at_unchecked(&self, x: usize, y: usize) -> T {
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
                (x, y, self.at_unchecked(x, y))
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

fn evolve1(map: &Map<u8>, state: &Map<bool>) -> (Map<bool>, bool) {
    let mut new_state = state.to_owned();
    let mut changed = false;
    map.iter().for_each(
        |(x, y, elem)| {
            if elem != b'L' {
                return;
            }

            let mut seats_filled: u8 = 0;
            for j in (-1 as i32)..2 {
                for i in (-1 as i32)..2 {
                    let position_x = x as i32 + i;
                    let position_y = y as i32 + j;
                    if let Some(square) = map.at(position_x, position_y) {
                        if square == b'L' {
                            seats_filled += if state.at(position_x, position_y).unwrap() { 1 } else { 0 };
                        }
                    }
                }
            }

            let old_seat_state = state.at_unchecked(x, y);
            let new_seat_state = match old_seat_state {
                false => seats_filled == 0,
                true => seats_filled <= 4,
            };
            if new_seat_state != old_seat_state {
                new_state.set(x, y, &new_seat_state);
                changed = true;
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
        let (evolved_state, changed) = evolve1(input, &state);
        if !changed {
            break;
        }
        state = evolved_state;
    }

    state.iter().filter(|(_, _, value)| *value).count()
}

fn evolve2(map: &Map<u8>, state: &Map<bool>) -> (Map<bool>, bool) {
    let mut new_state = state.to_owned();
    let mut changed = false;
    map.iter().for_each(
        |(x, y, elem)| {
            if elem != b'L' {
                return;
            }

            let mut seats_filled: u8 = 0;
            for j in (-1 as i32)..2 {
                for i in (-1 as i32)..2 {
                    let mut position_x = x as i32 + i;
                    let mut position_y = y as i32 + j;
                    while let Some(square) = map.at(position_x, position_y) {
                        if square == b'L' {
                            seats_filled += if state.at(position_x, position_y).unwrap() { 1 } else { 0 };
                            break;
                        }
                        position_x += i;
                        position_y += j;
                    }
                }
            }

            let old_seat_state = state.at_unchecked(x, y);
            let new_seat_state = match old_seat_state {
                false => seats_filled == 0,
                true => seats_filled <= 5,
            };
            if new_seat_state != old_seat_state {
                new_state.set(x, y, &new_seat_state);
                changed = true;
            }
        }
    );
    (new_state, changed)
}

fn part2(input: &Map<u8>) -> usize {
    let data = vec![false; input.width * input.height];
    let mut state = Map {
        data,
        width: input.width,
        height: input.height,
        stride_y: input.width,
    };

    loop {
        let (evolved_state, changed) = evolve2(input, &state);
        if !changed {
            break;
        }
        state = evolved_state;
    }

    state.iter().filter(|(_, _, value)| *value).count()
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
        assert_eq!(map.at(0, 0), Some(b'L'));
        assert_eq!(map.at(9, 0), Some(b'L'));
        assert_eq!(map.at(0, 1), Some(b'L'));
        assert_eq!(map.at(9, 9), Some(b'L'));
    }
    
    #[test]
    fn map_iter() {
        let map = prepare_input(EXAMPLE_INPUT);
        assert_eq!(map.iter().count(), 10*10);
    }
    
    #[test]
    fn part1_example() {
        assert_eq!(part1(&prepare_input(EXAMPLE_INPUT)), 37);
    }
    
    #[test]
    fn part2_example() {
        assert_eq!(part2(&prepare_input(EXAMPLE_INPUT)), 26);
    }
}