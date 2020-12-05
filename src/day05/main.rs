use std::fs;
use std::time;

type SeatId = u16;

fn parse(value: &[u8]) -> SeatId {
    let mut inverted_seat_id: SeatId = 0;

    for i in 0 .. 10 {
        inverted_seat_id |= ((value[i] as SeatId & 4) >> 2) << (9 - i);
    }

    return 1023 ^ inverted_seat_id;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parser_works() {
        assert_eq!(parse("FBFBBFFRLR".as_bytes()), 357);
        assert_eq!(parse("BFFFBBFRRR".as_bytes()), 567);
        assert_eq!(parse("FFFBBBFRRR".as_bytes()), 119);
        assert_eq!(parse("BBFFBBFRLL".as_bytes()), 820);
    }
}

fn main() {
    let input = fs::read("./src/day05/input.txt").expect("Unable to read input file");

    let start = time::Instant::now();

    let mut min_seat_id: SeatId = SeatId::MAX;
    let mut max_seat_id: SeatId = 0;
    let mut sum: u32 = 0;

    for chunk in input.chunks_exact(11) {
        let seat_id = parse(&chunk);
        min_seat_id = min_seat_id.min(seat_id);
        max_seat_id = max_seat_id.max(seat_id);
        sum += seat_id as u32;
    }

    let expected_sum = ((min_seat_id + max_seat_id) as f32 / 2.0) * (max_seat_id - min_seat_id + 1) as f32;
    let free_seat_id = expected_sum as u32 - sum;

    let end = time::Instant::now();

    println!("Min, max: {}, {}", min_seat_id, max_seat_id);
    println!("Free: {}", free_seat_id);

    println!("Time: {:?}", end - start);
}
