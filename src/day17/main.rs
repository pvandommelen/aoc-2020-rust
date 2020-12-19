use std::{collections::{HashMap, HashSet}, fs};
use aoc_2020_rust::util::bench;

#[cfg(test)]
use std::iter::FromIterator;

type Position = Vec<i8>;

#[derive(Debug, PartialEq, Clone)]
struct Space {
    dimensions: u32,
    active_set: HashSet<Position>,
}
impl Space {
    fn new(dimensions: u32) -> Space {
        Space {
            dimensions,
            active_set: HashSet::new(),
        }
    }

    fn new_increase_dimensions(other: &Space, dimensions: u32) -> Space {
        Space {
            dimensions: other.dimensions + dimensions,
            active_set: other.active_set.iter().map(|pos2| {
                let mut copy = pos2.clone();
                for _extra_dimension in 0..dimensions {
                    copy.push(0);
                }
                copy
            }).collect(),
        }
    }

    fn add(&mut self, pos: Position) {
        self.active_set.insert(pos);
    }

    fn has(&self, pos: &Position) -> bool {
        self.active_set.contains(pos)
    }

    fn len(&self) -> usize {
        self.active_set.len()
    }

    fn iter(&'_ self) -> impl Iterator<Item = &'_ Position> {
        self.active_set.iter()
    }
}

fn prepare_input(input: &str) -> Space {
    let mut space = Space::new(2);
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, char)| {
            if char == '#' {
                space.add(vec![x as i8 - 1, y as i8 - 1]);
            }
        });
    });

    space
}

fn call_neighbours<F>(pos: &Position, func: &mut F)
where F: FnMut(&Position)
{
    fn call_neighbours_internal<F>(pos: &Position, func: &mut F, iterating_dimension: usize, neighbour_position: &mut Position)
    where F: FnMut(&Position)
    {
        for i in -1..2 {
            neighbour_position[iterating_dimension] = pos[iterating_dimension] + i;
            if iterating_dimension == pos.len() - 1 {
                func(&neighbour_position);
            } else {
                call_neighbours_internal(pos, func, iterating_dimension + 1, neighbour_position);
            }
        }
    }

    let mut neighbour_position = vec![0; pos.len()];
    call_neighbours_internal(pos, &mut |neighbour_position| {
        if pos == neighbour_position {
            return;
        }
        func(neighbour_position);
    }, 0, &mut neighbour_position);
}

fn evolve(input: &Space) -> Space {
    let mut copy = Space::new(input.dimensions);
    let mut neighbours = HashMap::new();

    input.iter().for_each(|position| {
        let mut neighbour_count = 0;
        call_neighbours(&position, &mut |neighbour_position| {
            if input.has(&neighbour_position) {
                neighbour_count += 1;
            }

            if !input.has(neighbour_position) {
                neighbours.insert(neighbour_position.to_owned(), neighbours.get(neighbour_position).unwrap_or(&0) + 1);
            }
        });

        if neighbour_count == 2 || neighbour_count == 3 {
            copy.add(position.to_owned());
        }
    });

    neighbours.iter().for_each(|(position, count)| {
        if *count == 3 {
            copy.add(position.to_owned());
        }
    });

    copy
}

fn part1(input: &Space) -> usize {
    let mut current = Space::new_increase_dimensions(input, 1);
    for _cycle in 0..6 {
        current = evolve(&current);
    }

    current.len()
}
fn part2(input: &Space) -> usize {
    let mut current = Space::new_increase_dimensions(input, 2);
    for _cycle in 0..6 {
        current = evolve(&current);
    }

    current.len()
}

fn main() {
    let input = fs::read_to_string("./src/day17/input.txt").expect("Unable to read input file");

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
".#.
..#
###";

    #[test]
    fn parse_example() {
        assert_eq!(prepare_input(EXAMPLE_INPUT), Space {
            dimensions: 2,
            active_set: HashSet::from_iter(vec![
                vec![0, -1],
                vec![1, 0],
                vec![-1, 1],
                vec![0, 1],
                vec![1, 1],
            ].iter().cloned()),
        });
    }
    
    #[test]
    fn part1_example() {
        assert_eq!(part1(&prepare_input(EXAMPLE_INPUT)), 112);
    }
    
    #[test]
    fn part2_example() {
        assert_eq!(part2(&prepare_input(EXAMPLE_INPUT)), 848);
    }
}