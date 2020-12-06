use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("inputs/day6.txt").unwrap();
    let groups = parse_input(&input);
    let res = calculate_sum(&groups);
    println!("Part 1 - Answer: {}", res);
    let res = calculate_product(&groups);
    println!("Part 2 - Answer: {}", res);
}

type Group = Vec<HashSet<char>>;

fn parse_input(input: &str) -> Vec<Group> {
    let mut res = vec![];
    let mut group = vec![];
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            res.push(std::mem::take(&mut group));
            continue;
        }
        group.push(line.chars().collect());
    }
    if !group.is_empty() {
        res.push(group);
    }
    res
}

fn calculate_sum(groups: &[Group]) -> usize {
    groups
        .iter()
        .map(|group| {
            group.iter().skip(1).fold(group[0].clone(), |acc, x| &acc | x)
        })
        .map(|group| group.len())
        .sum()
}

fn calculate_product(groups: &[Group]) -> usize {
    groups
        .iter()
        .map(|group| {
            group.iter().skip(1).fold(group[0].clone(), |acc, x| &acc & x)
        })
        .map(|group| group.len())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = 
        "abc

        a
        b
        c
        
        ab
        ac
        
        a
        a
        a
        a
        
        b";

    #[test]
    fn test1() {
        let output = 11;
        let res = calculate_sum(&parse_input(INPUT));
        assert_eq!(res, output);
    }

    #[test]
    fn test2() {
        let output = 6;
        let res = calculate_product(&parse_input(INPUT));
        assert_eq!(res, output);
    }
}