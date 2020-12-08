fn main() {
    let input = std::fs::read_to_string("inputs/day8.txt").unwrap();
    let code = parse_input(&input);
    let res = check_1(&code).as_loop().unwrap();
    println!("Part 1 - Answer: {}", res);
    let res = check_2(&code);
    println!("Part 2 - Answer: {}", res);
}

fn parse_input(input: &str) -> Vec<(&str, i32)> {
    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut iter = line.split_whitespace();
            let op = iter.next().unwrap();
            let arg = iter.next().unwrap().parse().unwrap();
            (op, arg)
        })
        .collect()
}

#[derive(Debug, Eq, PartialEq)]
enum Exit {
    Loop(i32),
    Exit(i32),
}

impl Exit {
    fn as_loop(self) -> Option<i32> {
        match self {
            Exit::Loop(n) => Some(n),
            _ => None,
        }
    }
}

fn check_1(code: &[(&str, i32)]) -> Exit {
    let mut visited = vec![];
    let mut pc = 0;
    let mut acc = 0;
    loop {
        if pc as usize == code.len() {
            return Exit::Exit(acc);
        }
        if visited.contains(&pc) {
            return Exit::Loop(acc);
        }
        let mut nextpc = pc + 1;
        match code[pc as usize] {
            ("acc", arg) => acc += arg,
            ("jmp", arg) => nextpc = pc + arg,
            ("nop", _) => (),
            _ => unreachable!(),
        }
        visited.push(pc);
        pc = nextpc;
    }
}

fn check_2(code: &[(&str, i32)]) -> i32 {
    for i in 0..code.len() {
        let op = code[i].0;
        let new_op = match op {
            "jmp" => "nop",
            "nop" => "jmp",
            _ => continue,
        };
        let mut new_code = code.to_vec();
        new_code[i].0 = new_op;
        match check_1(&new_code) {
            Exit::Exit(n) => return n,
            _ => (),
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "nop +0
        acc +1
        jmp +4
        acc +3
        jmp -3
        acc -99
        acc +1
        jmp -4
        acc +6";

    #[test]
    fn test1() {
        let output = Exit::Loop(5);
        let res = check_1(&parse_input(INPUT));
        assert_eq!(res, output);
    }

    #[test]
    fn test2() {
        let output = 8;
        let res = check_2(&parse_input(INPUT));
        assert_eq!(res, output);
    }
}
