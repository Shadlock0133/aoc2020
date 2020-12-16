use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

fn main() {
    let input = std::fs::read_to_string("inputs/day16.txt").unwrap();
    let notes = parse_input(&input);
    let res = check_1(&notes);
    println!("Part 1 - Answer: {}", res);
    let res = check_2(&notes);
    println!("Part 2 - Answer: {}", res);
}

struct Notes<'a> {
    rules: HashMap<&'a str, Vec<RangeInclusive<usize>>>,
    your: Vec<usize>,
    nearby: Vec<Vec<usize>>,
}

fn parse_input(input: &str) -> Notes {
    let mut lines = input.lines().map(str::trim);
    let mut rules = HashMap::new();
    for line in &mut lines {
        if line.is_empty() {
            break;
        }
        let mut split = line.split(": ");
        let name = split.next().unwrap();
        let ranges = split
            .next()
            .unwrap()
            .split(" or ")
            .map(|x| {
                let mut range = x.split('-');
                let a = range.next().unwrap().parse().unwrap();
                let b = range.next().unwrap().parse().unwrap();
                a..=b
            })
            .collect();
        rules.insert(name, ranges);
    }
    let len = rules.len();

    assert_eq!(lines.next(), Some("your ticket:"));
    let your = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(your.len(), len);
    assert_eq!(lines.next(), Some(""));
    assert_eq!(lines.next(), Some("nearby tickets:"));
    let nearby = lines
        .map(|x| {
            let ticket = x.split(',').map(|x| x.parse().unwrap()).collect::<Vec<_>>();
            assert_eq!(ticket.len(), len);
            ticket
        })
        .collect();

    Notes {
        rules,
        your,
        nearby,
    }
}

fn check_1(notes: &Notes) -> usize {
    let valid_values = notes
        .rules
        .values()
        .flatten()
        .cloned()
        .flatten()
        .collect::<HashSet<usize>>();
    notes
        .nearby
        .iter()
        .flatten()
        .filter(|x| !valid_values.contains(x))
        .sum()
}

fn determine_rules_order<'a>(notes: &Notes<'a>) -> Vec<&'a str> {
    let valid_values = notes
        .rules
        .values()
        .flatten()
        .cloned()
        .flatten()
        .collect::<HashSet<usize>>();
    let valid_nearby = notes
        .nearby
        .iter()
        .filter(|x| x.iter().all(|x| valid_values.contains(x)))
        .collect::<Vec<_>>();

    let mut possible_rules: Vec<Vec<_>> = vec![];
    // find all possible rules for each column
    for column in 0..notes.rules.len() {
        let rules = notes
            .rules
            .iter()
            .filter(|(_, ranges)| {
                valid_nearby
                    .iter()
                    .all(|ticket| ranges.iter().any(|range| range.contains(&ticket[column])))
            })
            .map(|x| x.0)
            .collect();
        possible_rules.push(rules);
    }
    // dedup rules
    while possible_rules.iter().any(|x| x.len() > 1) {
        let singles = possible_rules.iter().cloned().filter(|x| x.len() == 1).flatten().collect::<Vec<_>>();
        for column in possible_rules.iter_mut().filter(|x| x.len() > 1) {
            column.retain(|x| !singles.contains(&x));
        }
    }
    assert!(possible_rules.iter().all(|x| x.len() == 1));
    possible_rules.into_iter().flatten().copied().collect()
}

fn check_2(notes: &Notes) -> usize {
    let rules = determine_rules_order(notes);
    rules
        .iter()
        .zip(notes.your.iter())
        .filter_map(|(name, value)| {
            if name.starts_with("departure") {
                Some(value)
            } else {
                None
            }
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "class: 1-3 or 5-7
            row: 6-11 or 33-44
            seat: 13-40 or 45-50
            
            your ticket:
            7,1,14
            
            nearby tickets:
            7,3,47
            40,4,50
            55,2,20
            38,6,12";
        let output = 71;
        let res = check_1(&parse_input(input));
        assert_eq!(res, output);
    }

    #[test]
    fn test2() {
        let input = "class: 0-1 or 4-19
            row: 0-5 or 8-19
            seat: 0-13 or 16-19
            
            your ticket:
            11,12,13
            
            nearby tickets:
            3,9,18
            15,1,5
            5,14,9";
        let output = ["row", "class", "seat"];
        let res = determine_rules_order(&parse_input(input));
        assert_eq!(res, output);
    }
}
