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

    const INPUT: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
        dark orange bags contain 3 bright white bags, 4 muted yellow bags.
        bright white bags contain 1 shiny gold bag.
        muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
        shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
        dark olive bags contain 3 faded blue bags, 4 dotted black bags.
        vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
        faded blue bags contain no other bags.
        dotted black bags contain no other bags.";

    #[test]
    fn test1() {
        let output = 4;
        let res = check_1(&parse_input(INPUT));
        assert_eq!(res, output);
    }
}