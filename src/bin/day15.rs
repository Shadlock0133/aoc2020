use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("inputs/day15.txt").unwrap();
    let list = parse_input(&input);
    let res = check_1(&list, 2020);
    println!("Part 1 - Answer: {}", res);
    let res = check_2(&list, 30000000);
    println!("Part 2 - Answer: {}", res);
}

fn parse_input(input: &str) -> Vec<u32> {
    input.split(',').map(|x| x.parse().unwrap()).collect()
}

fn check_1(list: &[u32], nth: usize) -> u32 {
    let mut seen = list.to_vec();
    for turn in seen.len()..nth {
        let (last, rest) = seen.split_last().unwrap();
        let pos = rest.iter().rposition(|x| x == last);
        let next = match pos {
            Some(pos) => (turn - pos - 1) as u32,
            None => 0,
        };
        seen.push(next);
    }
    *seen.last().unwrap()
}

fn check_2(list: &[u32], nth: usize) -> u32 {
    let mut last_seen: HashMap<_, _> = list //
        .iter()
        .enumerate()
        .map(|(i, x)| (*x, i + 1))
        .collect();
    let mut last = *list.last().unwrap();
    for turn in last_seen.len()..nth {
        last = last_seen
            .insert(last, turn)
            .map(|x| (turn - x) as u32)
            .unwrap_or_default();
    }
    last
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let tests = [
            ("0,3,6", 436),
            ("1,3,2", 1),
            ("2,1,3", 10),
            ("1,2,3", 27),
            ("2,3,1", 78),
            ("3,2,1", 438),
            ("3,1,2", 1836),
        ];
        for &(input, output) in &tests {
            let res = check_1(&parse_input(input), 2020);
            assert_eq!(res, output);
            let res = check_2(&parse_input(input), 2020);
            assert_eq!(res, output);
        }
    }

    #[test]
    #[ignore = "takes too long"]
    fn test2() {
        let tests = [
            ("0,3,6", 175594),
            ("1,3,2", 2578),
            ("2,1,3", 3544142),
            ("1,2,3", 261214),
            ("2,3,1", 6895259),
            ("3,2,1", 18),
            ("3,1,2", 362),
        ];
        for &(input, output) in &tests {
            let res = check_2(&parse_input(input), 30000000);
            assert_eq!(res, output);
        }
    }
}
