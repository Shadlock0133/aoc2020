use std::collections::VecDeque;

fn main() {
    let input = std::fs::read_to_string("inputs/day23.txt").unwrap();
    let list = parse_input(&input);
    let res = check_1(&list, 100);
    println!("Part 1 - Answer: {}", res);
    let res = check_2(&list, 1_000_000, 10_000_000);
    println!("Part 2 - Answer: {}", res);
}

fn parse_input(input: &str) -> Vec<u64> {
    input
        .chars()
        .filter_map(|x| x.to_digit(10).map(|x| x as u64))
        .collect()
}

fn output(mut list: VecDeque<u64>) -> String {
    list.rotate_left(list.iter().position(|x| *x == 1).unwrap());
    list.into_iter()
        .skip(1)
        .map(|x| std::char::from_digit(x as u32, 10).unwrap())
        .collect()
}

fn round(list: &mut VecDeque<u64>, highest: u64) {
    let mut current = list.front().copied().unwrap() - 1;
    if current == 0 {
        current = highest;
    }
    let pick_up = list.drain(1..4).collect::<Vec<_>>();
    let dest = loop {
        match list.iter().position(|x| *x == current) {
            Some(n) => break n,
            None => {
                current -= 1;
                if current == 0 {
                    current = highest;
                }
            }
        }
    };
    for (i, value) in pick_up.into_iter().enumerate() {
        list.insert(dest + i + 1, value);
    }
    list.rotate_left(1);
}

fn check_1(list: &[u64], n: usize) -> String {
    let highest = list.iter().max().copied().unwrap();
    let mut list = list.iter().copied().collect::<VecDeque<_>>();
    for _ in 0..n {
        round(&mut list, highest);
    }
    output(list)
}

fn check_2(list: &[u64], cups: u64, n: usize) -> u64 {
    let highest = list.iter().max().copied().unwrap();
    let mut list = list
        .iter()
        .copied()
        .chain(highest + 1..=cups)
        .collect::<VecDeque<_>>();
    assert_eq!(list.len(), cups as usize);
    let highest = list.iter().max().copied().unwrap();
    for i in 0..n {
        if i % 1000 == 0 {
            dbg!(i);
        }
        round(&mut list, highest);
    }
    list.rotate_left(list.iter().position(|x| *x == 1).unwrap());
    assert_eq!(list.pop_front().unwrap(), 1);
    let a = list.pop_front().unwrap();
    let b = list.pop_front().unwrap();
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "389125467";

    #[test]
    fn test1() {
        let output = "92658374";
        let res = check_1(&parse_input(INPUT), 10);
        assert_eq!(res, output);
        let output = "67384529";
        let res = check_1(&parse_input(INPUT), 100);
        assert_eq!(res, output);
    }

    #[test]
    #[ignore]
    fn test2() {
        let output = 149245887792;
        let res = check_2(&parse_input(INPUT), 1_000_000, 10_000_000);
        assert_eq!(res, output);
    }
}
