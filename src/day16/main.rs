use std::fs;
use aoc_2020_rust::util::{bench, parser::parse_decimal_u64, bitset::SmallIntegerSet64};
use nom::{IResult, bytes::complete::{is_not, tag}, character::complete::{char, newline}, combinator::opt, multi::many1, sequence::{preceded, separated_pair, terminated}};

type Range = (u64, u64);
type FieldAndRanges<'a> = (&'a [u8], (Range, Range));
type Fields<'a> = Vec<FieldAndRanges<'a>>;
type Ticket = Vec<u64>;

#[derive(Debug, PartialEq)]
struct PreparedInput<'a> {
    fields: Vec<FieldAndRanges<'a>>,
    your_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

fn parse_range(i: &[u8]) -> IResult<&[u8], Range> {
    separated_pair(parse_decimal_u64, tag("-"), parse_decimal_u64)(i)
}
fn parse_fieldname_and_ranges(i: &[u8]) -> IResult<&[u8], FieldAndRanges<'_>> {
    separated_pair(
        is_not(":"), 
        tag(": "), 
        separated_pair(
            parse_range, 
            tag(" or "), 
            parse_range
        )
    )(i)
}
fn parse_fields(i: &[u8]) -> IResult<&[u8], Fields<'_>> {
    many1(
        terminated(
            parse_fieldname_and_ranges,
            newline
        )
    )(i)
}

fn parse_ticket(i: &[u8]) -> IResult<&[u8], Ticket> {
    many1(
        terminated(
            parse_decimal_u64,
            opt(char(','))
        )
    )(i)
}

fn parse_your_ticket(i: &[u8]) -> IResult<&[u8], Ticket> {
    preceded(
        tag("your ticket:\n"),
        parse_ticket
    )(i)
}

fn parse_nearby_tickets(i: &[u8]) -> IResult<&[u8], Vec<Ticket>> {
    preceded(
        tag("nearby tickets:\n"),
        many1(terminated(
            parse_ticket, 
            opt(newline)
        ))
    )(i)
}

fn parse(i: &[u8]) -> IResult<&[u8], PreparedInput> {
    let (i, fields) = parse_fields(i)?;
    let (i, _) = newline(i)?;
    let (i, your_ticket) = parse_your_ticket(i)?;
    let (i, _) = newline(i)?;
    let (i, _) = newline(i)?;
    let (i, nearby_tickets) = parse_nearby_tickets(i)?;

    assert!(i.len() == 0);
    Ok((i, PreparedInput {
        fields,
        your_ticket,
        nearby_tickets,
    }))
}

fn prepare_input(input: &str) -> PreparedInput {
    parse(input.as_bytes()).unwrap().1
}

fn create_valid_map(fields: &Fields) -> [bool;1000] {
    fields.iter().fold(
        [false; 1000], 
        |mut valid, (_, ((a1, a2), (b1, b2)))| {
            for i in *a1..*a2+1 {
                valid[i as usize] = true;
            }
            for i in *b1..*b2+1 {
                valid[i as usize] = true;
            }
            valid
        }
    )
}

fn create_number_to_valid_fields(fields: &Fields) -> Vec<SmallIntegerSet64> {
    fields.iter().enumerate().fold(
        vec![SmallIntegerSet64::new(); 1000],
        |mut number_to_fields, (field_index,(_, ((a1, a2), (b1, b2))))| {
            for i in *a1..*a2+1 {
                number_to_fields[i as usize].insert(field_index);
            }
            for i in *b1..*b2+1 {
                number_to_fields[i as usize].insert(field_index);
            }
            number_to_fields
        }
    )
}

fn part1(input: &PreparedInput) -> u64 {
    let valid = create_valid_map(&input.fields);

    input.nearby_tickets.iter().flat_map(|ticket| {
        ticket.iter()
    }).filter(|&&number| {
        !valid[number as usize]
    }).sum()
}

fn validate(input: &PreparedInput, column_to_field: &Vec<usize>) {
    let number_to_valid_fields = create_number_to_valid_fields(&input.fields);

    let filtered_tickets = input.nearby_tickets.iter().filter(|ticket| {
        ticket.iter().all(|&number| number_to_valid_fields[number as usize].empty() != true)
    });
    filtered_tickets.for_each(|ticket| {
        ticket.iter().enumerate().for_each(|(column, number)| {
            let field = column_to_field[column];
            let is_valid = number_to_valid_fields[*number as usize].contains(field);
            assert!(is_valid, "{:?} {} {}", ticket, column, number);
        });
    });
}

fn part2(input: &PreparedInput) -> Vec<u64> {
    let number_to_valid_fields = create_number_to_valid_fields(&input.fields);

    let filtered_tickets = input.nearby_tickets.iter().filter(|ticket| {
        ticket.iter().all(|&number| number_to_valid_fields[number as usize].empty() != true)
    });

    let mut column_to_fields = vec![SmallIntegerSet64::new_filled(); 20];
    filtered_tickets.for_each(|ticket| {
        ticket.iter().enumerate().for_each(|(column, &number)| {
            let valid_fields = &number_to_valid_fields[number as usize];
            let considering_fields = &mut column_to_fields[column];
            considering_fields.retain_intersection(valid_fields);
        });
    });

    let mut known_column_to_field: Vec<Option<usize>> = vec![None; input.fields.len()];
    for _iteration in 0..input.fields.len() {
        let one_left = column_to_fields.iter_mut().enumerate().find(|(_, fields)| fields.len() == 1);
        let (column_index, fields_left) = one_left.expect("Expected to have a match for a single one left");
        let field_index = fields_left.pop().unwrap();
        known_column_to_field[column_index] = Some(field_index);

        column_to_fields.iter_mut().for_each(|fields| {
            fields.remove(&field_index);
        });
    }

    let column_to_field = known_column_to_field.iter().map(|column| column.unwrap()).collect();

    // validate
    validate(input, &column_to_field);

    let field_to_column = column_to_field.iter().enumerate().fold(
        vec![0; input.fields.len()],
        |mut field_to_column, (column, field)| {
            field_to_column[*field] = column;
            field_to_column
        }
    );
    
    field_to_column.iter().map(|&column| {
        input.your_ticket[column]
    }).collect()
}

fn main() {
    let input = fs::read_to_string("./src/day16/input.txt").expect("Unable to read input file");

    let prepared_input = bench::time("input preparation", || prepare_input(&input));
    let part1 = bench::time("part 1", || part1(&prepared_input));
    let part2 = bench::time(
        "part 2", 
        || part2(&prepared_input).iter().take(6).fold(1, |mult: u64, &number| {
            mult * number as u64
        })
    );

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT1: &str = 
"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    #[test]
    fn parse_example() {
        assert_eq!(parse_range("1-3".as_bytes()), Ok(("".as_bytes(), (1, 3))));
        assert_eq!(parse_fieldname_and_ranges("class: 1-3 or 5-7".as_bytes()), Ok(("".as_bytes(), ("class".as_bytes(), ((1, 3), (5, 7))))));
        
        assert_eq!(parse_ticket("7,1,14".as_bytes()), Ok(("".as_bytes(), vec![7, 1, 14])));

        assert_eq!(prepare_input(EXAMPLE_INPUT1), PreparedInput {
            fields: vec![
                ("class".as_bytes(), ((1,3), (5,7))),
                ("row".as_bytes(), ((6,11), (33,44))),
                ("seat".as_bytes(), ((13,40), (45,50))),
            ],
            your_ticket: vec![7,1,14],
            nearby_tickets: vec![
                vec![7,3,47],
                vec![40,4,50],
                vec![55,2,20],
                vec![38,6,12],
            ],
        });
    }
    
    #[test]
    fn part1_example() {
        assert_eq!(part1(&prepare_input(EXAMPLE_INPUT1)), 71);
    }

    const EXAMPLE_INPUT2: &str = 
"class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";
    
    #[test]
    fn part2_example() {
        assert_eq!(part2(&prepare_input(EXAMPLE_INPUT2)), vec![12, 11, 13]);
    }
}