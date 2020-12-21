use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("inputs/day19.txt").unwrap();
    let parsed = parse_input(&input);
    let res = check_1(&parsed);
    println!("Part 1 - Answer: {}", res);
}

#[derive(Debug)]
enum Rule {
    Char(char),
    SubRules(Vec<Vec<usize>>),
}

#[derive(Debug)]
struct Input<'a> {
    rules: HashMap<usize, Rule>,
    messages: Vec<&'a str>,
}

fn parse_input(input: &str) -> Input {
    let mut lines = input.lines().map(str::trim);
    let mut rules = HashMap::new();
    for line in &mut lines {
        if line.is_empty() {
            break;
        }
        let mut split = line.split(": ");
        let n = split.next().unwrap().parse().unwrap();
        let rule = split.next().unwrap();
        let rule = if let Some(rule) = rule.strip_prefix('"') {
            let char = rule.strip_suffix('"').unwrap();
            assert_eq!(char.chars().count(), 1);
            Rule::Char(char.chars().next().unwrap())
        } else {
            Rule::SubRules(
                rule.split(" | ")
                    .map(|x| x.split_whitespace().map(|x| x.parse().unwrap()).collect())
                    .collect(),
            )
        };
        rules.insert(n, rule);
    }
    let messages = lines.collect();
    Input { rules, messages }
}

fn check_message(rules: &HashMap<usize, Rule>, mut message: &str) -> bool {
    fn check_rule(
        rules: &HashMap<usize, Rule>,
        rule: usize,
        message: &mut &str,
    ) -> bool {
        dbg!(rule, &message);
        match &rules[&rule] {
            Rule::Char(c) => {
                let mut m = message.chars();
                let ret = m.next() == Some(*c);
                *message = m.as_str();
                ret
            },
            Rule::SubRules(subrules) => {
                let m = &mut *message;
                let r = subrules
                    .iter()
                    .any(|r| {
                        r.iter().all(|x| {
                            check_rule(rules, *x, m)
                        })
                    });
                if r {
                    *message = m;
                }
                r
            },
        }
    }
    check_rule(rules, 0, &mut message)
}

fn check_1_lazy(input: &Input) -> usize {
    input
        .messages
        .iter()
        .filter(|m| check_message(&input.rules, m))
        .count()
}

// fn check_1_eager(input: &Input) -> usize {
//     let mut valid_messages = vec![];
    
//     input
//         .messages
//         .iter()
//         .filter(|m| valid_messages.contains(&m))
//         .count()
// }

fn check_1(input: &Input) -> usize {
    check_1_lazy(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"0: 4 1 5
        1: 2 3 | 3 2
        2: 4 4 | 5 5
        3: 4 5 | 5 4
        4: "a"
        5: "b"
        
        ababbb
        bababa
        abbbab
        aaabbb
        aaaabbb"#;

    #[test]
    #[ignore]
    fn test1() {
        let output = 2;
        let res = check_1(&parse_input(INPUT));
        assert_eq!(res, output);
    }
}
