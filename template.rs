fn main() {
    let input = std::fs::read_to_string("inputs/day!.txt").unwrap();
    let rules = parse_input(&input);
    let res = check_1(&rules);
    println!("Part 1 - Answer: {}", res);
}

fn parse_input(input: &str) -> ! {
    todo!()
}

fn check_1(rules: &!) -> ! {
    
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str =
        "";

    #[test]
    fn test1() {
        let output = 4;
        let res = check_1(&parse_input(INPUT));
        assert_eq!(res, output);
    }
}
