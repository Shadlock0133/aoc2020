fn main() {
    let input = std::fs::read_to_string("inputs/day1.txt").unwrap();
    let list = parse_input(&input).unwrap();
    let (a, b) = find_pair(&list).unwrap();
    println!("Part 1 - Answer: {}", a * b);
    let (a, b, c) = find_triple(&list).unwrap();
    println!("Part 2 - Answer: {}", a * b * c);
}

fn parse_input(input: &str) -> Result<Vec<u32>, std::num::ParseIntError> {
    input.lines().map(|x| x.trim().parse()).collect()
}

fn find_pair(list: &[u32]) -> Option<(u32, u32)> {
    assert!(list.len() >= 2);
    for first in 0..list.len() - 1 {
        for second in first + 1..list.len() {
            let a = list[first];
            let b = list[second];
            if a + b == 2020 {
                return Some((a, b));
            }
        }
    }
    None
}

fn find_triple(list: &[u32]) -> Option<(u32, u32, u32)> {
    assert!(list.len() >= 3);
    for first in 0..list.len() - 2 {
        for second in first + 1..list.len() - 1 {
            for third in second + 1..list.len() {
                let a = list[first];
                let b = list[second];
                let c = list[third];
                if a + b + c == 2020 {
                    return Some((a, b, c));
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1721
        979
        366
        299
        675
        1456";

    #[test]
    fn test1() {
        let output = 514579;
        assert_eq!(
            find_pair(&parse_input(INPUT).unwrap())
                .map(|(a, b)| a * b)
                .unwrap(),
            output
        );
    }

    #[test]
    fn test2() {
        let output = 241861950;
        assert_eq!(
            find_triple(&parse_input(INPUT).unwrap())
                .map(|(a, b, c)| a * b * c)
                .unwrap(),
            output
        );
    }
}
