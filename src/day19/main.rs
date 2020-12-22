use std::{collections::HashMap, fs};
use aoc_2020_rust::util::bench;
use nom::{IResult, branch::alt, bytes::complete::tag, character::complete::{alpha1, char, digit1}, combinator::{all_consuming, map}, multi::separated_list1, sequence::delimited};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct State<'a> {
    remaining: &'a str,
}

type Rules = HashMap<u64, Box<dyn Parser>>;

trait Parser: ParserClone {
    fn consume<'a>(&self, rules: &Rules, state: &State<'a>) -> Result<State<'a>, ()>;
}
trait ParserClone {
    fn clone_box(&self) -> Box<dyn Parser>;
}

impl<T: 'static + Parser + Clone> ParserClone for T {
    fn clone_box(&self) -> Box<dyn Parser> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Parser> {
    fn clone(&self) -> Box<dyn Parser> {
        self.clone_box()
    }
}

#[derive(Clone)]
struct And {
    definitions: Vec<Box<dyn Parser>>,
}
impl Parser for And {
    fn consume<'a>(&self, rules: &Rules, state: &State<'a>) -> Result<State<'a>, ()> {
        let mut iterator = self.definitions.iter();
        let mut current_state: State<'a> = state.to_owned();
        while let Some(definition) = iterator.next() {
            let inner_result: Result<State<'a>, ()> = definition.consume(rules, &current_state);
            if inner_result.is_err() {
                return inner_result;
            }
            current_state = inner_result.unwrap();
        }
        Ok(current_state)
    }
}

#[derive(Clone)]
struct Or {
    definitions: Vec<Box<dyn Parser>>,
}
impl Parser for Or {
    fn consume<'a>(&self, rules: &Rules, state: &State<'a>) -> Result<State<'a>, ()> {
        let mut iterator = self.definitions.iter();
        while let Some(definition) = iterator.next() {
            let inner_result = definition.consume(rules, &state);
            if inner_result.is_ok() {
                return inner_result;
            }
        }
        Err(())
    }
}

#[derive(Clone)]
struct Reference {
    index: u64,
}
impl Parser for Reference {
    fn consume<'a>(&self, rules: &Rules, state: &State<'a>) -> Result<State<'a>, ()> {
        rules[&self.index].consume(rules, state)
    }

}

impl Parser for String {
    fn consume<'a>(&self, _rules: &Rules, state: &State<'a>) -> Result<State<'a>, ()> {
        if state.remaining.starts_with(self) {
            let (_, remainder) = state.remaining.split_at(self.len());
            Ok(State {
                remaining: remainder,
            })
        } else {
            Err(())
        }
    }
}

type Input<'a> = (Rules, Vec<&'a str>);

fn parse_rule(i: &str) -> Box<dyn Parser> {
    fn inner_parse_rule(i: &str) -> IResult<&str, Box<dyn Parser>> {
        map(separated_list1(
            tag(" | "),
            map(separated_list1(
                char(' '),
                alt((
                    map(
                        delimited(char('"'), alpha1, char('"')),
                        |char: &str| -> Box<dyn Parser> {
                            Box::new(char.to_owned())
                        }
                    ),
                    map(digit1, |a: &str| -> Box<dyn Parser> {
                        Box::new(Reference {
                            index: a.parse().unwrap(),
                        })
                    }),
                ))
            ), |parts| -> Box<dyn Parser> {
                Box::new(And {
                    definitions: parts,
                })
            })
        ), |parts| -> Box<dyn Parser> {
            Box::new(Or {
                definitions: parts,
            })
        })(i)
    }
    all_consuming(inner_parse_rule)(i).unwrap().1
}

fn prepare_input<'a>(input: &'a str) -> Input<'a> {
    let sections: Vec<&str> = input.split("\n\n").collect();
    assert!(sections.len() == 2);

    let rules = sections[0].lines().fold(HashMap::new(), |mut rules, line| {
        let mut line_split = line.split(": ");
        let index = line_split.next().unwrap().parse::<u64>().unwrap();
        let unparsed_rule = line_split.next().unwrap();
        let rule = parse_rule(unparsed_rule);
        
        rules.insert(index, rule);
        rules
    });

    let messages = sections[1].lines().collect();
    
    (
        rules,
        messages,
    )
}

fn part1(input: &Input) -> u64 {
    let rules = &input.0;
    let messages = &input.1;

    messages.iter().filter(|message| {
        let result = rules.get(&0).unwrap().consume(rules, &State {
            remaining: message,
        });

        match result {
            Ok(state) => state.remaining.len() == 0,
            _ => false,
        }
    }).count() as u64
}

fn part2(input: &Input) -> u64 {
    let rules = &input.0;
    let messages = &input.1;

    let mut updated_rules = (*rules).clone();
    updated_rules.insert(8, Box::new(And {
        definitions: vec![
            Box::new(Reference { index: 42 }),
            Box::new(Or {
                definitions: vec![
                    Box::new(Reference { index: 8 }),
                    Box::new("".to_owned()),
                ],
            }),
        ]
    }));
    updated_rules.insert(11, Box::new(And {
        definitions: vec![
            Box::new(Reference { index: 42 }),
            Box::new(Or {
                definitions: vec![
                    Box::new(And {
                        definitions: vec![
                            Box::new(Reference { index: 11 }),
                            Box::new(Reference { index: 31 }),
                        ],
                    }),
                    Box::new(Reference { index: 31 }),
                ],
            }),
        ]
    }));
    // updated_rules.insert(8, Box::new(Or {
    //     definitions: vec![
    //         Box::new(Reference { index: 42 }),
    //         Box::new(And {
    //             definitions: vec![
    //                 Box::new(Reference { index: 42 }),
    //                 Box::new(Reference { index: 8 }),
    //             ],
    //         }),
    //     ]
    // }));
    // updated_rules.insert(11, Box::new(Or {
    //     definitions: vec![
    //         Box::new(And {
    //             definitions: vec![
    //                 Box::new(Reference { index: 42 }),
    //                 Box::new(Reference { index: 31 }),
    //             ],
    //         }),
    //         Box::new(And {
    //             definitions: vec![
    //                 Box::new(Reference { index: 42 }),
    //                 Box::new(Reference { index: 11 }),
    //                 Box::new(Reference { index: 31 }),
    //             ],
    //         }),
    //     ]
    // }));

    messages.iter().filter(|message| {
        let result = rules.get(&0).unwrap().consume(&updated_rules, &State {
            remaining: message,
        });


        match result {
            Ok(state) => {
                println!("{}", state.remaining.len());
                state.remaining.len() == 0
            },
            _ => false,
        }
    }).count() as u64
}

fn main() {
    let input = fs::read_to_string("./src/day19/input.txt").expect("Unable to read input file");

    let prepared_input = bench::time("input preparation", || prepare_input(&input));
    let part1 = bench::time("part 1", || part1(&prepared_input));
    let part2 = bench::time("part 2", || part2(&prepared_input));

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parsing() {
        let rules = HashMap::new();
        let state = State { remaining: "ab" };
        assert_eq!("a".to_owned().consume(&rules, &state), Ok(State{remaining: "b"}));
        assert_eq!("".to_owned().consume(&rules, &state), Ok(State{remaining: "ab"}));

        assert_eq!(Or{definitions:vec![Box::new("a".to_owned())]}.consume(&rules, &state), Ok(State{remaining: "b"}));
        assert_eq!(Or{definitions:vec![Box::new("b".to_owned()), Box::new("a".to_owned())]}.consume(&rules, &state), Ok(State{remaining: "b"}));

        assert_eq!(And{definitions:vec![Box::new("a".to_owned())]}.consume(&rules, &state), Ok(State{remaining: "b"}));
        assert_eq!(And{definitions:vec![Box::new("a".to_owned()), Box::new("b".to_owned())]}.consume(&rules, &state), Ok(State{remaining: ""}));
    }
    #[test]
    fn part1_example1() {
        const EXAMPLE_INPUT: &str = 
"0: 1 2
1: \"a\"
2: 1 3 | 3 1
3: \"b\"

aab
aba";

        assert_eq!(part1(&prepare_input(EXAMPLE_INPUT)), 2);
    }
    #[test]
    fn part1_example2() {
        const EXAMPLE_INPUT: &str = 
"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb";
    
        assert_eq!(part1(&prepare_input(EXAMPLE_INPUT)), 2);
    }

    #[test]
    fn part2_example() {
        const EXAMPLE_INPUT: &str = 
"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";
    
        assert_eq!(part1(&prepare_input(EXAMPLE_INPUT)), 3);
        assert_eq!(part2(&prepare_input(EXAMPLE_INPUT)), 12);
    }
}