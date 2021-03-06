use std::{collections::HashMap, fs, iter::{empty, once}};
use aoc_2020_rust::util::bench;
use nom::{IResult, branch::alt, bytes::complete::tag, character::complete::{alpha1, char, digit1}, combinator::{all_consuming, map}, multi::separated_list1, sequence::delimited};

type BoxedIterator<'a, 'b> = Box<dyn Iterator<Item = &'a str> + 'b>;

type Rules = HashMap<u64, Parser>;

#[derive(Clone)]
enum Parser {
    And {
        definitions: Vec<Parser>,
    },
    Or {
        definitions: Vec<Parser>,
    },
    Reference {
        index: u64,
    },
    String {
        string: String,
    },
}
impl Parser {
    fn consume<'a: 'b, 'b>(&'b self, rules: &'b Rules, remaining: &'a str) -> BoxedIterator<'a, 'b> {
        match self {
            Self::And {definitions} => {
                fn recurse<'a: 'b, 'b>(rules: &'b Rules, definitions: &'b [Parser], remaining: &'a str) -> BoxedIterator<'a, 'b> {
                    if definitions.len() == 0 {
                        return Box::new(once(remaining));
                    }
        
                    let definition = &definitions[0];
                    Box::new(definition.consume(rules, remaining).flat_map(move |i| {
                        recurse(rules, &definitions[1..definitions.len()], i)
                    }))
                }
                recurse(rules, definitions, remaining)
            },
            Self::Or {definitions} => {
                Box::new(definitions.iter().flat_map(move |definition| {
                    definition.consume(rules, remaining)
                }))
            },
            Self::Reference {index} => {
                rules[index].consume(rules, remaining)
            },
            Self::String {string} => {
                if remaining.starts_with(string) {
                    let (_, remainder) = remaining.split_at(string.len());
                    Box::new(once(remainder))
                } else {
                    Box::new(empty())
                }
            },
        }
    }
}

type Input<'a> = (Rules, Vec<&'a str>);

fn parse_rule(i: &str) -> Parser {
    fn inner_parse_rule(i: &str) -> IResult<&str, Parser> {
        map(separated_list1(
            tag(" | "),
            map(separated_list1(
                char(' '),
                alt((
                    map(
                        delimited(char('"'), alpha1, char('"')),
                        |char: &str| -> Parser {
                            Parser::String { 
                                string: char.to_owned()
                            }
                        }
                    ),
                    map(digit1, |a: &str| -> Parser {
                        Parser::Reference {
                            index: a.parse().unwrap(),
                        }
                    }),
                ))
            ), |parts| -> Parser {
                if parts.len() == 1 {
                    return parts.into_iter().next().unwrap().into();
                }
                Parser::And {
                    definitions: parts,
                }
            })
        ), |parts| -> Parser {
            if parts.len() == 1 {
                return parts.into_iter().next().unwrap().into();
            }
            Parser::Or {
                definitions: parts,
            }
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

fn run(rules: &Rules, messages: &Vec<&str>) -> u64 {
    messages.iter().filter(|&&message| {
        let mut result = rules.get(&0).unwrap().consume(rules, message);

        result.find(|r| {
            r.len() == 0
        }).is_some().to_owned()
    }).count() as u64
}

fn part1(input: &Input) -> u64 {
    let rules = &input.0;
    let messages = &input.1;

    run(rules, messages)
}

fn part2(input: &Input) -> u64 {
    let rules = &input.0;
    let messages = &input.1;

    let mut updated_rules = (*rules).clone();
    updated_rules.insert(8, Parser::And{
        definitions: vec![
            Parser::Reference { index: 42 },
            Parser::Or {
                definitions: vec![
                    Parser::Reference { index: 8 },
                    Parser::String { string: "".to_owned() },
                ],
            },
        ]
    });
    updated_rules.insert(11, Parser::And{
        definitions: vec![
            Parser::Reference { index: 42 },
            Parser::Or {
                definitions: vec![
                    Parser::And{
                        definitions: vec![
                            Parser::Reference { index: 11 },
                            Parser::Reference { index: 31 },
                        ],
                    },
                    Parser::Reference { index: 31 },
                ],
            },
        ]
    });

    run(&updated_rules, messages)
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
        fn parse<'a, 'b>(parser: &'b Parser, i: &'a str) -> Vec<&'a str> {
            let rules: Rules = HashMap::new();
            parser.consume(&rules, i).collect()
        }

        assert_eq!(parse(&Parser::String{ string: "a".to_owned() }, "ab"), vec!["b"]);
        assert_eq!(parse(&Parser::String{ string: "".to_owned() }, "ab"), vec!["ab"]);

        assert_eq!(parse(&Parser::Or{definitions:vec![Parser::String{ string: "a".to_owned() }]}, "ab"), vec!["b"]);
        assert_eq!(parse(&Parser::Or{definitions:vec![Parser::String{ string: "b".to_owned() }, Parser::String{ string: "a".to_owned() }]}, "ab"), vec!["b"]);

        assert_eq!(parse(&Parser::And{definitions:vec![Parser::String{ string: "a".to_owned() }]}, "ab"), vec!["b"]);
        assert_eq!(parse(&Parser::And{definitions:vec![Parser::String{ string: "a".to_owned() }, Parser::String{ string: "b".to_owned() }]}, "ab"), vec![""]);
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