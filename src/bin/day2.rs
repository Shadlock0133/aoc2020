fn main() {
    let input = std::fs::read_to_string("inputs/day2.txt").unwrap();
    let list = parse_input(&input).unwrap();
    let res = list.iter().filter(|x| check_correct_1(x)).count();
    println!("Part 1 - Answer: {}", res);
    let res = list.iter().filter(|x| check_correct_2(x)).count();
    println!("Part 2 - Answer: {}", res);
}

fn parse_input(input: &str) -> Result<Vec<(u8, u8, char, &str)>, Box<dyn std::error::Error>> {
    input
        .lines()
        .map(|x| {
            let all: Vec<_> = x.split(':').collect();
            let abc: Vec<_> = all[0].split('-').collect();
            let a = abc[0].trim().parse()?;
            let bc: Vec<_> = abc[1].split_whitespace().collect();
            let b = bc[0].parse()?;
            let c = bc[1].parse()?;
            let password = all[1].trim();
            Ok((a, b, c, password))
        })
        .collect()
}

fn check_correct_1(line: &(u8, u8, char, &str)) -> bool {
    let &(lower, upper, char, password) = line;
    let count = password.chars().filter(|x| *x == char).count() as u8;
    (lower..=upper).contains(&count)
}

fn check_correct_2(line: &(u8, u8, char, &str)) -> bool {
    let &(a, b, char, password) = line;
    let first = password.chars().nth(a as usize - 1).unwrap() == char;
    let second = password.chars().nth(b as usize - 1).unwrap() == char;
    first ^ second
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1-3 a: abcde
    1-3 b: cdefg
    2-9 c: ccccccccc";

    #[test]
    fn test1() {
        let output = 2;
        let res = parse_input(INPUT)
            .unwrap()
            .into_iter()
            .filter(check_correct_1)
            .count();
        assert_eq!(res, output);
    }

    #[test]
    fn test2() {
        let output = 1;
        let res = parse_input(INPUT)
            .unwrap()
            .into_iter()
            .filter(check_correct_2)
            .count();
        assert_eq!(res, output);
    }
}
