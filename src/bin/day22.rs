use std::{cmp::Ordering, collections::HashSet};

fn main() {
    let input = std::fs::read_to_string("inputs/day22.txt").unwrap();
    let (p1, p2) = parse_input(&input);
    let res = check_1((&p1, &p2));
    println!("Part 1 - Answer: {}", res);
    let res = check_2((&p1, &p2));
    println!("Part 2 - Answer: {}", res);
}

fn parse_input(input: &str) -> (Vec<u8>, Vec<u8>) {
    fn parse_player<'a>(lines: &mut impl Iterator<Item = &'a str>, n: u8) -> Vec<u8> {
        assert_eq!(lines.next(), Some(format!("Player {}:", n)).as_deref());
        let mut ret = vec![];
        for line in lines {
            if line.is_empty() {
                break;
            }
            ret.push(line.parse().unwrap());
        }
        ret
    }
    let mut lines = input.lines().map(str::trim);
    (parse_player(&mut lines, 1), parse_player(&mut lines, 2))
}

fn score(cards: &[u8]) -> usize {
    cards
        .iter()
        .rev()
        .enumerate()
        .map(|(i, x)| (i + 1) * *x as usize)
        .sum()
}

fn check_1(players: (&[u8], &[u8])) -> usize {
    let (p1, p2) = players;
    let mut p1 = p1.to_vec();
    let mut p2 = p2.to_vec();

    let winner = loop {
        let t1 = p1.remove(0);
        let t2 = p2.remove(0);
        if t1 > t2 {
            p1.push(t1);
            p1.push(t2);
        } else if t1 < t2 {
            p2.push(t2);
            p2.push(t1);
        } else {
            todo!("t1 == t2: {} == {}", t1, t2);
        }
        if p1.is_empty() {
            break p2;
        } else if p2.is_empty() {
            break p1;
        }
    };
    score(&winner)
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct Decks(Vec<u8>, Vec<u8>);

enum Player {
    P1,
    P2,
}

fn check_2(players: (&[u8], &[u8])) -> usize {
    fn round(current: &mut Decks) -> Player {
        let mut history = HashSet::<Decks>::new();
        loop {
            if history.contains(current) {
                return Player::P1;
            }
            history.insert(current.clone());
            let t1 = current.0.remove(0);
            let t2 = current.1.remove(0);
            let winner = if t1 as usize <= current.0.len() && t2 as usize <= current.1.len() {
                let mut decks = current.clone();
                decks.0.truncate(t1 as usize);
                decks.1.truncate(t2 as usize);
                round(&mut decks)
            } else {
                match t1.cmp(&t2) {
                    Ordering::Greater => Player::P1,
                    Ordering::Less => Player::P2,
                    Ordering::Equal => unreachable!(),
                }
            };
            match winner {
                Player::P1 => {
                    current.0.push(t1);
                    current.0.push(t2);
                }
                Player::P2 => {
                    current.1.push(t2);
                    current.1.push(t1);
                }
            }
            if current.0.is_empty() {
                return Player::P2;
            } else if current.1.is_empty() {
                return Player::P1;
            }
        }
    }

    let (p1, p2) = players;
    let p1 = p1.to_vec();
    let p2 = p2.to_vec();
    let mut decks = Decks(p1, p2);
    let winner = match round(&mut decks) {
        Player::P1 => decks.0,
        Player::P2 => decks.1,
    };
    score(&winner)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Player 1:
        9
        2
        6
        3
        1
        
        Player 2:
        5
        8
        4
        7
        10";

    #[test]
    fn test1() {
        let output = 306;
        let (p1, p2) = parse_input(INPUT);
        let res = check_1((&p1, &p2));
        assert_eq!(res, output);
    }

    #[test]
    fn test2() {
        let output = 291;
        let (p1, p2) = parse_input(INPUT);
        let res = check_2((&p1, &p2));
        assert_eq!(res, output);
    }

    #[test]
    fn test2_looping() {
        let input = "Player 1:
            43
            19
            
            Player 2:
            2
            29
            14";
        let (p1, p2) = parse_input(input);
        check_2((&p1, &p2));
    }
}
