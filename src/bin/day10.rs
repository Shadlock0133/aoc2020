use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("inputs/day10.txt").unwrap();
    let list = parse_input(&input);
    let res = check_1(&list);
    println!("Part 1 - Answer: {}", res);
    let res = check_2(&list);
    println!("Part 2 - Answer: {}", res);
}

fn parse_input(input: &str) -> Vec<u32> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.trim().parse().unwrap())
        .collect()
}

fn check_1(list: &[u32]) -> usize {
    let mut list = list.to_vec();
    list.sort_unstable();
    list.push(list.last().unwrap() + 3); // Laptop input joltage
    let iter = list
        .iter()
        .scan(0, |acc, &x| {
            let old = *acc;
            *acc = x;
            Some(x - old)
        });
    let mut map = HashMap::<_, usize>::new();
    for n in iter {
        *map.entry(n).or_default() += 1;
    }
    map[&1] * map[&3]
}

fn check_2(list: &[u32]) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str =
        "16
        10
        15
        5
        1
        11
        7
        19
        6
        12
        4";

    const INPUT2: &str =
        "28
        33
        18
        42
        31
        14
        46
        20
        48
        47
        24
        23
        49
        45
        19
        38
        39
        11
        1
        32
        25
        35
        8
        17
        7
        9
        4
        2
        34
        10
        3";

    #[test]
    fn test1() {
        let output = 7 * 5;
        let res = check_1(&parse_input(INPUT));
        assert_eq!(res, output);
        let output = 22 * 10;
        let res = check_1(&parse_input(INPUT2));
        assert_eq!(res, output);
    }

    #[test]
    #[ignore = "not implemented"]
    fn test2() {
        let output = 8;
        let res = check_2(&parse_input(INPUT));
        assert_eq!(res, output);
        let output = 19208;
        let res = check_2(&parse_input(INPUT2));
        assert_eq!(res, output);
    }
}
