use std::convert::TryInto;

fn main() {
    let input = std::fs::read_to_string("inputs/day5.txt").unwrap();
    let lines = parse_input(&input);
    let ids: Vec<_> = lines.iter().map(calculate_id).collect();
    let max = ids.iter().max().unwrap();
    println!("Part 1 - Answer: {}", max);
    let seat = find_seat(&ids);
    println!("Part 2 - Answer: {}", seat);
}

fn parse_input(input: &str) -> Vec<(u8, u8)> {
    input
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() { return None; }
            let mut row = 0;
            let mut column = 0;
            for ch in line[..7].chars() {
                row <<= 1;
                row |= match ch {
                    'F' => 0,
                    'B' => 1,
                    _ => unreachable!(),
                };
            }
            for ch in line[7..].chars() {
                column <<= 1;
                column |= match ch {
                    'L' => 0,
                    'R' => 1,
                    _ => unreachable!(),
                };
            }
            Some((row, column))
        })
        .collect()
}

fn calculate_id(pos: &(u8, u8)) -> u16 {
    let &(row, col) = pos;
    (row as u16) << 3 | col as u16
}

fn find_seat(list: &[u16]) -> u16 {
    let mut list = list.to_vec();
    list.sort();
    for seats in list.windows(2) {
        let [a, b]: [u16; 2] = seats.try_into().unwrap();
        if a + 2 == b {
            return a + 1;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        const INPUT: &str =
            "BFFFBBFRRR
            FFFBBBFRRR
            BBFFBBFRLL";
        let res = parse_input(INPUT);
        let output = [(70, 7), (14, 7), (102, 4)];
        assert_eq!(res, output);
        let res: Vec<_> = res.iter().map(calculate_id).collect();
        let output = [567, 119, 820];
        assert_eq!(res, output);
    }
}