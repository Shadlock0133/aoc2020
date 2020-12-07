use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("inputs/day7.txt").unwrap();
    let rules = parse_input(&input);
    let res = check_1(&rules);
    println!("Part 1 - Answer: {}", res);
    let res = check_2(&rules);
    println!("Part 2 - Answer: {}", res);
}

type Rule<'a> = (usize, &'a str);
type Rules<'a> = HashMap<&'a str, Vec<Rule<'a>>>;

fn parse_input(input: &str) -> Rules {
    input
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }
            let mut iter = line.split(" bags contain ");
            let color = iter.next().unwrap().trim();
            let rest: Vec<_> = iter.next().unwrap().split(',').collect();
            if rest[0].trim() == "no other bags." {
                return Some((color, vec![]));
            }
            let content = rest
                .iter()
                .map(|x| {
                    let x = x.trim();
                    let n = x.split_whitespace().next().unwrap();
                    let color = x
                        .strip_prefix(n)
                        .unwrap()
                        .trim()
                        .trim_end_matches('.')
                        .trim_end_matches('s')
                        .strip_suffix(" bag")
                        .unwrap();
                    let n = n.parse().unwrap();
                    (n, color)
                })
                .collect();
            Some((color, content))
        })
        .collect()
}

fn check_1(rules: &Rules) -> usize {
    fn check_bag<'a>(color: &str, rule: &[Rule<'a>], rules: &Rules<'a>) -> bool {
        if rule.iter().any(|(_, name)| *name == color) {
            return true;
        }
        rule.iter().any(|(_, name)| {
            let rule = &rules[name];
            check_bag(color, rule, rules)
        })
    }
    rules
        .values()
        .filter(|rule| check_bag("shiny gold", rule, rules))
        .count()
}

fn check_2(rules: &Rules) -> usize {
    fn check_bag<'a>(rule: &[Rule<'a>], rules: &Rules<'a>) -> usize {
        rule.iter()
            .map(|(count, name)| {
                let rule = &rules[name];
                count * (1 + check_bag(rule, rules))
            })
            .sum()
    }
    let shiny_gold = &rules["shiny gold"];
    check_bag(shiny_gold, rules)
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

    #[test]
    fn test2_1() {
        let output = 32;
        let res = check_2(&parse_input(INPUT));
        assert_eq!(res, output);
    }

    #[test]
    fn test2_2() {
        const INPUT2: &str = "shiny gold bags contain 2 dark red bags.
            dark red bags contain 2 dark orange bags.
            dark orange bags contain 2 dark yellow bags.
            dark yellow bags contain 2 dark green bags.
            dark green bags contain 2 dark blue bags.
            dark blue bags contain 2 dark violet bags.
            dark violet bags contain no other bags.";
        let output = 126;
        let res = check_2(&parse_input(INPUT2));
        assert_eq!(res, output);
    }
}
