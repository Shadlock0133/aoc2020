use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("inputs/day14.txt").unwrap();
    let ops = parse_input(&input);
    let res = check_1(&ops);
    println!("Part 1 - Answer: {}", res);
    let res = check_2(&ops);
    println!("Part 2 - Answer: {}", res);
}

#[derive(Debug)]
enum Op<'a> {
    Mask(&'a str),
    Mem { address: u64, value: u64 },
}

fn parse_input(input: &str) -> Vec<Op> {
    input
        .lines()
        .map(str::trim)
        .filter(|x| !x.is_empty())
        .map(|line| {
            if let Some(mask) = line.strip_prefix("mask = ") {
                Op::Mask(mask)
            } else if let Some(rest) = line.strip_prefix("mem[") {
                let mut rest = rest.split(']');
                let address = rest.next().unwrap().parse().unwrap();
                let value = rest
                    .next()
                    .unwrap()
                    .strip_prefix(" = ")
                    .unwrap()
                    .parse()
                    .unwrap();
                Op::Mem { address, value }
            } else {
                panic!("unknown op: {}", line)
            }
        })
        .collect()
}

fn check_1(ops: &[Op]) -> u64 {
    let mut mem = HashMap::new();
    let mut and = 0;
    let mut or = 0;
    for op in ops {
        match op {
            Op::Mask(mask) => {
                and = 0;
                or = 0;
                for ch in mask.chars() {
                    and <<= 1;
                    or <<= 1;
                    if ch == '1' {
                        or |= 1;
                    }
                    if ch != '0' {
                        and |= 1;
                    }
                }
            }
            Op::Mem { address, value } => {
                *mem.entry(address).or_default() = value & and | or;
            }
        }
    }
    mem.values().sum()
}

fn gen_addresses(address: u64, mask: &str) -> Vec<u64> {
    let mut addresses = vec![address];
    for (i, ch) in mask.chars().rev().enumerate() {
        match ch {
            '0' => (),
            '1' => addresses.iter_mut().for_each(|x| {
                *x |= 1 << i;
            }),
            'X' => {
                for address in std::mem::take(&mut addresses) {
                    addresses.push(address | (1 << i));
                    addresses.push(address & !(1 << i));
                }
            }
            c => unreachable!("Got {:?} in mask", c),
        }
    }
    addresses
}

fn check_2(ops: &[Op]) -> u64 {
    let mut mem = HashMap::new();
    let mut mask = "";
    for op in ops {
        match op {
            Op::Mask(mmask) => mask = mmask,
            Op::Mem { address, value } => {
                let addresses = gen_addresses(*address, mask);
                for address in addresses {
                    *mem.entry(address).or_default() = *value;
                }
            }
        }
    }
    mem.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
            mem[8] = 11
            mem[7] = 101
            mem[8] = 0";
        let output = 165;
        let ops = parse_input(input);
        let res = check_1(&ops);
        assert_eq!(res, output);
    }

    #[test]
    fn test2() {
        let input = "mask = 000000000000000000000000000000X1001X
            mem[42] = 100
            mask = 00000000000000000000000000000000X0XX
            mem[26] = 1";
        let output = 208;
        let ops = parse_input(input);
        let res = check_2(&ops);
        assert_eq!(res, output);
    }
}
