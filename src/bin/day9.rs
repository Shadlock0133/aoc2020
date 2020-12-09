fn main() {
    let input = std::fs::read_to_string("inputs/day9.txt").unwrap();
    let data = parse_input(&input);
    let res = check_1(&data, 25);
    println!("Part 1 - Answer: {}", res);
    let res = check_2(&data, 25);
    println!("Part 2 - Answer: {}", res);
}

fn parse_input(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

fn is_valid(preamble: &[u64], data: u64) -> bool {
    for i in 0..preamble.len() - 1 {
        for j in i..preamble.len() {
            if preamble[i] + preamble[j] == data {
                return true;
            }
        }
    }
    false
}

fn check_1(data: &[u64], preamble: usize) -> u64 {
    for i in 0..data.len() - preamble {
        let (data, rest) = data.split_at(i).1.split_at(preamble);
        let x = *rest.split_first().unwrap().0;
        if !is_valid(data, x) {
            return x;
        }
    }
    unreachable!()
}

fn check_2(data: &[u64], preamble: usize) -> u64 {
    let target = check_1(data, preamble);
    'index: for i in 0..data.len() - 1 {
        for len in 2.. {
            let range = &data[i..][..len];
            let sum: u64 = range.iter().sum();
            if sum == target {
                let min = range.iter().min().unwrap();
                let max = range.iter().max().unwrap();
                return min + max;
            } else if sum > target {
                continue 'index;
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str =
        "35
        20
        15
        25
        47
        40
        62
        55
        65
        95
        102
        117
        150
        182
        127
        219
        299
        277
        309
        576";

    #[test]
    fn test1() {
        let output = 127;
        let data = parse_input(INPUT);
        let res = check_1(&data, 5);
        assert_eq!(res, output);
    }

    #[test]
    fn test2() {
        let output = 62;
        let data = parse_input(INPUT);
        let res = check_2(&data, 5);
        assert_eq!(res, output);
    }
}
